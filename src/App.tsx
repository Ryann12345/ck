import React, { useState, useEffect, useCallback } from 'react';
import { api } from './api';
import type { Order, Batch, Rack, WarehouseConfig, Statistics, Position } from './types';
import OrdersPanel from './components/OrdersPanel';
import MapPanel from './components/MapPanel';
import StatsPanel from './components/StatsPanel';
import RoutePanel from './components/RoutePanel';
import BatchPanel from './components/BatchPanel';
import PickingPanel from './components/PickingPanel';

const App: React.FC = () => {
  const [orders, setOrders] = useState<Order[]>([]);
  const [batches, setBatches] = useState<Batch[]>([]);
  const [racks, setRacks] = useState<Rack[]>([]);
  const [warehouseConfig, setWarehouseConfig] = useState<WarehouseConfig | null>(null);
  const [statistics, setStatistics] = useState<Statistics | null>(null);
  const [selectedOrder, setSelectedOrder] = useState<Order | null>(null);
  const [selectedBatch, setSelectedBatch] = useState<Batch | null>(null);
  const [isInitialized, setIsInitialized] = useState(false);

  const refreshData = useCallback(async () => {
    try {
      const [ordersData, batchesData, statsData] = await Promise.all([
        api.getAllOrders(),
        api.getAllBatches(),
        api.getStatistics(),
      ]);
      setOrders(ordersData);
      setBatches(batchesData);
      setStatistics(statsData);

      if (selectedBatch) {
        const updatedBatch = batchesData.find((b) => b.id === selectedBatch.id);
        if (updatedBatch) {
          setSelectedBatch(updatedBatch);
        }
      }
    } catch (error) {
      console.error('刷新数据失败:', error);
    }
  }, [selectedBatch]);

  useEffect(() => {
    const initWarehouse = async () => {
      try {
        const [configData, racksData] = await Promise.all([
          api.getWarehouseConfig(),
          api.getWarehouseRacks(),
        ]);
        setWarehouseConfig(configData);
        setRacks(racksData);
      } catch (error) {
        console.error('初始化仓库配置失败:', error);
      }
    };
    initWarehouse();
  }, []);

  useEffect(() => {
    if (!isInitialized) return;

    const interval = setInterval(refreshData, 2000);
    return () => clearInterval(interval);
  }, [isInitialized, refreshData]);

  const handleInitializeTestData = async () => {
    try {
      await api.initializeTestData();
      setIsInitialized(true);
      await refreshData();
    } catch (error) {
      console.error('初始化测试数据失败:', error);
      alert('初始化失败，请检查控制台');
    }
  };

  const handleCreateBatches = async () => {
    try {
      const newBatches = await api.createBatches();
      setBatches(newBatches);
      await refreshData();
      if (newBatches.length > 0) {
        setSelectedBatch(newBatches[0]);
      }
    } catch (error) {
      console.error('创建批次失败:', error);
      alert('创建批次失败，请检查控制台');
    }
  };

  const handleStartPicking = async (batchId: string) => {
    try {
      await api.startPicking(batchId);
      await refreshData();
    } catch (error) {
      console.error('开始拣货失败:', error);
      alert('开始拣货失败，请检查控制台');
    }
  };

  const handleUpdatePickingProgress = async (itemIdx: number, quantity: number) => {
    if (!selectedBatch) return;
    
    try {
      await api.updatePickingProgress(selectedBatch.id, itemIdx, quantity);
      await refreshData();
    } catch (error) {
      console.error('更新拣货进度失败:', error);
      alert('更新进度失败: ' + (error as Error).message);
    }
  };

  const handleMarkException = async (itemIdx: number, reason: string) => {
    if (!selectedBatch) return;
    
    try {
      await api.markItemException(selectedBatch.id, itemIdx, reason);
      await refreshData();
    } catch (error) {
      console.error('标记异常失败:', error);
      alert('标记异常失败，请检查控制台');
    }
  };

  const getCurrentPosition = (): Position | null => {
    if (!selectedBatch) return warehouseConfig?.start_position || null;
    
    if (selectedBatch.status === 'Completed') {
      return warehouseConfig?.end_position || null;
    }

    const currentItem = selectedBatch.items.find((item) => !item.is_picked && !item.is_exception);
    if (currentItem) {
      return currentItem.position;
    }

    const lastPickedItem = selectedBatch.items
      .filter((item) => item.is_picked)
      .pop();
    if (lastPickedItem) {
      return lastPickedItem.position;
    }

    return warehouseConfig?.start_position || null;
  };

  const inProgressBatch = batches.find((b) => b.status === 'InProgress');

  return (
    <div className="app-container">
      <header className="header">
        <h1>仓库拣货路径规划模拟器</h1>
        <div className="header-controls">
          <button className="btn btn-primary" onClick={handleInitializeTestData}>
            🧪 初始化测试数据
          </button>
          <button 
            className="btn btn-warning" 
            onClick={handleCreateBatches}
            disabled={!isInitialized}
          >
            📦 创建批次
          </button>
          <button className="btn btn-success" onClick={refreshData}>
            🔄 刷新数据
          </button>
        </div>
      </header>

      <main className="main-content">
        <OrdersPanel
          orders={orders}
          selectedOrderId={selectedOrder?.id || null}
          onSelectOrder={setSelectedOrder}
        />

        <MapPanel
          racks={racks}
          config={warehouseConfig}
          selectedBatch={selectedBatch}
          currentPosition={getCurrentPosition()}
        />

        <StatsPanel statistics={statistics} />
        
        <RoutePanel selectedBatch={selectedBatch} />
        
        <BatchPanel
          batches={batches}
          selectedBatchId={selectedBatch?.id || null}
          onSelectBatch={setSelectedBatch}
          onStartPicking={handleStartPicking}
        />
      </main>

      <PickingPanel
        currentBatch={inProgressBatch || selectedBatch}
        onUpdateProgress={handleUpdatePickingProgress}
        onMarkException={handleMarkException}
      />
    </div>
  );
};

export default App;
