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

    #[cfg(feature = "tt-stats")]
    pub stats: TTStats,
}





use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Default)] // TODO: This is mainly for debuging, so i may not need it in the future
pub struct TTStats {
    pub probes: AtomicU64,
    pub hits: AtomicU64,
    pub cutoffs: AtomicU64,
    pub stores: AtomicU64,
    pub replaces: AtomicU64,
    pub collisions: AtomicU64,
}


impl TT{

    // Create a table with `entries` amount of slots. `entries` must be a power of two so that we can use mask to index it (need for speed)
    pub fn new(entries: usize) -> Self {
        assert!(entries.is_power_of_two());
        Self {
            table: vec![TTEntry::EMPTY; entries],
            mask: entries - 1,
            age: 0,
            #[cfg(feature = "tt-stats")]
            stats: TTStats::default(),
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




    /// This returns TTProbe, that if there exist a position in the table that maches the current position, it returns the best move, 
    /// a bool telling it that it hit, and a cutof. The cutoff is the score at the end if the depth was deep enough and None otherwise.
    /// It only returns a cutoff if it is safe to asume that that is the best posible score
    pub fn probe(&self, key: ZobristKey, depth: i8, alpha: i32, beta: i32) -> TTProbe {

        #[cfg(feature = "tt-stats")]
        self.stats.probes.fetch_add(1, Ordering::Relaxed);


        let key_val = key.as_u64();
        let entry = self.table[self.index(key_val)];

        if entry.is_empty() || entry.key != key_val {
            #[cfg(feature = "tt-stats")]
            self.stats.collisions.fetch_add(1, Ordering::Relaxed);

            return TTProbe { cutoff: None, best: None, hit: false };
        }

        #[cfg(feature = "tt-stats")]
        self.stats.hits.fetch_add(1, Ordering::Relaxed);

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
            #[cfg(feature = "tt-stats")]
            if cutoff.is_some() {
                self.stats.cutoffs.fetch_add(1, Ordering::Relaxed);
            }

            return TTProbe { cutoff, best, hit: true };
        }
        

        TTProbe { cutoff: None, best, hit: true }
    }

    /// Store an entry. `bound` should be decided by alpha_orig/beta logic.
    pub fn store(&mut self, key: ZobristKey, depth: i8, score: i32, bound: Bound, best: Option<BitMove>) {
        #[cfg(feature = "tt-stats")]
        self.stats.stores.fetch_add(1, Ordering::Relaxed);

        let k = key.as_u64();
        let idx = self.index(k);
        let old = self.table[idx];

        // Replace if:
        // - empty
        // - different key
        // - deeper
        // - old entry from a previous age
        let replace = old.is_empty()
            || old.key != k
            || depth >= old.depth
            || old.age != self.age;

        #[cfg(feature = "tt-stats")]
        if replace && !old.is_empty() {
            self.stats.replaces.fetch_add(1, Ordering::Relaxed);
        }


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





#[cfg(feature = "tt-stats")]
impl TT {
    pub fn dump_stats(&self, depth: usize) {
        use std::{
            fs::OpenOptions,
            io::Write,
            sync::atomic::Ordering,
        };

        let probes = self.stats.probes.load(Ordering::Relaxed);
        let hits = self.stats.hits.load(Ordering::Relaxed);
        let cutoffs = self.stats.cutoffs.load(Ordering::Relaxed);
        let stores = self.stats.stores.load(Ordering::Relaxed);
        let replaces = self.stats.replaces.load(Ordering::Relaxed);
        let collisions = self.stats.collisions.load(Ordering::Relaxed);

        let hit_rate = if probes == 0 { 0.0 } else { hits as f64 / probes as f64 };
        let cutoff_rate = if probes == 0 { 0.0 } else { cutoffs as f64 / probes as f64 };

        let line = format!(
            "TT (depth {}): probes={} hits={} ({:.1}%) cutoffs={} ({:.1}%) stores={} replaces={} collisions={}\n",
            depth, probes, hits, 100.0 * hit_rate, cutoffs, 100.0 * cutoff_rate, stores, replaces, collisions
        );

        // append to file
        if let Ok(mut f) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("/tmp/tt-stats.log")
        {
            let _ = f.write_all(line.as_bytes());
        }

        // optional: keep printing too
        // eprint!("{line}");
    }

    pub fn reset_stats(&mut self) {
        use std::sync::atomic::Ordering;
        self.stats.probes.store(0, Ordering::Relaxed);
        self.stats.hits.store(0, Ordering::Relaxed);
        self.stats.cutoffs.store(0, Ordering::Relaxed);
        self.stats.stores.store(0, Ordering::Relaxed);
        self.stats.replaces.store(0, Ordering::Relaxed);
        self.stats.collisions.store(0, Ordering::Relaxed);
    }
}














#[cfg(test)]
mod tests {
    use super::*;
    use chess_core::position::ZobristKey;

    // Helper: construct a ZobristKey from a raw u64.
    // Change this if your chess_core API differs.
    fn k(x: u64) -> ZobristKey {
        ZobristKey::from_u64(x)
    }

    #[test]
    fn probe_miss_on_empty() {
        let tt = TT::new(8);
        let res = tt.probe(k(123), 4, -50, 50);
        assert!(!res.hit);
        assert!(res.cutoff.is_none());
        assert!(res.best.is_none());
    }

    #[test]
    fn store_and_probe_exact_cutoff_when_depth_sufficient() {
        let mut tt = TT::new(8);
        tt.store(k(42), 5, 17, Bound::Exact, None);

        let res = tt.probe(k(42), 5, -100, 100);
        assert!(res.hit);
        assert_eq!(res.cutoff, Some(17));
    }

    #[test]
    fn exact_entry_does_not_cutoff_if_depth_insufficient_but_is_a_hit() {
        let mut tt = TT::new(8);
        tt.store(k(99), 3, 30, Bound::Exact, None);

        // Request deeper than stored depth
        let res = tt.probe(k(99), 4, -100, 100);
        assert!(res.hit);
        assert!(res.cutoff.is_none());
    }

    #[test]
    fn lower_bound_only_cutoffs_when_score_ge_beta() {
        let mut tt = TT::new(8);
        tt.store(k(7), 6, 80, Bound::Lower, None);

        // score(80) >= beta(50) => cutoff
        let res1 = tt.probe(k(7), 6, -100, 50);
        assert!(res1.hit);
        assert_eq!(res1.cutoff, Some(80));

        // score(80) < beta(90) => no cutoff
        let res2 = tt.probe(k(7), 6, -100, 90);
        assert!(res2.hit);
        assert!(res2.cutoff.is_none());
    }

    #[test]
    fn upper_bound_only_cutoffs_when_score_le_alpha() {
        let mut tt = TT::new(8);
        tt.store(k(8), 6, -20, Bound::Upper, None);

        // score(-20) <= alpha(-10) => cutoff
        let res1 = tt.probe(k(8), 6, -10, 100);
        assert!(res1.hit);
        assert_eq!(res1.cutoff, Some(-20));

        // score(-20) <= alpha(-50)? no, -20 > -50 => no cutoff
        let res2 = tt.probe(k(8), 6, -50, 100);
        assert!(res2.hit);
        assert!(res2.cutoff.is_none());
    }

    #[test]
    fn collision_same_index_different_key_must_not_hit() {
        // entries = 8 => mask = 7, so keys 1 and 9 both index to 1 (1 & 7 == 1, 9 & 7 == 1)
        let mut tt = TT::new(8);
        tt.store(k(1), 5, 10, Bound::Exact, None);

        // Overwrite same slot with different key
        tt.store(k(9), 5, 20, Bound::Exact, None);

        // Probing old key should miss due to key mismatch
        let res_old = tt.probe(k(1), 5, -100, 100);
        assert!(!res_old.hit);
        assert!(res_old.cutoff.is_none());

        // New key should hit
        let res_new = tt.probe(k(9), 5, -100, 100);
        assert!(res_new.hit);
        assert_eq!(res_new.cutoff, Some(20));
    }

    #[test]
    fn replacement_policy_prefers_deeper_same_age() {
        let mut tt = TT::new(8);
        let key = k(1234);

        tt.store(key, 3, 11, Bound::Exact, None);
        tt.store(key, 5, 22, Bound::Exact, None); // deeper should replace

        let res = tt.probe(key, 5, -100, 100);
        assert!(res.hit);
        assert_eq!(res.cutoff, Some(22));
    }

    #[test]
    fn replacement_policy_does_not_replace_with_shallower_same_age() {
        let mut tt = TT::new(8);
        let key = k(555);

        tt.store(key, 6, 99, Bound::Exact, None);
        tt.store(key, 4, 11, Bound::Exact, None); // shallower, same age: should NOT replace

        let res = tt.probe(key, 6, -100, 100);
        assert!(res.hit);
        assert_eq!(res.cutoff, Some(99));
    }

    #[test]
    fn replacement_policy_replaces_previous_age_even_if_shallower() {
        let mut tt = TT::new(8);
        let key = k(777);

        // Age 0 by default
        tt.store(key, 6, 99, Bound::Exact, None);

        // New search increments age; policy allows replacing if old.age != self.age
        tt.new_search();
        tt.store(key, 2, 11, Bound::Exact, None); // shallower, but new age => replace

        let res = tt.probe(key, 2, -100, 100);
        assert!(res.hit);
        assert_eq!(res.cutoff, Some(11));
    }
}

