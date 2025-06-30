


use crate::piece::PieceIndex;
use crate::position::{Color, Position};
use crate::square::{Square};
use crate::board::Bitboard;



// TODO this can be changes to insted use magic. Look up magic bitboard. I am to lacy to do it now.



fn shift_attack(
    mut original_square: Bitboard, 
    max_shift: usize, 
    all_occ: Bitboard, 
    shift_fn: fn(&mut Bitboard), 

    ) -> Bitboard{
    let mut result_occ = Bitboard::new_empty();

    for _ in 0..max_shift{
        shift_fn(&mut original_square);
        result_occ |= original_square;

        if all_occ.intersects(original_square){
            break;
        }
    }
    return result_occ;
}


pub fn rook_attacks(square: Square, all_occ: Bitboard)-> Bitboard{
    let original_square_mask = Bitboard::from(square.index());
    let (row, col) = Bitboard::index_to_coord(square.index());

    let mut result_occ = Bitboard::new_empty();

    result_occ |= shift_attack(original_square_mask, 7-row, all_occ, Bitboard::shift_up);
    result_occ |= shift_attack(original_square_mask, row, all_occ, Bitboard::shift_down);
    result_occ |= shift_attack(original_square_mask, 7-col, all_occ, Bitboard::shift_right);
    result_occ |= shift_attack(original_square_mask, col, all_occ, Bitboard::shift_left);
    result_occ
}
pub fn bishop_attacks(square: Square, all_occ: Bitboard)-> Bitboard{
    let original_square_mask = Bitboard::from(square.index());
    let (row, col) = Bitboard::index_to_coord(square.index());

    let mut result_occ = Bitboard::new_empty();

    let max_shift_ur = usize::min(7-row, 7-col);
    let max_shift_ul = usize::min(7-row, col);
    let max_shift_dr = usize::min(row, 7-col);
    let max_shift_dl = usize::min(row, col);

    result_occ |= shift_attack(original_square_mask, max_shift_ul, all_occ, Bitboard::shift_upp_left);
    result_occ |= shift_attack(original_square_mask, max_shift_ur, all_occ, Bitboard::shift_upp_right);
    result_occ |= shift_attack(original_square_mask, max_shift_dr, all_occ, Bitboard::shift_down_right);
    result_occ |= shift_attack(original_square_mask, max_shift_dl, all_occ, Bitboard::shift_down_left);
    result_occ
}
pub fn queen_attacks(square: Square, all_occ: Bitboard)-> Bitboard{
    let mut result_occ = rook_attacks(square, all_occ);
    result_occ |= bishop_attacks(square, all_occ);
    result_occ

}
pub fn king_attacks(square: Square)-> Bitboard{
    let orig   = Bitboard::from(square.index());
    let mut atk = Bitboard::new_empty();


    let mut tmp = orig;
    tmp.shift_up();    atk |= tmp;

    tmp = orig;
    tmp.shift_down();  atk |= tmp;

    tmp = orig;
    tmp.shift_left();  atk |= tmp;

    tmp = orig;
    tmp.shift_right(); atk |= tmp;

    tmp = orig;
    tmp.shift_upp_left();   atk |= tmp;

    tmp = orig;
    tmp.shift_upp_right();  atk |= tmp;

    tmp = orig;
    tmp.shift_down_left();  atk |= tmp;

    tmp = orig;
    tmp.shift_down_right(); atk |= tmp;

    atk
}
pub fn knight_attacks(square: Square)-> Bitboard{
    let (row, col) = Bitboard::index_to_coord(square.index());
    let possible_moves = [
        (2, 1), (2, -1), (-2, 1), (-2, -1),
        (1, 2), (1, -2), (-1, 2), (-1, -2),
    ];
    let mut result_board = Bitboard::new_empty();

    for (dx, dy) in possible_moves.into_iter(){
        let target = (row as i32 + dx, col as i32 + dy);
        if target.0 <= 7 && target.0 >=0 && target.1 <= 7 && target.1 >= 0{
            result_board |= Bitboard::from(Bitboard::coord_to_index(target.0 as usize, target.1 as usize));
        }
    }
    result_board
}
pub fn pawn_attacks(square: Square, all_occ: Bitboard, color : Color)-> Bitboard{
    let origin = Bitboard::from(square.index());
    let (row, _) = Bitboard::index_to_coord(square.index());
    let mut temp = origin;
    let mut result_occ = Bitboard::new_empty();
    match color {
        Color::White => {
            temp.shift_up();
            
            if !all_occ.intersects(temp){ // Forran
                result_occ |= temp;
                temp.shift_up();
                if !all_occ.intersects(temp) && row == 1{ // to forran
                    result_occ |= temp;
                }
            }
            temp = origin;
            temp.shift_upp_right();
            result_occ |= temp;
            temp = origin;
            temp.shift_upp_left();
            result_occ |= temp;

        },
        Color::Black => {
            temp.shift_down();
            
            if !all_occ.intersects(temp){ // Bak
                result_occ |= temp;
                temp.shift_down();
                if !all_occ.intersects(temp) && row == 6{ // to bak
                    result_occ |= temp;
                }
            }
            temp = origin;
            temp.shift_down_right();
            result_occ |= temp;
            temp = origin;
            temp.shift_down_left();
            result_occ |= temp;

        }
    };
    result_occ
}


pub fn get_attacks(piece:PieceIndex, square: Square, all_occ: Bitboard, color : Color)-> Bitboard{
    match piece {
        PieceIndex::WhitePawn | PieceIndex::BlackPawn => {
            pawn_attacks(square, all_occ, color)
        }
        PieceIndex::WhiteKnight | PieceIndex::BlackKnight => {
            knight_attacks(square)
        }
        PieceIndex::WhiteBishop | PieceIndex::BlackBishop => {
            bishop_attacks(square, all_occ)
        }
        PieceIndex::WhiteRook | PieceIndex::BlackRook => {
            rook_attacks(square, all_occ)
        }
        PieceIndex::WhiteQueen | PieceIndex::BlackQueen => {
            queen_attacks(square, all_occ)
        }
        PieceIndex::WhiteKing | PieceIndex::BlackKing => {
            king_attacks(square)
        }
    }
}




#[test]
fn test_attacks(){
    
    let position = Position::new(Some("r3k3/8/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1"));



    let new_b = rook_attacks(Square::from_idx(56).unwrap(), position.current.bitboards.all_occupancy);

    dbg!(new_b);

}