use chess_core::{moves::BitMove, position::ZobristKey};



#[derive(Clone, Copy)]
pub enum Bound { Exact, Lower, Upper }

#[derive(Clone, Copy)]
pub struct TTEntry {
    pub key: u64,
    pub depth: i8,
    pub score: i32,
    pub bound: Bound,
    pub best: Option<BitMove>,
    pub age: u8, // optional but very helpful
}


impl TTEntry{
    pub const EMPTY: TTEntry = TTEntry {
        key: 0,
        depth: i8::MIN,
        score: 0,
        bound: Bound::Upper, // arbitrary
        best: None,
        age: 0,
    };

    #[inline]
    pub fn is_empty(&self) -> bool { self.depth == i8::MIN }
}


pub struct TTProbe {
    pub cutoff: Option<i32>,
    pub best: Option<BitMove>,
    pub hit: bool,
}


pub struct TT {
    table: Vec<TTEntry>,
    mask: usize,
    age: u8,
}



impl TT{

    // Create a table with `entries` amount of slots. `entries` must be a power of two so that we can use mask to index it (need for speed)
    pub fn new(entries: usize) -> Self {
        assert!(entries.is_power_of_two());
        Self {
            table: vec![TTEntry::EMPTY; entries],
            mask: entries - 1,
            age: 0,
        }
    }

    pub fn index(&self, key: u64) -> usize {
        (key as usize) & self.mask
    }

    // Clear all entries.
    pub fn clear(&mut self) {
        self.table.fill(TTEntry::EMPTY);
        self.age = 0;
    }

    // Call once per root search iteration (or each new root search).
    pub fn new_search(&mut self) {
        self.age = self.age.wrapping_add(1);
    }





    /// Probe the table. If depth is sufficient and the bound proves something for the
    /// (alpha, beta) window, returns `cutoff=Some(score)`.
    pub fn probe(&self, key: ZobristKey, depth: i8, alpha: i32, beta: i32) -> TTProbe {
        let key_val = key.as_u64();
        let entry = self.table[self.index(key_val)];

        if entry.is_empty() || entry.key != key_val {
            return TTProbe { cutoff: None, best: None, hit: false };
        }

        // Always return best move for ordering if present.
        let best = entry.best;

        // Only allow cutoff/return if entry depth is deep enough.
        if entry.depth >= depth {
            let cutoff = match entry.bound {
                Bound::Exact => Some(entry.score),
                Bound::Lower if entry.score >= beta => Some(entry.score),
                Bound::Upper if entry.score <= alpha => Some(entry.score),
                _ => None,
            };
            return TTProbe { cutoff, best, hit: true };
        }

        TTProbe { cutoff: None, best, hit: true }
    }

    /// Store an entry. `bound` should be decided by alpha_orig/beta logic.
    pub fn store(&mut self, key: ZobristKey, depth: i8, score: i32, bound: Bound, best: Option<BitMove>) {
        let k = key.as_u64();
        let idx = self.index(k);
        let old = self.table[idx];

        // Simple replacement policy:
        // - replace if empty
        // - or different key
        // - or deeper
        // - or old entry from a previous age
        let replace = old.is_empty()
            || old.key != k
            || depth >= old.depth
            || old.age != self.age;

        if replace {
            self.table[idx] = TTEntry {
                key: k,
                depth,
                score,
                bound,
                best,
                age: self.age,
            };
        }
    }
}


