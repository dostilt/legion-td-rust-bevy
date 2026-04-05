use bevy::prelude::*;
use std::collections::VecDeque;
use crate::components::TowerObstacle;

#[derive(Resource, Clone)]
pub struct FlowField {
    pub width: usize,
    pub height: usize,
    pub directions: Vec<Vec2>,
    pub blocked: Vec<bool>,
    pub target: IVec2,
}

impl FlowField {
    pub fn new(width: usize, height: usize, target: IVec2) -> Self {
        Self {
            width,
            height,
            directions: vec![Vec2::ZERO; width * height],
            blocked: vec![false; width * height],
            target,
        }
    }

    pub fn idx(&self, x: i32, y: i32) -> Option<usize> {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            Some((y as usize) * self.width + (x as usize))
        } else {
            None
        }
    }

    pub fn pos_to_grid(&self, pos: Vec3) -> IVec2 {
        // Assume center of lane is x=0, z=0. The grid is width 8 (-4 to +4) and height 40 (-20 to +20).
        // For simplicity, let's say bottom-left is (0,0) mapped from (-4, -20)
        let x = (pos.x + 4.0).floor() as i32;
        let y = (pos.z + 20.0).floor() as i32;
        IVec2::new(x, y)
    }

    pub fn get_direction(&self, pos: Vec3) -> Vec2 {
        let grid_pos = self.pos_to_grid(pos);
        if let Some(i) = self.idx(grid_pos.x, grid_pos.y) {
            let dir = self.directions[i];
            if dir == Vec2::ZERO {
                Vec2::new(0.0, 1.0) // Fallback to +Z (King) if no path
            } else {
                dir
            }
        } else {
            // Fallback: move towards +Z
            Vec2::new(0.0, 1.0)
        }
    }

    pub fn compute(&mut self) {
        // Reset directions
        for d in self.directions.iter_mut() {
            *d = Vec2::ZERO;
        }

        let mut distances = vec![u32::MAX; self.width * self.height];
        let mut queue = VecDeque::new();

        if let Some(start_i) = self.idx(self.target.x, self.target.y) {
            distances[start_i] = 0;
            queue.push_back(self.target);
        }

        let neighbors = [
            IVec2::new(1, 0), IVec2::new(-1, 0),
            IVec2::new(0, 1), IVec2::new(0, -1),
            IVec2::new(1, 1), IVec2::new(-1, -1),
            IVec2::new(1, -1), IVec2::new(-1, 1),
        ];

        // BFS / Dijkstra
        while let Some(current) = queue.pop_front() {
            let current_idx = self.idx(current.x, current.y).unwrap();
            let dist = distances[current_idx];

            for n in neighbors.iter() {
                let next = current + *n;
                if let Some(next_idx) = self.idx(next.x, next.y) {
                    if !self.blocked[next_idx] {
                        let step_cost = if n.x != 0 && n.y != 0 { 14 } else { 10 };
                        if distances[next_idx] > dist + step_cost {
                            distances[next_idx] = dist + step_cost;
                            queue.push_back(next);
                            
                            // Point direction vector from neighbor to current
                            let dir = Vec2::new(-n.x as f32, -n.y as f32).normalize_or_zero();
                            self.directions[next_idx] = dir;
                        }
                    }
                }
            }
        }
    }
}


pub fn update_flow_field_system(
    mut flow_field: ResMut<FlowField>,
    tower_query: Query<&Transform, With<TowerObstacle>>,
    mut tower_count: Local<usize>,
    mut initialized: Local<bool>,
) {
    let current_count = tower_query.iter().count();
    if !*initialized || *tower_count != current_count {
        *initialized = true;
        *tower_count = current_count;
        
        // Reset blocked
        for b in flow_field.blocked.iter_mut() {
            *b = false;
        }

        // Mark blocked based on towers
        for transform in tower_query.iter() {
            let grid_pos = flow_field.pos_to_grid(transform.translation);
            if let Some(i) = flow_field.idx(grid_pos.x, grid_pos.y) {
                flow_field.blocked[i] = true;
            }
        }

        flow_field.compute();
    }
}
