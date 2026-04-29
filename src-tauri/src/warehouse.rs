use crate::models::{Position, Rack, WarehouseConfig};
use std::collections::HashMap;

pub struct Warehouse {
    config: WarehouseConfig,
    racks: Vec<Rack>,
    obstacles: HashMap<(u32, u32), bool>,
}

impl Warehouse {
    pub fn new(config: WarehouseConfig) -> Self {
        let racks = Self::create_default_racks(config.grid_width, config.grid_height);
        let mut obstacles = HashMap::new();

        for rack in &racks {
            for x in rack.position.x..rack.position.x + rack.width {
                for y in rack.position.y..rack.position.y + rack.height {
                    obstacles.insert((x, y), true);
                }
            }
        }

        Self {
            config,
            racks,
            obstacles,
        }
    }

    fn create_default_racks(width: u32, height: u32) -> Vec<Rack> {
        let mut racks = Vec::new();
        
        // 创建货架布局：多排货架，中间留通道
        // 通道在 x = 4, 8, 12, 16 等位置
        // 通道在 y = 3, 7, 11 等位置
        
        for row in 0..3 {
            for col in 0..4 {
                let x = col * 5;
                let y = if row == 0 { 1 } else if row == 1 { 6 } else { 11 };
                
                racks.push(Rack::new(x, y, 2, 2, &format!("R-{}-{}", row, col)));
                racks.push(Rack::new(x + 3, y, 2, 2, &format!("R-{}-{}", row, col + 4)));
            }
        }
        
        racks
    }

    pub fn is_passable(&self, pos: &Position) -> bool {
        if pos.x >= self.config.grid_width || pos.y >= self.config.grid_height {
            return false;
        }
        !self.obstacles.contains_key(&(pos.x, pos.y))
    }

    pub fn get_neighbors(&self, pos: &Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for (dx, dy) in directions.iter() {
            let nx = pos.x as i32 + dx;
            let ny = pos.y as i32 + dy;

            if nx >= 0 && ny >= 0 {
                let new_pos = Position::new(nx as u32, ny as u32);
                if self.is_passable(&new_pos) {
                    neighbors.push(new_pos);
                }
            }
        }

        neighbors
    }

    pub fn get_racks(&self) -> &[Rack] {
        &self.racks
    }

    pub fn get_config(&self) -> &WarehouseConfig {
        &self.config
    }

    pub fn find_valid_position(&self, preferred: &Position) -> Position {
        if self.is_passable(preferred) {
            return preferred.clone();
        }

        for radius in 1..10 {
            for dx in -(radius as i32)..=radius as i32 {
                for dy in -(radius as i32)..=radius as i32 {
                    if dx.abs() + dy.abs() == radius as i32 {
                        let nx = preferred.x as i32 + dx;
                        let ny = preferred.y as i32 + dy;
                        if nx >= 0 && ny >= 0 {
                            let pos = Position::new(nx as u32, ny as u32);
                            if self.is_passable(&pos) {
                                return pos;
                            }
                        }
                    }
                }
            }
        }

        self.config.start_position.clone()
    }
}
