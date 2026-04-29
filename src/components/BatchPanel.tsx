import React from 'react';
import type { Batch } from '../types';
import { batchStatusColors, batchStatusLabels } from '../types';

interface BatchPanelProps {
  batches: Batch[];
  selectedBatchId: string | null;
  onSelectBatch: (batch: Batch) => void;
  onStartPicking: (batchId: string) => void;
}

const BatchPanel: React.FC<BatchPanelProps> = ({ 
  batches, 
  selectedBatchId, 
  onSelectBatch,
  onStartPicking 
}) => {
  if (batches.length === 0) {
    return (
      <div className="panel batch-panel">
        <div className="panel-header">
          <h2>📦 批次列表</h2>
          <span className="badge badge-priority-low">0</span>
        </div>
        <div className="panel-body">
          <div className="empty-state">
            <div className="empty-state-icon">📦</div>
            <div className="empty-state-text">暂无批次，请先创建批次</div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="panel batch-panel">
      <div className="panel-header">
        <h2>📦 批次列表</h2>
        <span className="badge badge-priority-medium">{batches.length}</span>
      </div>
      <div className="panel-body">
        <div className="batch-list">
          {batches.map((batch) => (
            <div
              key={batch.id}
              className={`batch-card ${selectedBatchId === batch.id ? 'selected' : ''}`}
              onClick={() => onSelectBatch(batch)}
            >
              <div className="batch-header">
                <span className="batch-number">{batch.batch_number}</span>
                <span 
                  className="badge badge-status"
                  style={{ backgroundColor: batchStatusColors[batch.status] }}
                >
                  {batchStatusLabels[batch.status]}
                </span>
              </div>
              
              <div className="batch-meta">
                <span>📋 {batch.order_ids.length} 订单</span>
                <span>📦 {batch.items.length} 商品</span>
                <span>📏 {batch.total_distance} 格</span>
              </div>

              <div className="batch-item-count">
                总重量: {batch.total_weight.toFixed(1)} kg
              </div>

              <div className="order-progress">
                <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 4, fontSize: 11, color: '#a6adc8' }}>
                  <span>进度</span>
                  <span>{batch.progress.toFixed(1)}%</span>
                </div>
                <div className="progress-bar">
                  <div 
                    className="progress-fill" 
                    style={{ width: `${batch.progress}%` }}
                  />
                </div>
              </div>

              {batch.status === 'Ready' && (
                <div className="batch-actions">
                  <button 
                    className="btn btn-success"
                    onClick={(e) => {
                      e.stopPropagation();
                      onStartPicking(batch.id);
                    }}
                  >
                    ▶️ 开始拣货
                  </button>
                </div>
              )}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default BatchPanel;
