import React from 'react';
import type { Order } from '../types';
import { priorityColors, priorityLabels, orderStatusColors, orderStatusLabels } from '../types';

interface OrdersPanelProps {
  orders: Order[];
  selectedOrderId: string | null;
  onSelectOrder: (order: Order) => void;
}

const OrdersPanel: React.FC<OrdersPanelProps> = ({ orders, selectedOrderId, onSelectOrder }) => {
  if (orders.length === 0) {
    return (
      <div className="panel orders-panel">
        <div className="panel-header">
          <h2>📋 订单列表</h2>
          <span className="badge badge-priority-low">0</span>
        </div>
        <div className="panel-body">
          <div className="empty-state">
            <div className="empty-state-icon">📋</div>
            <div className="empty-state-text">暂无订单，请先初始化测试数据</div>
          </div>
        </div>
      </div>
    );
  }

  const sortedOrders = [...orders].sort((a, b) => {
    const priorityOrder = { Urgent: 4, High: 3, Medium: 2, Low: 1 };
    if (priorityOrder[a.priority] !== priorityOrder[b.priority]) {
      return priorityOrder[b.priority] - priorityOrder[a.priority];
    }
    return new Date(a.created_at).getTime() - new Date(b.created_at).getTime();
  });

  return (
    <div className="panel orders-panel">
      <div className="panel-header">
        <h2>📋 订单列表</h2>
        <span className="badge badge-priority-medium">{orders.length}</span>
      </div>
      <div className="panel-body">
        {sortedOrders.map((order) => (
          <div
            key={order.id}
            className={`order-card ${selectedOrderId === order.id ? 'selected' : ''}`}
            onClick={() => onSelectOrder(order)}
          >
            <div className="order-header">
              <span className="order-number">{order.order_number}</span>
              <div className="order-meta">
                <span 
                  className="badge badge-priority"
                  style={{ backgroundColor: priorityColors[order.priority] }}
                >
                  {priorityLabels[order.priority]}
                </span>
                <span 
                  className="badge badge-status"
                  style={{ backgroundColor: orderStatusColors[order.status] }}
                >
                  {orderStatusLabels[order.status]}
                </span>
              </div>
            </div>
            
            <div className="order-items-preview">
              {order.items.length} 件商品 · {order.total_weight.toFixed(1)} kg
            </div>
            
            <div className="order-progress">
              <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 4, fontSize: 11, color: '#a6adc8' }}>
                <span>进度</span>
                <span>{order.progress.toFixed(1)}%</span>
              </div>
              <div className="progress-bar">
                <div 
                  className="progress-fill" 
                  style={{ width: `${order.progress}%` }}
                />
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default OrdersPanel;
