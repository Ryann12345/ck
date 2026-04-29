import React from 'react';
import type { Statistics } from '../types';

interface StatsPanelProps {
  statistics: Statistics | null;
}

const StatsPanel: React.FC<StatsPanelProps> = ({ statistics }) => {
  if (!statistics) {
    return (
      <div className="panel stats-panel">
        <div className="panel-header">
          <h2>📊 统计面板</h2>
        </div>
        <div className="panel-body">
          <div className="empty-state">
            <div className="empty-state-icon">📊</div>
            <div className="empty-state-text">加载统计数据中...</div>
          </div>
        </div>
      </div>
    );
  }

  const stats = [
    { label: '总订单', value: statistics.total_orders, colorClass: 'stat-value-primary', icon: '📋' },
    { label: '待处理', value: statistics.pending_orders, colorClass: 'stat-value-warning', icon: '⏳' },
    { label: '进行中', value: statistics.in_progress_orders, colorClass: 'stat-value-info', icon: '🔄' },
    { label: '已完成', value: statistics.completed_orders, colorClass: 'stat-value-success', icon: '✅' },
    { label: '异常', value: statistics.exception_orders, colorClass: 'stat-value-danger', icon: '⚠️' },
    { label: '总批次', value: statistics.total_batches, colorClass: 'stat-value-primary', icon: '📦' },
  ];

  return (
    <div className="panel stats-panel">
      <div className="panel-header">
        <h2>📊 统计面板</h2>
      </div>
      <div className="panel-body">
        <div className="stats-grid">
          {stats.map((stat, index) => (
            <div key={index} className="stat-card">
              <div className={`stat-value ${stat.colorClass}`}>{stat.value}</div>
              <div className="stat-label">{stat.label}</div>
            </div>
          ))}
        </div>
        
        <div className="efficiency-meter">
          <div className="efficiency-header">
            <span className="efficiency-label">效率评分</span>
            <span className="efficiency-score">{statistics.efficiency_score.toFixed(1)}%</span>
          </div>
          <div className="progress-bar">
            <div 
              className="progress-fill" 
              style={{ width: `${statistics.efficiency_score}%` }}
            />
          </div>
        </div>

        <div className="efficiency-meter">
          <div className="efficiency-header">
            <span className="efficiency-label">总行驶距离</span>
            <span className="efficiency-score" style={{ color: '#89b4fa' }}>
              {statistics.total_distance} 格
            </span>
          </div>
        </div>

        <div className="efficiency-meter">
          <div className="efficiency-header">
            <span className="efficiency-label">平均订单重量</span>
            <span className="efficiency-score" style={{ color: '#cba6f7' }}>
              {statistics.average_order_weight.toFixed(2)} kg
            </span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default StatsPanel;
