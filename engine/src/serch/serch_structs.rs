use std::{sync::{Arc, atomic::{self, AtomicBool, Ordering}}, time::Instant};

use chess_core::moves::{BitMove, MovePath};




#[derive(Debug, Default)]
pub struct SearchResult {
    pub best_move: Option<BitMove>,
    pub score: i32,
    pub pv: MovePath,   // starting with best_move and as long as posible. Not always to the end of the serch
    pub depth: usize,
    pub nodes: u64,
    pub aborted: bool,
}

impl SearchResult{
    pub fn abort() -> Self{
        Self { best_move: None, score: 0, pv: MovePath::new_empty(), depth: 0, nodes: 0, aborted: true }
    }
}

pub struct SearchLimits{
    stop: Arc<AtomicBool>,
    start_time: Instant,
    pub max_depth: Option<usize>,
    pub max_time_ms: Option<u64>,
    pub max_nodes: Option<u64>,

    node_count: usize, // this is for checking when i should check for time stops for speed
    nodes_to_check: usize,
}


impl SearchLimits{
    pub fn new(
        max_depth: Option<usize>,
        max_time_ms: Option<u64>,
        max_nodes: Option<u64>,
        nodes_to_check: Option<usize>,
        stopflag: Arc<AtomicBool>,
    ) -> Self {
        let nodes_to_check = nodes_to_check.unwrap_or(1024);
        Self {
            stop: stopflag,
            start_time: Instant::now(),
            max_depth,
            max_time_ms,
            max_nodes,
            node_count: 0,
            nodes_to_check,
        }
    }

    pub fn time_elapsed_ms(&self) -> u128 {
        self.start_time.elapsed().as_millis()
    }

    #[inline]
    pub fn get_node_count(&self) -> u64{
        self.node_count as u64
    }

    #[inline]
    pub fn reset_stop(&mut self){
        self.stop.store(false, Ordering::Relaxed);
    }

    pub fn start_new_search(&mut self){
        self.start_time = Instant::now();
        self.node_count = 0;
    }

    // Executed rarely
    pub fn should_stop(&self) -> bool{
        if let Some(time_limit) = self.max_time_ms {
            if self.start_time.elapsed() >= std::time::Duration::from_millis(time_limit as u64){
                return true;
            }
        }
        if let Some(node_limit) = self.max_nodes{
            if self.node_count >= node_limit as usize{
                return true
            }
        }

        false
    }

    // Executed every node
    pub fn visit_node_and_check_stop(&mut self) -> bool{
        self.node_count += 1;
        
        if self.stop.load(Ordering::Relaxed){return true;}

        // Node limits must be checked on every node. Sorry speed, thats the rule
        if let Some(max_nodes) = self.max_nodes {
            if self.node_count >= max_nodes as usize{
                return true;
            }   
        }

        if self.node_count % self.nodes_to_check == 0{
            return self.should_stop();
        }
        false
    }
}