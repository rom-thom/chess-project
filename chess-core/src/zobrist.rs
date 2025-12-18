use crate::{position::{Color, Position, Snapshot}, square::Square};



impl Position{
    
    pub fn zobrist_key(&self) -> ZobristKey{
        self.current.zobrist_key
    }
}



#[derive(Clone, Copy, PartialEq)]
pub struct ZobristKey(u64);

impl Default for ZobristKey {
    fn default() -> Self {
        ZobristKey(0) // create an "empty" zobrist key
    }
}


#[derive(Debug)]
pub struct Zobrist { piece_sq: [[u64;64];12], side: u64, castle: [u64;16], ep_file: [u64;8] }


#[derive(Clone)]
struct SplitMix64(u64);

impl SplitMix64 { // Dont ask what happens here. i know it produses a new u64 number and it is the same across runs, but why(i have no clue)
    fn new(seed: u64) -> Self { Self(seed) }
    fn next_u64(&mut self) -> u64 {
        let mut z = { self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15); self.0 };
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

impl Zobrist {
    pub fn new(seed: u64) -> Self {
        let mut rng = SplitMix64::new(seed);

        let mut piece_sq = [[0u64; 64]; 12];
        for p in 0..12 {
            for sq in 0..64 {
                piece_sq[p][sq] = rng.next_u64(

                );
            }
        }

        let side = rng.next_u64();

        let mut castle = [0u64; 16];
        for m in 0..16 {
            castle[m] = rng.next_u64();
        }

        let mut ep_file = [0u64; 8];
        for f in 0..8 {
            ep_file[f] = rng.next_u64();
        }

        Self { piece_sq, side, castle, ep_file }
    }




    pub fn compute(&self, snapshot: &Snapshot) -> ZobristKey {
        let mut key: u64 = 0;


        for sq in 0..64 {
            if let Some(p) = snapshot.bitboards.piece_on_square(Square::from_idx(sq).expect("this should have returned a square as i loop trough legal squares")) { // <- you implement this accessor
                key ^= self.piece_sq[p.index()][sq as usize];
            }
        }

        if snapshot.side_to_move == Color::Black {
            key ^= self.side;
        }

        let cr = snapshot.castling.rights as usize;
        key ^= self.castle[cr];

        if let Some(ep_sq) = snapshot.en_passant {
            // TODO make it check wether the EP square is capturable or not, and if it isn't then we can ignore this part
            // if snapshot.ep_capture_is_legalish(ep_sq) {// <- implement (cheap check)
            key ^= self.ep_file[ep_sq.col() as usize];
            
        }

        ZobristKey(key)
    }
}





#[test]
fn test_zobrist(){
    let s = Zobrist::new(0x9E3770B9AF4A7C15);
    for _ in 0..9{
        dbg!(&s);
    }
}