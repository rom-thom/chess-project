use std::{sync::atomic::{self, AtomicBool}, time::Instant};

use chess_core::moves::{BitMove, MovePath};




#[derive(Debug, Default)]
pub struct SearchResult {
    pub best_move: Option<BitMove>,
    pub score: i32,
    pub pv: MovePath,   // starting with best_move and as long as posible. Not always to the end of the serch
    pub depth: usize,
    // pub nodes: u64, Should maby have this for debuging but i cant be bathered
    pub aborted: bool,
}

impl SearchResult{
    pub fn abort() -> Self{
        Self { best_move: None, score: 0, pv: MovePath::new_empty(), depth: 0, aborted: true }
    }
}

pub struct SearchLimits{
    stop: AtomicBool,
    start_time: Instant,
    pub max_depth: Option<usize>,
    pub max_time_ms: Option<u64>,

    node_count: usize, // this is for checking when i should check for time stops for speed
    nodes_to_check: usize,
}


impl SearchLimits{
    pub fn new(
        max_depth: Option<usize>,
        max_time_ms: Option<u64>,
        nodes_to_check: Option<usize>,
    ) -> Self {
        let nodes_to_check = nodes_to_check.unwrap_or(1024);
        Self {
            stop: AtomicBool::new(false),
            start_time: Instant::now(),
            max_depth,
            max_time_ms,
            node_count: 0,
            nodes_to_check,
        }
    }

    pub fn start_new_search(&mut self){
        self.start_time = Instant::now();
        self.node_count = 0;
        self.stop.store(false, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn should_stop(&self) -> bool{
        if self.stop.load(std::sync::atomic::Ordering::Relaxed){
            return true;
        }
        if let Some(limit) = self.max_time_ms {
            self.start_time.elapsed() >= std::time::Duration::from_millis(limit as u64)
        } else {
            false
        }
    }

    pub fn check_stop(&mut self) -> bool{
        self.node_count += 1;

        if self.node_count % self.nodes_to_check == 0{
            return self.should_stop();
        }
        false
    }
}