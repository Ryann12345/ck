import React, { useState } from 'react';
import type { Batch, MergedItem } from '../types';

interface PickingPanelProps {
  currentBatch: Batch | null;
  onUpdateProgress: (itemIdx: number, quantity: number) => void;
  onMarkException: (itemIdx: number, reason: string) => void;
}

const PickingPanel: React.FC<PickingPanelProps> = ({ 
  currentBatch, 
  onUpdateProgress,
  onMarkException 
}) => {
  const [quantity, setQuantity] = useState<number>(0);
  const [showExceptionModal, setShowExceptionModal] = useState(false);
  const [exceptionReason, setExceptionReason] = useState('');

  if (!currentBatch || currentBatch.status !== 'InProgress') {
    return null;
  }

  const currentItemIdx = currentBatch.items.findIndex((item) => !item.is_picked && !item.is_exception);
  
  if (currentItemIdx === -1) {
    return null;
  }

  const currentItem = currentBatch.items[currentItemIdx];

  const handleConfirm = () => {
    if (quantity > 0 && quantity <= currentItem.total_quantity) {
      onUpdateProgress(currentItemIdx, quantity);
      setQuantity(0);
    }
  };

  const handleFullPick = () => {
    onUpdateProgress(currentItemIdx, currentItem.total_quantity);
    setQuantity(0);
  };

  const handleException = () => {
    setShowExceptionModal(true);
  };

  const handleConfirmException = () => {
    if (exceptionReason.trim()) {
      onMarkException(currentItemIdx, exceptionReason);
      setExceptionReason('');
      setShowExceptionModal(false);
    }
  };

  return (
    <>
      <div className="picking-panel">
        <div className="picking-panel-content">
          <div className="picking-item-info">
            <div className="picking-item-name">
              🔹 当前拣货: {currentItem.product_name}
            </div>
            <div className="picking-item-meta">
              <span>SKU: {currentItem.product_sku}</span>
              <span>位置: ({currentItem.position.x}, {currentItem.position.y})</span>
              <span>需求数量: {currentItem.total_quantity}</span>
              <span>来自 {currentItem.from_orders.length} 个订单</span>
            </div>
          </div>

          <div className="picking-quantity-control">
            <span style={{ color: '#a6adc8', fontSize: 13 }}>拣货数量:</span>
            <input
              type="number"
              className="quantity-input"
              value={quantity || ''}
              onChange={(e) => setQuantity(parseInt(e.target.value) || 0)}
              min={1}
              max={currentItem.total_quantity}
              placeholder={currentItem.total_quantity.toString()}
            />
          </div>

          <div className="picking-actions">
            <button className="btn btn-success" onClick={handleFullPick}>
              ✓ 全部拣完 ({currentItem.total_quantity})
            </button>
            <button 
              className="btn btn-primary" 
              onClick={handleConfirm}
              disabled={quantity <= 0 || quantity > currentItem.total_quantity}
            >
              确认拣货
            </button>
            <button className="btn btn-danger" onClick={handleException}>
              ⚠️ 标记异常
            </button>
          </div>
        </div>
      </div>

      {showExceptionModal && (
        <div className="modal-overlay" onClick={() => setShowExceptionModal(false)}>
          <div className="modal-content" onClick={(e) => e.stopPropagation()}>
            <div className="modal-header">
              ⚠️ 标记异常
            </div>
            <div className="modal-body">
              <div className="form-group">
                <label className="form-label">商品信息</label>
                <div style={{ 
                  padding: 12, 
                  background: '#11111b', 
                  borderRadius: 8, 
                  fontSize: 13,
                  color: '#cdd6f4'
                }}>
                  <div><strong>{currentItem.product_name}</strong></div>
                  <div style={{ marginTop: 4, color: '#a6adc8' }}>
                    SKU: {currentItem.product_sku} · 需求: {currentItem.total_quantity} 件
                  </div>
                </div>
              </div>
              <div className="form-group">
                <label className="form-label">异常原因</label>
                <textarea
                  className="form-textarea"
                  value={exceptionReason}
                  onChange={(e) => setExceptionReason(e.target.value)}
                  placeholder="请描述异常原因，如：库存不足、商品损坏、位置错误等..."
                />
              </div>
            </div>
            <div className="modal-footer">
              <button 
                className="btn btn-primary" 
                onClick={() => setShowExceptionModal(false)}
              >
                取消
              </button>
              <button 
                className="btn btn-danger" 
                onClick={handleConfirmException}
                disabled={!exceptionReason.trim()}
              >
                确认标记异常
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
};

export default PickingPanel;
