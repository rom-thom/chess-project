pub mod position;
pub mod board;
pub mod kastling;
pub mod fen_string;
pub mod square;
pub mod attack;
pub mod piece;
pub mod moves;
pub mod bitboard_consts;
pub mod movegen;
pub mod test_core;
pub mod move_convertion;
pub mod movegen_per_piece;
pub mod debug_file;
pub mod zobrist;
#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
