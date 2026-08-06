use chess_core::{movegen::MoveGen, moves::{BitMove, MoveList}, position::{Color, Position}};
use rand::RngExt;
use std::{collections::HashMap, vec};




pub struct OpeningBook{
    book: HashMap<u64, Vec<(BitMove, u32)>>,
}

impl OpeningBook{
    pub fn lookup(&self, pos: &Position) -> Option<&Vec<(BitMove, u32)>>{
        let key = &pos.zobrist_key().as_u64();

        self.book.get(key)
    }

    pub fn parse(book_txt: &str, color: Color) -> Self{
        let mut book = HashMap::new();

        for line in book_txt.lines(){
            let line = line.trim();

            if line.is_empty() || line.starts_with("#"){
                continue;
            }

            let mut tokens = line.split_whitespace();
            let weight: u32 = tokens.next().expect("manglar vekt på linje").parse().expect("vekt må vere eit heiltal");

            let mut pos = Position::new(None);

            for (iter_nr, move_str) in tokens.enumerate(){

                let mov = MoveGen::stringmove_to_bitmove(&mut pos, move_str).unwrap_or_else(|e| panic!("Move conversion failed: {e}"));

                match color {
                    Color::White => if !iter_nr.is_multiple_of(2){
                        pos.make_move(&mov);
                        continue;
                    }
                    Color::Black => if iter_nr.is_multiple_of(2){
                        pos.make_move(&mov);
                        continue;
                    }
                }

                let zobrist_key = pos.zobrist_key().as_u64();

                book.entry(zobrist_key).and_modify(|moves: &mut Vec<(BitMove, u32)>| moves.push((mov, weight))).or_insert_with(|| vec![(mov, weight)]);

                pos.make_move(&mov);
            }
        }


        Self { book }
    }

    pub fn pick(&self, pos: &Position, rng: &mut impl rand::Rng) -> Option<BitMove>{
        let candidates = self.lookup(pos)?;
        let total = candidates.iter().map(|(_, weight)| weight).sum();

        let random = rng.random_range(0..total);
        let mut current_sum = 0;
        for (b_move, weight) in candidates{
            current_sum += weight;
            if current_sum > random{
                return Some(*b_move)
            }
        };
        panic!("This part should be unreachable unless picker has failed you")
    }

    
}



const OPENING_WHITE: &str = include_str!("opening_white.txt");
const OPENING_BLACK: &str = include_str!("opening_black.txt");

use std::sync::LazyLock;

pub static WHITE_BOOK: LazyLock<OpeningBook> =
    LazyLock::new(|| OpeningBook::parse(OPENING_WHITE, Color::White));
pub static BLACK_BOOK: LazyLock<OpeningBook> =
    LazyLock::new(|| OpeningBook::parse(OPENING_BLACK, Color::Black));



