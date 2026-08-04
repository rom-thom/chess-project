

use chess_core::position::{Color};

use crate::debug_file::log_dbg;





pub struct TimeUsage{
    pub wtime_ms: Option<u64>,
    pub btime_ms: Option<u64>,

    pub winc_ms: Option<u64>,
    pub binc_ms: Option<u64>,

    pub moves_to_ctrl: Option<u32>, // Moves until timecontroll

    pub fixed_move_time: Option<u64>,
}



impl TimeUsage{
    pub fn new(wtime_ms: Option<u64>, btime_ms: Option<u64>, winc_ms: Option<u64>, binc_ms: Option<u64>, moves_to_ctrl: Option<u32>, fixed_move_time: Option<u64>)->Self{
        Self { wtime_ms, btime_ms, winc_ms, binc_ms, moves_to_ctrl, fixed_move_time}
    }

    pub fn update(&mut self, wtime_ms: Option<u64>, btime_ms: Option<u64>, winc_ms: Option<u64>, binc_ms: Option<u64>, moves_to_ctrl: Option<u32>){
        self.wtime_ms = wtime_ms;
        self.btime_ms = btime_ms;
        self.moves_to_ctrl = moves_to_ctrl;
        self.winc_ms = winc_ms;
        self.binc_ms = binc_ms;
    }

    pub fn my_time_ms(&self, my_color: Color) -> Option<u64> {
        match my_color {
            Color::White => self.wtime_ms,
            Color::Black => self.btime_ms,
        }
    }
    pub fn opp_time_ms(&self, my_color: Color) -> Option<u64> {
        match my_color {
            Color::White => self.btime_ms,
            Color::Black => self.wtime_ms,
        }
    }
    pub fn my_inc_ms(&self, my_color: Color) -> Option<u64> {
        match my_color {
            Color::White => self.winc_ms,
            Color::Black => self.binc_ms,
        }
    }

    pub fn opp_inc_ms(&self, my_color: Color) -> Option<u64> {
        match my_color {
            Color::White => self.binc_ms,
            Color::Black => self.winc_ms,
        }
    }

    pub fn time_to_use_ms(&self, my_color: Color) -> Option<u64>{

        if let Some(fixed) = self.fixed_move_time{
            return Some(fixed)
        }

        let my_time = match self.my_time_ms(my_color) {
            Some(t) => t,
            None => {
                log_dbg("/tmp/chess_log/search_scores.log", "my_time_ms()_returned None", None::<&String>, file!(), line!()).expect("Oh no it feiled");
                return None
        }
        };


        let opp_time = self.opp_time_ms(my_color).unwrap_or(my_time); // if missing, assume equal
        let inc = self.my_inc_ms(my_color);

        // 3) Moves-to-go estimate
        let mtg: u64 = self.moves_to_ctrl.map(|x| x as u64).unwrap_or(30);

        // 4) Reserve: keep back 3% of remaining, at least 50ms
        // reserve = max(50, my_time * 3 / 100)
        let reserve = u64::max(50u64, my_time.saturating_mul(3) / 100);

        // 5) Usable time after reserve and overhead
        let usable = my_time
            .saturating_sub(reserve);

        // If we’re basically out of time, just move fast
        if usable <= 20 {
            return Some(1);
        }

        // 6) Base allocation + increment portion
        let mut budget = usable / (mtg + 2);

        match inc {
            None => (),
            Some(inc_part) => budget += (inc_part * 7)/10 // 70% of increment
        }

        // 7) Opponent-time modifiers (gentle!)
        if my_time > opp_time.saturating_mul(3) / 2 {
            budget = (budget * 11) / 10; // +10%
        } else if my_time.saturating_mul(3) / 2 < opp_time {
            budget = (budget * 85) / 100; // -15%
        }

        // 8) Caps: don’t spend too much of remaining time on one move
        let max_move = u64::max(10u64, usable / 4); // <= 25% of usable
        let min_move = 5u64;

        budget = u64::min(budget, max_move);
        budget = u64::max(budget, min_move);

        // 9) Panic mode caps
        if my_time < 2_000 {
            budget = u64::min(budget, 50);
        }
        if my_time < 1_000 {
            budget = u64::min(budget, 20);
        }
        if my_time < 500 {
            budget = u64::min(budget, 10);
        }

        Some(budget.max(1))
     
    }
}