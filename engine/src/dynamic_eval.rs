

use chess_core::{moves::{BitMove, MoveList}, position::Position};




// For picking a sertain move type
pub enum PickerMode{All, CapturesOnly} // Add more as i go


// This trait should make me able to sort from worst to best move
pub trait MoveOrdering{
    fn sort(&mut self, pos: &Position, moves: &mut [BitMove]);
}



pub struct MovePicker{
    list: MoveList,
    idx: usize
}

impl Iterator for MovePicker{
    type Item = BitMove;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.list.size() { return None; }
        let m = self.list.as_slice()[self.idx];
        self.idx += 1;
        Some(m)
    }
}