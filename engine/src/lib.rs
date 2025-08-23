pub mod static_eval;
pub mod dynamic_eval;
pub mod serch;
pub mod piece_square_table;
pub mod positional_eval;
#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
