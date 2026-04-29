use crate::models::{
    Batch, BatchStatus, MergedItem, Order, OrderItem, OrderPriority, OrderStatus,
    Product, Statistics, WarehouseConfig, ExceptionRecord, ExceptionType,
};
use crate::pathfinding::PathFinder;
use crate::warehouse::Warehouse;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use chrono::Local;

pub struct PickingScheduler {
    warehouse: Warehouse,
    path_finder: PathFinder,
    products: HashMap<Uuid, Product>,
    orders: HashMap<Uuid, Order>,
    batches: HashMap<Uuid, Batch>,
    exceptions: HashMap<Uuid, ExceptionRecord>,
    config: WarehouseConfig,
}

impl PickingScheduler {
    pub fn new(config: WarehouseConfig) -> Self {
        let warehouse = Warehouse::new(config.clone());
        let path_finder = PathFinder::new(warehouse.clone());

        Self {
            warehouse,
            path_finder,
            products: HashMap::new(),
            orders: HashMap::new(),
            batches: HashMap::new(),
            exceptions: HashMap::new(),
            config,
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.insert(product.id, product);
    }

    pub fn get_product(&self, id: &Uuid) -> Option<&Product> {
        self.products.get(id)
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.insert(order.id, order);
    }

    pub fn get_order(&self, id: &Uuid) -> Option<&Order> {
        self.orders.get(id)
    }

    pub fn get_all_orders(&self) -> Vec<Order> {
        self.orders.values().cloned().collect()
    }

    pub fn get_all_batches(&self) -> Vec<Batch> {
        self.batches.values().cloned().collect()
    }

    pub fn get_warehouse_racks(&self) -> Vec<crate::models::Rack> {
        self.warehouse.get_racks().to_vec()
    }

    pub fn get_warehouse_config(&self) -> &WarehouseConfig {
        &self.config
    }

    // 订单优先级调度
    pub fn get_prioritized_orders(&self) -> Vec<Order> {
        let mut orders: Vec<Order> = self.orders
            .values()
            .filter(|o| o.status == OrderStatus::Pending)
            .cloned()
            .collect();

        orders.sort_by(|a, b| {
            let priority_cmp = b.priority.to_u32().cmp(&a.priority.to_u32());
            if priority_cmp != std::cmp::Ordering::Equal {
                priority_cmp
            } else {
                a.created_at.cmp(&b.created_at)
            }
        });

        orders
    }

    // 多订单合并 + 重复商品合并
    pub fn create_batches(&mut self) -> Vec<Batch> {
        let prioritized_orders = self.get_prioritized_orders();
        if prioritized_orders.is_empty() {
            return vec![];
        }

        let mut batches = Vec::new();
        let mut remaining_orders: Vec<Order> = prioritized_orders.clone();

        while !remaining_orders.is_empty() {
            // 按优先级分组
            let current_priority = remaining_orders[0].priority.clone();
            let same_priority: Vec<Order> = remaining_orders
                .iter()
                .filter(|o| o.priority == current_priority)
                .cloned()
                .collect();

            // 合并商品，处理重复商品
            let mut merged_items: HashMap<Uuid, MergedItem> = HashMap::new();
            let mut batch_order_ids = Vec::new();
            let mut current_weight = 0.0;
            let mut current_items_count = 0;

            for order in &same_priority {
                let order_weight: f64 = order.items.iter()
                    .map(|i| i.weight * i.requested_quantity as f64)
                    .sum();
                let order_items_count: u32 = order.items.iter()
                    .map(|i| i.requested_quantity)
                    .sum();

                // 检查容量限制
                if current_weight + order_weight > self.config.max_batch_weight
                    || current_items_count + order_items_count > self.config.max_batch_items
                {
                    // 创建当前批次
                    if !batch_order_ids.is_empty() {
                        let batch = self.create_batch_from_merged_items(
                            &merged_items,
                            &batch_order_ids,
                        );
                        batches.push(batch);
                    }

                    merged_items.clear();
                    batch_order_ids.clear();
                    current_weight = 0.0;
                    current_items_count = 0;
                }

                // 合并商品
                for item in &order.items {
                    let entry = merged_items.entry(item.product_id)
                        .or_insert_with(|| MergedItem {
                            product_id: item.product_id,
                            product_sku: item.product_sku.clone(),
                            product_name: item.product_name.clone(),
                            position: item.position.clone(),
                            total_quantity: 0,
                            picked_quantity: 0,
                            total_weight: 0.0,
                            from_orders: Vec::new(),
                            is_picked: false,
                            is_exception: false,
                            exception_reason: None,
                        });

                    entry.total_quantity += item.requested_quantity;
                    entry.total_weight += item.weight * item.requested_quantity as f64;
                    if !entry.from_orders.contains(&order.id) {
                        entry.from_orders.push(order.id);
                    }
                }

                batch_order_ids.push(order.id);
                current_weight += order_weight;
                current_items_count += order_items_count;
            }

            // 创建最后一个批次
            if !batch_order_ids.is_empty() {
                let batch = self.create_batch_from_merged_items(
                    &merged_items,
                    &batch_order_ids,
                );
                batches.push(batch);
            }

            // 移除已处理的订单
            let processed_ids: HashSet<Uuid> = batches
                .iter()
                .flat_map(|b| b.order_ids.iter().copied())
                .collect();

            remaining_orders.retain(|o| !processed_ids.contains(&o.id));
        }

        // 保存批次并更新订单状态
        for batch in &batches {
            self.batches.insert(batch.id, batch.clone());

            for order_id in &batch.order_ids {
                if let Some(order) = self.orders.get_mut(order_id) {
                    order.status = OrderStatus::Scheduled;
                    order.assigned_batch = Some(batch.id);
                }
            }
        }

        batches
    }

    fn create_batch_from_merged_items(
        &self,
        merged_items: &HashMap<Uuid, MergedItem>,
        order_ids: &[Uuid],
    ) -> Batch {
        let items: Vec<MergedItem> = merged_items.values().cloned().collect();
        let total_weight: f64 = items.iter().map(|i| i.total_weight).sum();

        // 优化路径
        let (path, item_order, total_distance) = self.path_finder.optimize_picking_path(
            &items,
            &self.config.start_position,
            &self.config.end_position,
        );

        // 重新排序商品以匹配路径顺序
        let ordered_items: Vec<MergedItem> = item_order
            .iter()
            .map(|&idx| items[idx].clone())
            .collect();

        Batch {
            id: Uuid::new_v4(),
            batch_number: format!("BATCH-{:06}", self.batches.len() + 1),
            order_ids: order_ids.to_vec(),
            items: ordered_items,
            path,
            total_distance,
            total_weight,
            status: BatchStatus::Ready,
            created_at: Local::now().to_rfc3339(),
            current_position: Some(self.config.start_position.clone()),
            progress: 0.0,
        }
    }

    // 开始拣货
    pub fn start_picking(&mut self, batch_id: &Uuid) -> Result<(), String> {
        let batch = self.batches.get_mut(batch_id)
            .ok_or("批次不存在")?;

        if batch.status != BatchStatus::Ready {
            return Err("批次状态不正确，无法开始拣货".to_string());
        }

        batch.status = BatchStatus::InProgress;

        // 更新关联订单状态
        for order_id in &batch.order_ids {
            if let Some(order) = self.orders.get_mut(order_id) {
                order.status = OrderStatus::Picking;
            }
        }

        Ok(())
    }

    // 更新拣货进度
    pub fn update_picking_progress(
        &mut self,
        batch_id: &Uuid,
        item_idx: usize,
        picked_quantity: u32,
    ) -> Result<(), String> {
        let batch = self.batches.get_mut(batch_id)
            .ok_or("批次不存在")?;

        if item_idx >= batch.items.len() {
            return Err("商品索引超出范围".to_string());
        }

        let item = &mut batch.items[item_idx];
        let requested = item.total_quantity;

        if picked_quantity > requested {
            return Err(format!("拣货数量 ({}) 不能超过需求数量 ({})", picked_quantity, requested));
        }

        // 检查库存
        if let Some(product) = self.products.get(&item.product_id) {
            if picked_quantity > product.quantity {
                return self.mark_item_exception(
                    batch_id,
                    item_idx,
                    ExceptionType::OutOfStock,
                    format!("库存不足，需求 {}，实际库存 {}", picked_quantity, product.quantity),
                );
            }
        }

        item.picked_quantity = picked_quantity;
        item.is_picked = picked_quantity == requested;

        // 更新批次进度
        self.update_batch_progress(batch_id);

        // 更新关联订单
        self.update_orders_from_batch(batch_id);

        Ok(())
    }

    // 标记商品异常
    pub fn mark_item_exception(
        &mut self,
        batch_id: &Uuid,
        item_idx: usize,
        exception_type: ExceptionType,
        reason: String,
    ) -> Result<(), String> {
        let batch = self.batches.get_mut(batch_id)
            .ok_or("批次不存在")?;

        if item_idx >= batch.items.len() {
            return Err("商品索引超出范围".to_string());
        }

        let item = &mut batch.items[item_idx];
        item.is_exception = true;
        item.exception_reason = Some(reason.clone());

        // 创建异常记录
        let exception = ExceptionRecord {
            id: Uuid::new_v4(),
            order_id: Uuid::default(), // 将在下面填充
            product_id: item.product_id,
            exception_type,
            reason,
            reported_at: Local::now().to_rfc3339(),
            resolved: false,
            resolved_at: None,
        };

        // 为每个关联的订单创建异常记录
        for order_id in &item.from_orders {
            let mut order_exception = exception.clone();
            order_exception.id = Uuid::new_v4();
            order_exception.order_id = *order_id;
            self.exceptions.insert(order_exception.id, order_exception);
        }

        // 更新批次状态
        batch.status = BatchStatus::Exception;

        // 更新订单
        for order_id in &batch.order_ids {
            if let Some(order) = self.orders.get_mut(order_id) {
                order.status = OrderStatus::Exception;
            }
        }

        Ok(())
    }

    fn update_batch_progress(&mut self, batch_id: &Uuid) {
        if let Some(batch) = self.batches.get_mut(batch_id) {
            let total: u32 = batch.items.iter().map(|i| i.total_quantity).sum();
            let picked: u32 = batch.items.iter().map(|i| i.picked_quantity).sum();
            
            batch.progress = if total > 0 { (picked as f64 / total as f64) * 100.0 } else { 0.0 };

            // 更新当前位置（基于已拣完的商品）
            let picked_items: Vec<_> = batch.items
                .iter()
                .filter(|i| i.is_picked)
                .collect();

            if !picked_items.is_empty() {
                let last_picked = picked_items.last().unwrap();
                batch.current_position = Some(last_picked.position.clone());
            }

            // 检查是否完成
            if batch.progress >= 100.0 {
                batch.status = BatchStatus::Completed;
                batch.current_position = Some(self.config.end_position.clone());
            }
        }
    }

    fn update_orders_from_batch(&mut self, batch_id: &Uuid) {
        if let Some(batch) = self.batches.get(batch_id) {
            for order_id in &batch.order_ids {
                if let Some(order) = self.orders.get_mut(order_id) {
                    // 更新每个订单项的拣货数量
                    for order_item in &mut order.items {
                        if let Some(batch_item) = batch.items
                            .iter()
                            .find(|bi| bi.product_id == order_item.product_id)
                        {
                            // 计算该订单应分配的拣货数量
                            let total_requested = batch_item.total_quantity;
                            let total_picked = batch_item.picked_quantity;
                            let order_requested = order_item.requested_quantity;

                            // 按比例分配
                            if total_requested > 0 {
                                let allocated = ((order_requested as f64 / total_requested as f64) 
                                    * total_picked as f64) as u32;
                                order_item.picked_quantity = allocated.min(order_requested);
                            }
                        }
                    }

                    order.update_progress();
                }
            }
        }
    }

    // 获取统计信息
    pub fn get_statistics(&self) -> Statistics {
        let total_orders = self.orders.len() as u32;
        let pending_orders = self.orders.values().filter(|o| o.status == OrderStatus::Pending).count() as u32;
        let in_progress_orders = self.orders.values().filter(|o| o.status == OrderStatus::Picking || o.status == OrderStatus::Scheduled).count() as u32;
        let completed_orders = self.orders.values().filter(|o| o.status == OrderStatus::Completed || o.status == OrderStatus::Picked).count() as u32;
        let exception_orders = self.orders.values().filter(|o| o.status == OrderStatus::Exception).count() as u32;

        let total_batches = self.batches.len() as u32;
        let completed_batches = self.batches.values().filter(|b| b.status == BatchStatus::Completed).count() as u32;
        let total_distance: u32 = self.batches.values().filter(|b| b.status == BatchStatus::Completed).map(|b| b.total_distance).sum();
        
        let total_weight_picked: f64 = self.orders.values()
            .filter(|o| o.status == OrderStatus::Completed || o.status == OrderStatus::Picked)
            .map(|o| o.total_weight)
            .sum();

        let average_order_weight = if total_orders > 0 {
            self.orders.values().map(|o| o.total_weight).sum::<f64>() / total_orders as f64
        } else {
            0.0
        };

        // 效率评分：基于完成率和路径优化
        let completion_rate = if total_batches > 0 {
            completed_batches as f64 / total_batches as f64
        } else {
            0.0
        };

        let efficiency_score = completion_rate * 100.0;

        Statistics {
            total_orders,
            pending_orders,
            in_progress_orders,
            completed_orders,
            exception_orders,
            total_batches,
            completed_batches,
            total_distance,
            total_weight_picked,
            average_order_weight,
            efficiency_score,
        }
    }

    // 解析异常
    pub fn resolve_exception(&mut self, exception_id: &Uuid) -> Result<(), String> {
        let exception = self.exceptions.get_mut(exception_id)
            .ok_or("异常记录不存在")?;

        exception.resolved = true;
        exception.resolved_at = Some(Local::now().to_rfc3339());

        Ok(())
    }

    pub fn get_path_finder(&self) -> &PathFinder {
        &self.path_finder
    }
}

// 创建测试数据
pub fn create_test_data(scheduler: &mut PickingScheduler) {
    // 创建测试商品 - 使用通道旁边的位置（确保在可通行）
    // 位置坐标要放在通道附近，避免在货架之间的通道位置
    let test_products = vec![
        Product::new("SKU-001", "可口可乐 330ml", crate::models::Position::new(0, 2), 100, 0.35),
        Product::new("SKU-002", "百事可乐 330ml", crate::models::Position::new(0, 3), 80, 0.35),
        Product::new("SKU-003", "怡宝矿泉水 550ml", crate::models::Position::new(3, 2), 200, 0.6),
        Product::new("SKU-004", "农夫山泉 550ml", crate::models::Position::new(3, 3), 150, 0.6),
        Product::new("SKU-005", "康师傅红烧牛肉面", crate::models::Position::new(5, 2), 100, 0.85),
        Product::new("SKU-006", "康师傅香辣牛肉面", crate::models::Position::new(5, 3), 90, 0.85),
        Product::new("SKU-007", "统一老坛酸菜面", crate::models::Position::new(8, 2), 80, 0.85),
        Product::new("SKU-008", "奥利奥饼干 97g", crate::models::Position::new(8, 3), 120, 0.3),
        Product::new("SKU-009", "乐事薯片 75g", crate::models::Position::new(10, 2), 100, 0.2),
        Product::new("SKU-010", "品客薯片 110g", crate::models::Position::new(10, 3), 60, 0.4),
        Product::new("SKU-011", "德芙巧克力 43g", crate::models::Position::new(0, 6), 150, 0.1),
        Product::new("SKU-012", "费列罗巧克力", crate::models::Position::new(0, 8), 80, 0.3),
        Product::new("SKU-013", "旺仔牛奶 245ml", crate::models::Position::new(3, 6), 100, 0.4),
        Product::new("SKU-014", "蒙牛纯牛奶 250ml", crate::models::Position::new(3, 8), 200, 0.3),
        Product::new("SKU-015", "伊利纯牛奶 250ml", crate::models::Position::new(5, 6), 180, 0.3),
        Product::new("SKU-016", "光明酸奶 200g", crate::models::Position::new(5, 8), 120, 0.25),
        Product::new("SKU-017", "达利园面包", crate::models::Position::new(8, 6), 90, 0.4),
        Product::new("SKU-018", "盼盼法式小面包", crate::models::Position::new(8, 8), 85, 0.35),
        Product::new("SKU-019", "洁柔抽纸 100抽", crate::models::Position::new(10, 6), 200, 0.5),
        Product::new("SKU-020", "维达抽纸 150抽", crate::models::Position::new(10, 8), 150, 0.6),
    ];

    // 先保存产品用于创建订单，然后再添加到scheduler
    let products_clone: Vec<Product> = test_products.clone();

    for product in test_products {
        scheduler.add_product(product);
    }

    let test_orders = vec![
        create_test_order(
            "ORD-2024001",
            OrderPriority::Urgent,
            &products_clone[0..4],
            vec![10, 5, 15, 8],
        ),
        create_test_order(
            "ORD-2024002",
            OrderPriority::High,
            &products_clone[4..8],
            vec![6, 4, 3, 12],
        ),
        create_test_order(
            "ORD-2024003",
            OrderPriority::Medium,
            &products_clone[8..12],
            vec![8, 6, 4, 2],
        ),
        create_test_order(
            "ORD-2024004",
            OrderPriority::Low,
            &products_clone[12..16],
            vec![5, 10, 8, 6],
        ),
        create_test_order(
            "ORD-2024005",
            OrderPriority::High,
            &[products_clone[0].clone(), products_clone[2].clone()],
            vec![5, 10],
        ),
        create_test_order(
            "ORD-2024006",
            OrderPriority::High,
            &[products_clone[4].clone(), products_clone[5].clone()],
            vec![4, 2],
        ),
        create_test_order(
            "ORD-2024007",
            OrderPriority::Medium,
            &products_clone[16..20],
            vec![20, 15, 25, 18],
        ),
    ];

    for order in test_orders {
        scheduler.add_order(order);
    }
}

fn create_test_order(
    order_number: &str,
    priority: OrderPriority,
    products: &[Product],
    quantities: Vec<u32>,
) -> Order {
    let items: Vec<OrderItem> = products
        .iter()
        .zip(quantities.iter())
        .map(|(product, &qty)| OrderItem {
            product_id: product.id,
            product_sku: product.sku.clone(),
            product_name: product.name.clone(),
            position: product.position.clone(),
            requested_quantity: qty,
            picked_quantity: 0,
            weight: product.weight,
            is_exception: false,
            exception_reason: None,
        })
        .collect();

    Order::new(order_number, priority, items)
}
