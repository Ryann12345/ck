#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod lib;

use lib::models::*;
use lib::scheduler::*;
use std::sync::Mutex;
use tauri::State;

struct AppState {
    scheduler: Mutex<PickingScheduler>,
}

#[tauri::command]
fn initialize_test_data(state: State<AppState>) -> Result<(), String> {
    let mut scheduler = state.scheduler.lock().unwrap();
    create_test_data(&mut scheduler);
    Ok(())
}

#[tauri::command]
fn get_all_orders(state: State<AppState>) -> Vec<Order> {
    let scheduler = state.scheduler.lock().unwrap();
    scheduler.get_all_orders()
}

#[tauri::command]
fn get_all_batches(state: State<AppState>) -> Vec<Batch> {
    let scheduler = state.scheduler.lock().unwrap();
    scheduler.get_all_batches()
}

#[tauri::command]
fn create_batches(state: State<AppState>) -> Vec<Batch> {
    let mut scheduler = state.scheduler.lock().unwrap();
    scheduler.create_batches()
}

#[tauri::command]
fn start_picking(batch_id: String, state: State<AppState>) -> Result<(), String> {
    let batch_uuid = uuid::Uuid::parse_str(&batch_id).map_err(|e| e.to_string())?;
    let mut scheduler = state.scheduler.lock().unwrap();
    scheduler.start_picking(&batch_uuid)
}

#[tauri::command]
fn update_picking_progress(
    batch_id: String,
    item_idx: usize,
    picked_quantity: u32,
    state: State<AppState>,
) -> Result<(), String> {
    let batch_uuid = uuid::Uuid::parse_str(&batch_id).map_err(|e| e.to_string())?;
    let mut scheduler = state.scheduler.lock().unwrap();
    scheduler.update_picking_progress(&batch_uuid, item_idx, picked_quantity)
}

#[tauri::command]
fn get_warehouse_racks(state: State<AppState>) -> Vec<Rack> {
    let scheduler = state.scheduler.lock().unwrap();
    scheduler.get_warehouse_racks()
}

#[tauri::command]
fn get_warehouse_config(state: State<AppState>) -> WarehouseConfig {
    let scheduler = state.scheduler.lock().unwrap();
    scheduler.get_warehouse_config().clone()
}

#[tauri::command]
fn get_statistics(state: State<AppState>) -> Statistics {
    let scheduler = state.scheduler.lock().unwrap();
    scheduler.get_statistics()
}

#[tauri::command]
fn get_prioritized_orders(state: State<AppState>) -> Vec<Order> {
    let scheduler = state.scheduler.lock().unwrap();
    scheduler.get_prioritized_orders()
}

#[tauri::command]
fn mark_item_exception(
    batch_id: String,
    item_idx: usize,
    reason: String,
    state: State<AppState>,
) -> Result<(), String> {
    let batch_uuid = uuid::Uuid::parse_str(&batch_id).map_err(|e| e.to_string())?;
    let mut scheduler = state.scheduler.lock().unwrap();
    scheduler.mark_item_exception(
        &batch_uuid,
        item_idx,
        ExceptionType::OutOfStock,
        reason,
    )
}

fn main() {
    let config = WarehouseConfig::default();
    let scheduler = PickingScheduler::new(config);

    tauri::Builder::default()
        .manage(AppState {
            scheduler: Mutex::new(scheduler),
        })
        .invoke_handler(tauri::generate_handler![
            initialize_test_data,
            get_all_orders,
            get_all_batches,
            create_batches,
            start_picking,
            update_picking_progress,
            get_warehouse_racks,
            get_warehouse_config,
            get_statistics,
            get_prioritized_orders,
            mark_item_exception,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
