#[repr(u8)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,

    PieceTypeCount,
}

impl From<u8> for PieceType {
    fn from(value: u8) -> Self {
        match value {
            0 => PieceType::Pawn,
            1 => PieceType::Knight,
            2 => PieceType::Bishop,
            3 => PieceType::Rook,
            4 => PieceType::Queen,
            5 => PieceType::King,
            _ => panic!(),
        }
    }
}


#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Piece {
    WPawn,    BPawn,
    WKnight,  BKnight,
    WBishop,  BBishop,
    WRook,    BRook,
    WQueen,   BQueen,
    WKing,    BKing,

    PieceCount,
}

impl Piece {
    pub fn is_white(&self) -> bool {
        (*self as u8 & 1) == 0
    }
}

impl From<u8> for Piece {
    fn from(value: u8) -> Self {
        match value {
            0  => Piece::WPawn,
            1  => Piece::BPawn,
            2  => Piece::WKnight,
            3  => Piece::BKnight,
            4  => Piece::WBishop,
            5  => Piece::BBishop,
            6  => Piece::WRook,
            7  => Piece::BRook,
            8  => Piece::WQueen,
            9  => Piece::BQueen,
            10 => Piece::WKing,
            11 => Piece::BKing,
            _ => panic!(),
        }
    }
}

impl From<char> for Piece {
    fn from(value: char) -> Self {
        match value {
            'P'  => Piece::WPawn,
            'p'  => Piece::BPawn,
            'N'  => Piece::WKnight,
            'n'  => Piece::BKnight,
            'B'  => Piece::WBishop,
            'b'  => Piece::BBishop,
            'R'  => Piece::WRook,
            'r'  => Piece::BRook,
            'Q'  => Piece::WQueen,
            'q'  => Piece::BQueen,
            'K' => Piece::WKing,
            'k' => Piece::BKing,
            _ => panic!(),
        }
    }
}


const PIECE_VALUES: [i32; 6] = [100, 300, 300, 500, 900, 31415926];


enum MoveFlag {
    None            = 0,
    Castling        = 1,
    PromotionKnight = 2,
    PromotionBishop = 4,
    PromotionRook   = 8,
    PromotionQueen  = 16,
}


pub fn get_piece_type(piece: Piece) -> PieceType {
    PieceType::from(piece as u8 >> 1)
}
