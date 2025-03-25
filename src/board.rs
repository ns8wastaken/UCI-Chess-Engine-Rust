use crate::piece::*;
use crate::utils;


pub type Bitboard = u64;
pub type BitboardArray = [Bitboard; 12];
pub type Mailbox = [Option<Piece>; 64];
pub type Square = u8;


pub struct Move {
    pub from:                 u8,
    pub to:                   u8,
    pub promotion_piece_type: Option<PieceType>,
    pub is_castle:            bool,
}


pub struct PrecomputedMoves {
    pub knight_moves: [Bitboard; 64],
    pub king_moves:   [Bitboard; 64],
}


pub struct BoardState {
    pub bitboards:         BitboardArray,
    pub mailbox:           Mailbox,
    pub en_passant_square: Square,
    pub castling_flags:    u8,
    pub occupied_squares:  [Bitboard; 2],
}


pub struct Board {
    pub precomputed_moves: PrecomputedMoves,
    pub history: Vec<BoardState>,

    pub castling_flags:    u8,
    pub en_passant_square: u8,

    pub bitboards: BitboardArray,
    pub mailbox: Mailbox,

    pub attacked_squares: Bitboard,

    // 0: Black
    // 1: White
    pub occupied_squares: [Bitboard; 2],
}


impl Board {
    pub fn precompute_moves(&mut self) {
        /*
         *            Bitshift offsets
         *
         *         | <<15 |      | <<17 |
         *   ------|------|------|------|------
         *    << 6 | << 7 | << 8 | << 9 | <<10
         *   ------|------|------|------|------
         *         | << 1 |   0  | >> 1 |
         *   ------|------|------|------|------
         *    >>10 | >> 9 | >> 8 | >> 7 | >> 6
         *   ------|------|------|------|------
         *         | >>17 |      | >>15 |
         */

        for i in 0usize..64usize {
            let position: Bitboard = 1 << i;

            // Knight
            self.precomputed_moves.knight_moves[i] |= (position & utils::BIT_MASK_A) << 17;
            self.precomputed_moves.knight_moves[i] |= (position & utils::BIT_MASK_A) << 15;
            self.precomputed_moves.knight_moves[i] |= (position & utils::BIT_MASK_B2) << 10;
            self.precomputed_moves.knight_moves[i] |= (position & utils::BIT_MASK_A2) << 6;
            self.precomputed_moves.knight_moves[i] |= (position & utils::BIT_MASK_B2) >> 6;
            self.precomputed_moves.knight_moves[i] |= (position & utils::BIT_MASK_A2) >> 10;
            self.precomputed_moves.knight_moves[i] |= (position & utils::BIT_MASK_B) >> 15;
            self.precomputed_moves.knight_moves[i] |= (position & utils::BIT_MASK_A) >> 17;


            // King
            self.precomputed_moves.king_moves[i] |= (position & utils::BIT_MASK_B) << 9;
            self.precomputed_moves.king_moves[i] |= position << 8;
            self.precomputed_moves.king_moves[i] |= (position & utils::BIT_MASK_A) << 7;
            self.precomputed_moves.king_moves[i] |= (position & utils::BIT_MASK_B) << 1;
            self.precomputed_moves.king_moves[i] |= (position & utils::BIT_MASK_A) >> 1;
            self.precomputed_moves.king_moves[i] |= (position & utils::BIT_MASK_B) >> 7;
            self.precomputed_moves.king_moves[i] |= position >> 8;
            self.precomputed_moves.king_moves[i] |= (position & utils::BIT_MASK_A) >> 9;
        }
    }

    pub fn place_piece(&mut self, piece: Piece, square: Square) {
        self.bitboards[piece as usize] |= 1u64 << square;
        self.mailbox[square as usize] = Some(piece);
        self.occupied_squares[piece.is_white() as usize] |= 1u64 << square;
    }

    pub fn remove_piece(&mut self, piece: Piece, square: Square) {
        self.bitboards[piece as usize] &= !(1u64 << square);
        self.mailbox[square as usize] = None;
        self.occupied_squares[piece.is_white() as usize] &= !(1u64 << square);
    }
}
