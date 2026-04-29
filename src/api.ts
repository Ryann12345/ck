import { invoke } from '@tauri-apps/api/tauri';
import type { Order, Batch, Rack, WarehouseConfig, Statistics } from './types';

export const api = {
  async initializeTestData(): Promise<void> {
    return invoke('initialize_test_data');
  },

  async getAllOrders(): Promise<Order[]> {
    return invoke('get_all_orders');
  },

  async getAllBatches(): Promise<Batch[]> {
    return invoke('get_all_batches');
  },

  async createBatches(): Promise<Batch[]> {
    return invoke('create_batches');
  },

  async startPicking(batchId: string): Promise<void> {
    return invoke('start_picking', { batchId });
  },

  async updatePickingProgress(
    batchId: string,
    itemIdx: number,
    pickedQuantity: number,
  ): Promise<void> {
    return invoke('update_picking_progress', {
      batchId,
      itemIdx,
      pickedQuantity,
    });
  },

  async getWarehouseRacks(): Promise<Rack[]> {
    return invoke('get_warehouse_racks');
  },

  async getWarehouseConfig(): Promise<WarehouseConfig> {
    return invoke('get_warehouse_config');
  },

  async getStatistics(): Promise<Statistics> {
    return invoke('get_statistics');
  },

  async getPrioritizedOrders(): Promise<Order[]> {
    return invoke('get_prioritized_orders');
  },

  async markItemException(
    batchId: string,
    itemIdx: number,
    reason: string,
  ): Promise<void> {
    return invoke('mark_item_exception', {
      batchId,
      itemIdx,
      reason,
    });
  },
};
