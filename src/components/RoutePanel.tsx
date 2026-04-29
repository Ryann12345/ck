import React from 'react';
import type { Batch } from '../types';

interface RoutePanelProps {
  selectedBatch: Batch | null;
}

const RoutePanel: React.FC<RoutePanelProps> = ({ selectedBatch }) => {
  if (!selectedBatch || selectedBatch.items.length === 0) {
    return (
      <div className="panel route-panel">
        <div className="panel-header">
          <h2>🛤️ 拣货路线</h2>
        </div>
        <div className="panel-body">
          <div className="empty-state">
            <div className="empty-state-icon">🛤️</div>
            <div className="empty-state-text">选择一个批次查看路线</div>
          </div>
        </div>
      </div>
    );
  }

  const { items, path, total_distance } = selectedBatch;

  const getStepStatus = (index: number): 'completed' | 'current' | 'pending' => {
    const completedCount = items.filter((item) => item.is_picked).length;
    if (index < completedCount) return 'completed';
    if (index === completedCount) return 'current';
    return 'pending';
  };

  return (
    <div className="panel route-panel">
      <div className="panel-header">
        <h2>🛤️ 拣货路线</h2>
        <span className="badge badge-priority-medium">
          距离: {total_distance} 格
        </span>
      </div>
      <div className="panel-body">
        <div className="route-list">
          <div className={`route-step ${getStepStatus(-1) === 'completed' ? 'completed' : ''}`}>
            <div className="route-step-number">起</div>
            <div className="route-step-info">
              <div className="route-step-location">起点</div>
              <div className="route-step-details">拣货开始位置</div>
            </div>
          </div>

          {items.map((item, index) => {
            const status = getStepStatus(index);
            return (
              <div 
                key={item.product_id} 
                className={`route-step ${status === 'current' ? 'current' : status === 'completed' ? 'completed' : ''}`}
              >
                <div className="route-step-number">
                  {status === 'completed' ? '✓' : index + 1}
                </div>
                <div className="route-step-info">
                  <div className="route-step-location">{item.product_name}</div>
                  <div className="route-step-details">
                    {item.product_sku} · 位置 ({item.position.x}, {item.position.y})
                  </div>
                </div>
                <div className="route-step-meta">
                  {item.picked_quantity}/{item.total_quantity} 件
                </div>
              </div>
            );
          })}

          <div className={`route-step ${items.every((i) => i.is_picked) ? 'completed' : ''}`}>
            <div className="route-step-number">终</div>
            <div className="route-step-info">
              <div className="route-step-location">终点</div>
              <div className="route-step-details">拣货完成位置</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default RoutePanel;
