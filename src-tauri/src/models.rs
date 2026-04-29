use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Position) -> u32 {
        ((self.x as i32 - other.x as i32).abs() + (self.y as i32 - other.y as i32).abs()) as u32
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub sku: String,
    pub name: String,
    pub position: Position,
    pub quantity: u32,
    pub weight: f64,
}

impl Product {
    pub fn new(sku: &str, name: &str, position: Position, quantity: u32, weight: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            sku: sku.to_string(),
            name: name.to_string(),
            position,
            quantity,
            weight,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderPriority {
    Low,
    Medium,
    High,
    Urgent,
}

impl OrderPriority {
    pub fn to_u32(&self) -> u32 {
        match self {
            OrderPriority::Low => 1,
            OrderPriority::Medium => 2,
            OrderPriority::High => 3,
            OrderPriority::Urgent => 4,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    Pending,
    Scheduled,
    Picking,
    Picked,
    Exception,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub product_id: Uuid,
    pub product_sku: String,
    pub product_name: String,
    pub position: Position,
    pub requested_quantity: u32,
    pub picked_quantity: u32,
    pub weight: f64,
    pub is_exception: bool,
    pub exception_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub order_number: String,
    pub priority: OrderPriority,
    pub items: Vec<OrderItem>,
    pub status: OrderStatus,
    pub created_at: String,
    pub assigned_batch: Option<Uuid>,
    pub total_weight: f64,
    pub progress: f64,
}

impl Order {
    pub fn new(order_number: &str, priority: OrderPriority, items: Vec<OrderItem>) -> Self {
        let total_weight: f64 = items.iter().map(|item| item.weight * item.requested_quantity as f64).sum();
        Self {
            id: Uuid::new_v4(),
            order_number: order_number.to_string(),
            priority,
            items,
            status: OrderStatus::Pending,
            created_at: chrono::Local::now().to_rfc3339(),
            assigned_batch: None,
            total_weight,
            progress: 0.0,
        }
    }

    pub fn update_progress(&mut self) {
        let total: u32 = self.items.iter().map(|i| i.requested_quantity).sum();
        let picked: u32 = self.items.iter().map(|i| i.picked_quantity).sum();
        self.progress = if total > 0 { (picked as f64 / total as f64) * 100.0 } else { 0.0 };

        if self.progress >= 100.0 {
            self.status = OrderStatus::Picked;
        } else if self.progress > 0.0 {
            self.status = OrderStatus::Picking;
        }

        let has_exception = self.items.iter().any(|i| i.is_exception);
        if has_exception {
            self.status = OrderStatus::Exception;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Batch {
    pub id: Uuid,
    pub batch_number: String,
    pub order_ids: Vec<Uuid>,
    pub items: Vec<MergedItem>,
    pub path: Vec<Position>,
    pub total_distance: u32,
    pub total_weight: f64,
    pub status: BatchStatus,
    pub created_at: String,
    pub current_position: Option<Position>,
    pub progress: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BatchStatus {
    Pending,
    Ready,
    InProgress,
    Completed,
    Exception,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergedItem {
    pub product_id: Uuid,
    pub product_sku: String,
    pub product_name: String,
    pub position: Position,
    pub total_quantity: u32,
    pub picked_quantity: u32,
    pub total_weight: f64,
    pub from_orders: Vec<Uuid>,
    pub is_picked: bool,
    pub is_exception: bool,
    pub exception_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseConfig {
    pub grid_width: u32,
    pub grid_height: u32,
    pub start_position: Position,
    pub end_position: Position,
    pub max_batch_weight: f64,
    pub max_batch_items: u32,
}

impl Default for WarehouseConfig {
    fn default() -> Self {
        Self {
            grid_width: 20,
            grid_height: 15,
            start_position: Position::new(0, 7),
            end_position: Position::new(19, 7),
            max_batch_weight: 50.0,
            max_batch_items: 20,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rack {
    pub id: Uuid,
    pub position: Position,
    pub width: u32,
    pub height: u32,
    pub name: String,
}

impl Rack {
    pub fn new(x: u32, y: u32, width: u32, height: u32, name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            position: Position::new(x, y),
            width,
            height,
            name: name.to_string(),
        }
    }

    pub fn occupies(&self, pos: &Position) -> bool {
        pos.x >= self.position.x
            && pos.x < self.position.x + self.width
            && pos.y >= self.position.y
            && pos.y < self.position.y + self.height
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathNode {
    pub position: Position,
    pub product_sku: Option<String>,
    pub product_name: Option<String>,
    pub quantity: u32,
    pub is_picked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics {
    pub total_orders: u32,
    pub pending_orders: u32,
    pub in_progress_orders: u32,
    pub completed_orders: u32,
    pub exception_orders: u32,
    pub total_batches: u32,
    pub completed_batches: u32,
    pub total_distance: u32,
    pub total_weight_picked: f64,
    pub average_order_weight: f64,
    pub efficiency_score: f64,
}

impl Default for Statistics {
    fn default() -> Self {
        Self {
            total_orders: 0,
            pending_orders: 0,
            in_progress_orders: 0,
            completed_orders: 0,
            exception_orders: 0,
            total_batches: 0,
            completed_batches: 0,
            total_distance: 0,
            total_weight_picked: 0.0,
            average_order_weight: 0.0,
            efficiency_score: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExceptionType {
    OutOfStock,
    Damaged,
    WrongLocation,
    Missing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExceptionRecord {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub exception_type: ExceptionType,
    pub reason: String,
    pub reported_at: String,
    pub resolved: bool,
    pub resolved_at: Option<String>,
}
