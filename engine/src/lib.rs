

pub mod piece_square_table;
pub mod score;
pub mod eval;
pub mod serch;
pub mod trans_pos_table;
pub mod engine;


#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}

