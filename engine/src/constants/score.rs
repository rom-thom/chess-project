



pub const INF: i32 = 1_000_000; // Making that not overflow, like ever
pub const MATE_THRESHOLD: i32 = 800_000; // Scores above this is mate


// Move-type score
pub const TT_SCORE: i32 = 1_000_000;
pub const PROMOTION_SCORE: i32 = 900_000;
pub const CAPTURE_SCORE: i32 = 800_000;
pub const KILLER_SCORE: i32 = 700_000;


// piece score
pub const KING_VAL: i32 = 100_000; // "My lord ..., he is priceless! Don't you dare put a value on him", "Well i just did bitch"
pub const QUEEN_VAL: i32 = 900;
pub const ROOK_VAL: i32 = 500;
pub const BISHOP_VAL: i32 = 330;
pub const KNIGHT_VAL: i32 = 320;
pub const PAWN_VAL: i32 = 100;
