use crate::board::{Square, Move};
use crate::piece::PieceType;

/*
 *                 Bitmasks

 *       BitMaskA           BitMaskB
 *   0 1 1 1 1 1 1 1     1 1 1 1 1 1 1 0
 *   0 1 1 1 1 1 1 1     1 1 1 1 1 1 1 0
 *   0 1 1 1 1 1 1 1     1 1 1 1 1 1 1 0
 *   0 1 1 1 1 1 1 1     1 1 1 1 1 1 1 0
 *   0 1 1 1 1 1 1 1     1 1 1 1 1 1 1 0
 *   0 1 1 1 1 1 1 1     1 1 1 1 1 1 1 0
 *   0 1 1 1 1 1 1 1     1 1 1 1 1 1 1 0
 *   0 1 1 1 1 1 1 1     1 1 1 1 1 1 1 0

 *       BitMaskA2          BitMaskB2
 *   0 0 1 1 1 1 1 1     1 1 1 1 1 1 0 0
 *   0 0 1 1 1 1 1 1     1 1 1 1 1 1 0 0
 *   0 0 1 1 1 1 1 1     1 1 1 1 1 1 0 0
 *   0 0 1 1 1 1 1 1     1 1 1 1 1 1 0 0
 *   0 0 1 1 1 1 1 1     1 1 1 1 1 1 0 0
 *   0 0 1 1 1 1 1 1     1 1 1 1 1 1 0 0
 *   0 0 1 1 1 1 1 1     1 1 1 1 1 1 0 0
 *   0 0 1 1 1 1 1 1     1 1 1 1 1 1 0 0
 */

pub const BIT_MASK_A:  u64 = !0x101010101010101;
pub const BIT_MASK_A2: u64 = !0x303030303030303;

pub const BIT_MASK_B:  u64 = !0x8080808080808080;
pub const BIT_MASK_B2: u64 = !0xc0c0c0c0c0c0c0c0;


pub const W_PAWN_START: u64 = 0xff000000000000;
pub const B_PAWN_START: u64 = 0xff00;



pub fn square_from_uci(uci_square: &String) -> Square {
    let mut c = uci_square.chars();
    ((c.next_back().unwrap() as u32 - '1' as u32) * 8
    + c.next().unwrap()      as u32 - 'a' as u32) as Square
}


pub fn move_from_uci(uci_move: &String) -> Move {
    let mut c = uci_move.chars();
    let c0 = c.next().unwrap();
    let c1 = c.next().unwrap();
    let c2 = c.next().unwrap();
    let c3 = c.next().unwrap();

    Move {
        from: ((c1 as u32 - '1' as u32) * 8 + c0 as u32 - 'a' as u32) as u8,
        to:   ((c3 as u32 - '1' as u32) * 8 + c2 as u32 - 'a' as u32) as u8,

        promotion_piece_type:
        if uci_move.len() > 5 {
            Some(PieceType::from(c.next().unwrap()))
        }
        else {
            None
        },

        is_castle: false,
    }
}
