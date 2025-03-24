use crate::piece::*;
use crate::board::*;


enum CastlingRightsFlags {
    WKingside = 1,
    WQueenside = 2,
    BKingside = 4,
    BQueenside = 8,
}


pub struct Engine {
    pub own_pieces:   [Piece; Piece::PieceCount as usize],
    pub enemy_pieces: [Piece; Piece::PieceCount as usize],

    pub board: Board,
    pub is_white_turn: bool,
    pub ply_count: u32,
}


impl Engine {
    pub fn from_fen(fen: &Vec<String>) -> Self {
        let mut engine = Self {
            own_pieces:   [Piece::WPawn; Piece::PieceCount as usize],
            enemy_pieces: [Piece::WPawn; Piece::PieceCount as usize],

            board: Board {
                precomputed_moves: PrecomputedMoves { 
                    knight_moves: [0; 64],
                    king_moves: [0; 64],
                },
                history: Vec::new(),

                castling_flags: 0,
                en_passant_square: 0,

                bitboards: [0; Piece::PieceCount as usize],
                mailbox:   [None; 64],

                attacked_squares: 0,

                occupied_squares: [0; 2],
            },

            is_white_turn: true,
            ply_count: 0,
        };

        let mut square: Square = 56;
        for c in fen[0].chars() {
            if c == '/' {
                square -= 16;
            }
            else if c.is_ascii_digit() {
                square += (c as u32 - '0' as u32) as u8;
            }
            else {
                let piece = Piece::from(c);

                engine.board.bitboards[piece as usize] |= 1u64 << square as u8;
                engine.board.mailbox[square as usize] = Some(piece);
                engine.board.occupied_squares[piece.is_white() as usize] |= 1u64 << square;

                square += 1;
            }
        }

        engine.set_color(fen[1] == "w");

        for c in fen[2].chars() {
            match c {
                'K' => engine.board.castling_flags |= CastlingRightsFlags::WKingside as u8,
                'Q' => engine.board.castling_flags |= CastlingRightsFlags::WQueenside as u8,
                'k' => engine.board.castling_flags |= CastlingRightsFlags::BKingside as u8,
                'q' => engine.board.castling_flags |= CastlingRightsFlags::BQueenside as u8,
                _ => panic!(),
            }
        }

        if fen[3] == "-" {
            engine.board.en_passant_square = 64;
        }
        else {
            //engine.board.en_passant_square = square_from_uci(fen[3]);
        }

        engine
    }

    // Should only be used when initializing with FEN
    pub fn set_color(&mut self, is_white: bool) {
        self.is_white_turn = is_white;

        if is_white {
            self.own_pieces[PieceType::Pawn as usize]   = Piece::WPawn;
            self.own_pieces[PieceType::Knight as usize] = Piece::WKnight;
            self.own_pieces[PieceType::Bishop as usize] = Piece::WBishop;
            self.own_pieces[PieceType::Rook as usize]   = Piece::WRook;
            self.own_pieces[PieceType::Queen as usize]  = Piece::WQueen;
            self.own_pieces[PieceType::King as usize]   = Piece::WKing;

            self.enemy_pieces[PieceType::Pawn as usize]   = Piece::BPawn;
            self.enemy_pieces[PieceType::Knight as usize] = Piece::BKnight;
            self.enemy_pieces[PieceType::Bishop as usize] = Piece::BBishop;
            self.enemy_pieces[PieceType::Rook as usize]   = Piece::BRook;
            self.enemy_pieces[PieceType::Queen as usize]  = Piece::BQueen;
            self.enemy_pieces[PieceType::King as usize]   = Piece::BKing;
        }
        else {
            self.own_pieces[PieceType::Pawn as usize]   = Piece::BPawn;
            self.own_pieces[PieceType::Knight as usize] = Piece::BKnight;
            self.own_pieces[PieceType::Bishop as usize] = Piece::BBishop;
            self.own_pieces[PieceType::Rook as usize]   = Piece::BRook;
            self.own_pieces[PieceType::Queen as usize]  = Piece::BQueen;
            self.own_pieces[PieceType::King as usize]   = Piece::BKing;

            self.enemy_pieces[PieceType::Pawn as usize]   = Piece::WPawn;
            self.enemy_pieces[PieceType::Knight as usize] = Piece::WKnight;
            self.enemy_pieces[PieceType::Bishop as usize] = Piece::WBishop;
            self.enemy_pieces[PieceType::Rook as usize]   = Piece::WRook;
            self.enemy_pieces[PieceType::Queen as usize]  = Piece::WQueen;
            self.enemy_pieces[PieceType::King as usize]   = Piece::WKing;
        }
    }

    pub fn flip_color(&mut self) {
        self.is_white_turn = !self.is_white_turn;

        self.enemy_pieces[PieceType::Pawn as usize]   = self.own_pieces[PieceType::Pawn as usize];
        self.enemy_pieces[PieceType::Knight as usize] = self.own_pieces[PieceType::Knight as usize];
        self.enemy_pieces[PieceType::Bishop as usize] = self.own_pieces[PieceType::Bishop as usize];
        self.enemy_pieces[PieceType::Rook as usize]   = self.own_pieces[PieceType::Rook as usize];
        self.enemy_pieces[PieceType::Queen as usize]  = self.own_pieces[PieceType::Queen as usize];
        self.enemy_pieces[PieceType::King as usize]   = self.own_pieces[PieceType::King as usize];

        self.own_pieces[PieceType::Pawn as usize]   = Piece::from(self.own_pieces[PieceType::Pawn as usize] as u8 ^ 1);
        self.own_pieces[PieceType::Knight as usize] = Piece::from(self.own_pieces[PieceType::Knight as usize] as u8 ^ 1);
        self.own_pieces[PieceType::Bishop as usize] = Piece::from(self.own_pieces[PieceType::Bishop as usize] as u8 ^ 1);
        self.own_pieces[PieceType::Rook as usize]   = Piece::from(self.own_pieces[PieceType::Rook as usize] as u8 ^ 1);
        self.own_pieces[PieceType::Queen as usize]  = Piece::from(self.own_pieces[PieceType::Queen as usize] as u8 ^ 1);
        self.own_pieces[PieceType::King as usize]   = Piece::from(self.own_pieces[PieceType::King as usize] as u8 ^ 1);
    }
}
