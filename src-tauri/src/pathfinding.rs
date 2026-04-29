use crate::models::{MergedItem, Position};
use crate::warehouse::Warehouse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;

pub struct PathFinder {
    warehouse: Warehouse,
}

impl PathFinder {
    pub fn new(warehouse: Warehouse) -> Self {
        Self { warehouse }
    }

    pub fn find_path(&self, start: &Position, end: &Position) -> Option<Vec<Position>> {
        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<Position, Position> = HashMap::new();
        let mut g_score: HashMap<Position, u32> = HashMap::new();
        let mut f_score: HashMap<Position, u32> = HashMap::new();
        let mut closed_set: HashSet<Position> = HashSet::new();

        let start_h = start.distance_to(end);
        g_score.insert(start.clone(), 0);
        f_score.insert(start.clone(), start_h);
        open_set.push(Reverse((start_h, 0, start.clone())));

        while let Some(Reverse((_f, _g, current))) = open_set.pop() {
            if current == *end {
                return Some(self.reconstruct_path(&came_from, current));
            }

            if closed_set.contains(&current) {
                continue;
            }
            closed_set.insert(current.clone());

            let current_g = *g_score.get(&current).unwrap_or(&0);

            for neighbor in self.warehouse.get_neighbors(&current) {
                if closed_set.contains(&neighbor) {
                    continue;
                }

                let tentative_g = current_g + 1;
                let neighbor_g = *g_score.get(&neighbor).unwrap_or(&u32::MAX);

                if tentative_g < neighbor_g {
                    came_from.insert(neighbor.clone(), current.clone());
                    g_score.insert(neighbor.clone(), tentative_g);
                    
                    let h = neighbor.distance_to(end);
                    let f = tentative_g + h;
                    f_score.insert(neighbor.clone(), f);
                    
                    open_set.push(Reverse((f, tentative_g, neighbor)));
                }
            }
        }

        None
    }

    fn reconstruct_path(&self, came_from: &HashMap<Position, Position>, current: Position) -> Vec<Position> {
        let mut path = vec![current];
        let mut current = &current;

        while let Some(prev) = came_from.get(current) {
            path.push(prev.clone());
            current = prev;
        }

        path.reverse();
        path
    }

    pub fn optimize_picking_path(
        &self,
        items: &[MergedItem],
        start: &Position,
        end: &Position,
    ) -> (Vec<Position>, Vec<usize>, u32) {
        if items.is_empty() {
            let direct_path = self.find_path(start, end).unwrap_or_else(|| vec![start.clone(), end.clone()]);
            let total_distance = direct_path.len().saturating_sub(1) as u32;
            return (direct_path, vec![], total_distance);
        }

        // 使用最近邻算法 + 2-opt 优化的 TSP 求解
        let mut pick_positions: Vec<(usize, Position)> = items
            .iter()
            .enumerate()
            .map(|(i, item)| (i, item.position.clone()))
            .collect();

        // 计算所有点之间的距离
        let mut distance_matrix = HashMap::new();
        let n = pick_positions.len();

        // 预计算所有点之间的路径
        for i in 0..n {
            for j in i + 1..n {
                if let Some(path) = self.find_path(
                    &pick_positions[i].1,
                    &pick_positions[j].1,
                ) {
                    let dist = path.len().saturating_sub(1) as u32;
                    distance_matrix.insert((i, j), dist);
                    distance_matrix.insert((j, i), dist);
                }
            }
        }

        // 起点到各点的距离
        let mut start_distances = Vec::with_capacity(n);
        for i in 0..n {
            if let Some(path) = self.find_path(start, &pick_positions[i].1) {
                start_distances.push((i, path.len().saturating_sub(1) as u32));
            } else {
                start_distances.push((i, u32::MAX / 2));
            }
        }

        // 使用最近邻算法构建初始路径
        let mut visited = vec![false; n];
        let mut visit_order = Vec::new();
        let mut total_distance = 0;

        // 从最近的点开始
        start_distances.sort_by_key(|&(_, d)| d);
        let mut current_idx = start_distances[0].0;
        visited[current_idx] = true;
        visit_order.push(current_idx);
        total_distance += start_distances[0].1;

        while visit_order.len() < n {
            let mut nearest = None;
            let mut min_dist = u32::MAX;

            for i in 0..n {
                if !visited[i] {
                    let dist = *distance_matrix.get(&(current_idx, i)).unwrap_or(&(u32::MAX / 2));
                    if dist < min_dist {
                        min_dist = dist;
                        nearest = Some(i);
                    }
                }
            }

            if let Some(idx) = nearest {
                visited[idx] = true;
                visit_order.push(idx);
                total_distance += min_dist;
                current_idx = idx;
            } else {
                break;
            }
        }

        // 添加从最后一个点到终点的距离
        if let Some(path) = self.find_path(&pick_positions[*visit_order.last().unwrap()].1, end) {
            total_distance += path.len().saturating_sub(1) as u32;
        }

        // 构建完整路径
        let mut full_path = Vec::new();
        let mut item_order = Vec::new();

        // 从起点到第一个拣货点
        if let Some(path) = self.find_path(start, &pick_positions[visit_order[0]].1) {
            full_path.extend(path);
        } else {
            full_path.push(start.clone());
        }

        for i in 0..visit_order.len() {
            let item_idx = visit_order[i];
            item_order.push(item_idx);

            // 添加拣货点（重复添加以确保它在路径中）
            if full_path.last() != Some(&pick_positions[item_idx].1) {
                full_path.push(pick_positions[item_idx].1.clone());
            }

            // 前往下一个点或终点
            if i < visit_order.len() - 1 {
                let next_idx = visit_order[i + 1];
                if let Some(path) = self.find_path(
                    &pick_positions[item_idx].1,
                    &pick_positions[next_idx].1,
                ) {
                    if !path.is_empty() {
                        full_path.extend(path.iter().skip(1).cloned());
                    }
                }
            } else {
                // 最后一个点，前往终点
                if let Some(path) = self.find_path(&pick_positions[item_idx].1, end) {
                    if !path.is_empty() {
                        full_path.extend(path.iter().skip(1).cloned());
                    }
                }
            }
        }

        (full_path, item_order, total_distance)
    }

    pub fn calculate_path_distance(&self, path: &[Position]) -> u32 {
        path.windows(2)
            .map(|w| w[0].distance_to(&w[1]))
            .sum()
    }
}
