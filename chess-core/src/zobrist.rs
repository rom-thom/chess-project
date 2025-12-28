use crate::{board::Bitboards, kastling::{Castling, CastlingSide, Imposter}, moves::{BitMove, Move}, piece::{Piece, PieceIndex}, position::{Color, Position, Snapshot}, square::Square};
use std::sync::OnceLock;


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
impl ZobristKey{

    pub fn make_move(&mut self, mov: &BitMove, boards_before_move: &Bitboards, captured_piece_square: Option<(PieceIndex, Square)>, color: Color, new_castling: &Castling, old_castling: &Castling, new_ep: Option<&Square>, old_ep: Option<&Square>){
        let from = mov.get_start_square();
        let to = mov.get_end_square();
        let piece = mov.get_piece(boards_before_move);

        self.0 ^= zob().side;
        self.0 ^= zob().piece_sq[piece.index()][from.index() as usize];
        self.0 ^= zob().piece_sq[piece.index()][to.index() as usize];
        
        
        if let Some(premoted) = mov.get_premotion_piece(){
            let p = PieceIndex::from_piece(premoted, color);
            self.0 ^= zob().piece_sq[piece.index()][to.index() as usize]; // To remove the previously set square
            self.0 ^= zob().piece_sq[p.index()][to.index() as usize];

        }

        if let Some(piece_square_taken) = captured_piece_square{
            self.0 ^= zob().piece_sq[piece_square_taken.0.index() as usize][piece_square_taken.1.index() as usize];
        }
        
        if let Some(side) = mov.get_castle_side(){
            // Here i dont need to change the normal move (from the king) as that is done above
            let (rook, rook_start_square, rook_end_square) = match (color, side) {
                (Color::White, Imposter::King)  => (PieceIndex::WhiteRook, Square::H1, Square::F1),
                (Color::White, Imposter::Queen) => (PieceIndex::WhiteRook, Square::A1, Square::D1),
                (Color::Black, Imposter::King)  => (PieceIndex::BlackRook, Square::H8, Square::F8),
                (Color::Black, Imposter::Queen) => (PieceIndex::BlackRook, Square::A8, Square::D8),
            };
            self.0 ^= zob().piece_sq[rook.index()][rook_start_square.index() as usize] ^ zob().piece_sq[rook.index()][rook_end_square.index() as usize];
        }


        self.0 ^= zob().castle[old_castling.rights as usize];
        self.0 ^= zob().castle[new_castling.rights as usize];


        //TODO: Add the en passant part, that checks for legality of en passant captures (this is not strictly nesesary, but its usefull)

        // if let Some(old_ep) = old_ep_sq_if_capturable { self.0 ^= zob().ep_file[old_ep.col() as usize]; }
        // if let Some(new_ep) = new_ep_sq_if_capturable { self.0 ^= zob().ep_file[new_ep.col() as usize]; }

        if let Some(ep) = old_ep { self.0 ^= zob().ep_file[ep.col() as usize]; }
        if let Some(ep) = new_ep { self.0 ^= zob().ep_file[ep.col() as usize]; }


    }
}


#[derive(Debug)]
pub struct Zobrist { pub piece_sq: [[u64;64];12], pub side: u64, pub castle: [u64;16], pub ep_file: [u64;8] }


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



// Ensures the Zobrist table is initialized only once for the entire duration of the program and can be accessed globally.

static ZOBRIST: OnceLock<Zobrist> = OnceLock::new();

fn zob() -> &'static Zobrist { // ?? If i ever want to use zob() anyware else change this to pub fn ...
    ZOBRIST.get_or_init(|| Zobrist::new(0x9E3770B9AF4A7C15))
}




#[test]
fn test_zobrist(){
    let s = Zobrist::new(0x9E3770B9AF4A7C15);
    for _ in 0..9{
        dbg!(&s);
    }
}