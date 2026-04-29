export interface Position {
  x: number;
  y: number;
}

export interface Rack {
  id: string;
  position: Position;
  width: number;
  height: number;
  name: string;
}

export interface WarehouseConfig {
  grid_width: number;
  grid_height: number;
  start_position: Position;
  end_position: Position;
  max_batch_weight: number;
  max_batch_items: number;
}

export type OrderPriority = 'Low' | 'Medium' | 'High' | 'Urgent';
export type OrderStatus = 'Pending' | 'Scheduled' | 'Picking' | 'Picked' | 'Exception' | 'Completed';

export interface OrderItem {
  product_id: string;
  product_sku: string;
  product_name: string;
  position: Position;
  requested_quantity: number;
  picked_quantity: number;
  weight: number;
  is_exception: boolean;
  exception_reason: string | null;
}

export interface Order {
  id: string;
  order_number: string;
  priority: OrderPriority;
  items: OrderItem[];
  status: OrderStatus;
  created_at: string;
  assigned_batch: string | null;
  total_weight: number;
  progress: number;
}

export type BatchStatus = 'Pending' | 'Ready' | 'InProgress' | 'Completed' | 'Exception';

export interface MergedItem {
  product_id: string;
  product_sku: string;
  product_name: string;
  position: Position;
  total_quantity: number;
  picked_quantity: number;
  total_weight: number;
  from_orders: string[];
  is_picked: boolean;
  is_exception: boolean;
  exception_reason: string | null;
}

export interface Batch {
  id: string;
  batch_number: string;
  order_ids: string[];
  items: MergedItem[];
  path: Position[];
  total_distance: number;
  total_weight: number;
  status: BatchStatus;
  created_at: string;
  current_position: Position | null;
  progress: number;
}

export interface Statistics {
  total_orders: number;
  pending_orders: number;
  in_progress_orders: number;
  completed_orders: number;
  exception_orders: number;
  total_batches: number;
  completed_batches: number;
  total_distance: number;
  total_weight_picked: number;
  average_order_weight: number;
  efficiency_score: number;
}

export const priorityColors: Record<OrderPriority, string> = {
  Low: '#6c757d',
  Medium: '#17a2b8',
  High: '#ffc107',
  Urgent: '#dc3545',
};

export const priorityLabels: Record<OrderPriority, string> = {
  Low: '低',
  Medium: '中',
  High: '高',
  Urgent: '紧急',
};

export const orderStatusColors: Record<OrderStatus, string> = {
  Pending: '#6c757d',
  Scheduled: '#007bff',
  Picking: '#ffc107',
  Picked: '#28a745',
  Exception: '#dc3545',
  Completed: '#17a2b8',
};

export const orderStatusLabels: Record<OrderStatus, string> = {
  Pending: '待处理',
  Scheduled: '已调度',
  Picking: '拣货中',
  Picked: '已拣货',
  Exception: '异常',
  Completed: '已完成',
};

export const batchStatusColors: Record<BatchStatus, string> = {
  Pending: '#6c757d',
  Ready: '#007bff',
  InProgress: '#ffc107',
  Completed: '#28a745',
  Exception: '#dc3545',
};

export const batchStatusLabels: Record<BatchStatus, string> = {
  Pending: '待处理',
  Ready: '就绪',
  InProgress: '进行中',
  Completed: '已完成',
  Exception: '异常',
};
