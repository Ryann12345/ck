import React from 'react';
import type { Position, Rack, WarehouseConfig, Batch, MergedItem } from '../types';

interface MapPanelProps {
  racks: Rack[];
  config: WarehouseConfig | null;
  selectedBatch: Batch | null;
  currentPosition: Position | null;
}

const MapPanel: React.FC<MapPanelProps> = ({ racks, config, selectedBatch, currentPosition }) => {
  if (!config) {
    return (
      <div className="panel map-panel">
        <div className="panel-header">
          <h2>🗺️ 仓库地图</h2>
        </div>
        <div className="panel-body">
          <div className="empty-state">
            <div className="empty-state-icon">🗺️</div>
            <div className="empty-state-text">加载仓库配置中...</div>
          </div>
        </div>
      </div>
    );
  }

  const { grid_width, grid_height, start_position, end_position } = config;

  const rackMap = new Map<string, Rack>();
  racks.forEach((rack) => {
    for (let x = rack.position.x; x < rack.position.x + rack.width; x++) {
      for (let y = rack.position.y; y < rack.position.y + rack.height; y++) {
        rackMap.set(`${x},${y}`, rack);
      }
    }
  });

  const pathSet = new Set<string>();
  if (selectedBatch && selectedBatch.path.length > 0) {
    selectedBatch.path.forEach((pos) => {
      pathSet.add(`${pos.x},${pos.y}`);
    });
  }

  const productMap = new Map<string, MergedItem>();
  if (selectedBatch) {
    selectedBatch.items.forEach((item) => {
      productMap.set(`${item.position.x},${item.position.y}`, item);
    });
  }

  const currentPosKey = currentPosition ? `${currentPosition.x},${currentPosition.y}` : null;

  const getCellClass = (x: number, y: number): string => {
    const key = `${x},${y}`;

    if (currentPosKey === key && currentPosition) {
      return 'cell-current';
    }

    if (x === start_position.x && y === start_position.y) {
      return 'cell-start';
    }
    if (x === end_position.x && y === end_position.y) {
      return 'cell-end';
    }

    if (productMap.has(key)) {
      const item = productMap.get(key)!;
      return item.is_picked ? 'cell-product-picked' : 'cell-product';
    }

    if (pathSet.has(key) && !rackMap.has(key)) {
      return 'cell-path';
    }

    if (rackMap.has(key)) {
      return 'cell-rack';
    }

    return 'cell-empty';
  };

  const getCellContent = (x: number, y: number): string => {
    if (x === start_position.x && y === start_position.y) {
      return '起';
    }
    if (x === end_position.x && y === end_position.y) {
      return '终';
    }

    const key = `${x},${y}`;
    if (productMap.has(key)) {
      const item = productMap.get(key)!;
      return item.is_picked ? '✓' : `${item.total_quantity}`;
    }

    if (rackMap.has(key)) {
      const rack = rackMap.get(key)!;
      if (x === rack.position.x && y === rack.position.y) {
        return rack.name.split('-').slice(1).join('-');
      }
    }

    return '';
  };

  const renderGrid = () => {
    const rows = [];
    for (let y = 0; y < grid_height; y++) {
      const cells = [];
      for (let x = 0; x < grid_width; x++) {
        const cellClass = getCellClass(x, y);
        const cellContent = getCellContent(x, y);
        cells.push(
          <div key={`${x}-${y}`} className={`map-cell ${cellClass}`}>
            {cellContent}
          </div>
        );
      }
      rows.push(
        <div key={y} className="map-row">
          {cells}
        </div>
      );
    }
    return rows;
  };

  return (
    <div className="panel map-panel">
      <div className="panel-header">
        <h2>🗺️ 仓库地图</h2>
        <span style={{ fontSize: 12, color: '#a6adc8' }}>
          {selectedBatch ? `批次: ${selectedBatch.batch_number}` : '未选择批次'}
        </span>
      </div>
      <div className="map-container">
        <div className="map-wrapper">
          <div className="map-grid">
            {renderGrid()}
          </div>
        </div>
        <div className="map-legend">
          <div className="legend-item">
            <div className="legend-color" style={{ background: 'linear-gradient(135deg, #a6e3a1 0%, #94e2d5 100%)' }} />
            <span>起点</span>
          </div>
          <div className="legend-item">
            <div className="legend-color" style={{ background: 'linear-gradient(135deg, #f9e2af 0%, #fab387 100%)' }} />
            <span>终点</span>
          </div>
          <div className="legend-item">
            <div className="legend-color" style={{ background: '#6c7086' }} />
            <span>货架</span>
          </div>
          <div className="legend-item">
            <div className="legend-color" style={{ background: 'linear-gradient(135deg, #b4befe 0%, #cba6f7 100%)' }} />
            <span>待拣货</span>
          </div>
          <div className="legend-item">
            <div className="legend-color" style={{ background: 'linear-gradient(135deg, #a6e3a1 0%, #94e2d5 100%)' }} />
            <span>已拣货</span>
          </div>
          <div className="legend-item">
            <div className="legend-color" style={{ background: 'linear-gradient(135deg, #89b4fa 0%, #74c7ec 100%)' }} />
            <span>规划路径</span>
          </div>
          <div className="legend-item">
            <div className="legend-color" style={{ background: 'linear-gradient(135deg, #f38ba8 0%, #eba0ac 100%)' }} />
            <span>当前位置</span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default MapPanel;
