

pub mod constants;
pub mod eval;
pub mod serch;
pub mod stored_moves;
pub mod engine;
pub mod debug_file;
pub mod time_spending;

pub mod opening;

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}

