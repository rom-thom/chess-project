// For static evaluation
use chess_core::position::{Color, Position};
use chess_core::piece::PieceIndex;
use chess_core::board::Bitboards;


const VALS: [i32; 5] = [100, 320, 330, 500, 900]; // P,N,B,R,Q // ?can be local as i wil only use the evaluate function later i think

// Low-level, pure, easy to unit-test
#[inline(always)]
fn material_from_bitboards(bb: &Bitboards, color: Color) -> i32 {
    let pcs = bb.color_slice(color);           // [P,N,B,R,Q,K]
    let mut sum = 0i32;
    for (k, &b) in pcs.iter().take(5).enumerate() {
        sum += (b.count_ones() as i32) * VALS[k]; // VALS = [100,320,330,500,900]
    }
    if matches!(color, Color::White) { sum } else { -sum }
}

// Public API – flexible to evolve later
#[inline]
pub fn evaluate_material(pos: &Position) -> i32 {
    // If you add a material cache, return it here:
    // return pos.material_cache[color as usize];

    material_from_bitboards(&pos.current.bitboards, pos.current.side_to_move) + 
    material_from_bitboards(&pos.current.bitboards, !pos.current.side_to_move)
}




// TODO this should be a lot more extensive, looking at the positional state and so on
pub fn evaluate(pos: &Position) -> i32 { 



    evaluate_material(pos)
}



#[test]
fn test_eval(){
    let pos = Position::new(Some("8/3k4/5q2/8/5Q2/3K4/8/8 w - - 0 1"));
    dbg!(evaluate(&pos));
}