use crate::board::Bitboard;


// Rank masks
    pub const RANK_1: Bitboard = Bitboard::new_const(0x0000_0000_0000_00FF);
    pub const RANK_2: Bitboard = Bitboard::new_const(0x0000_0000_0000_FF00);
    pub const RANK_3: Bitboard = Bitboard::new_const(0x0000_0000_00FF_0000);
    pub const RANK_4: Bitboard = Bitboard::new_const(0x0000_0000_FF00_0000);
    pub const RANK_5: Bitboard = Bitboard::new_const(0x0000_00FF_0000_0000);
    pub const RANK_6: Bitboard = Bitboard::new_const(0x0000_FF00_0000_0000);
    pub const RANK_7: Bitboard = Bitboard::new_const(0x00FF_0000_0000_0000);
    pub const RANK_8: Bitboard = Bitboard::new_const(0xFF00_0000_0000_0000);

// File masks
    pub const FILE_A: Bitboard = Bitboard::new_const(0x0101_0101_0101_0101);
    pub const FILE_B: Bitboard = Bitboard::new_const(0x0202_0202_0202_0202);
    pub const FILE_C: Bitboard = Bitboard::new_const(0x0404_0404_0404_0404);
    pub const FILE_D: Bitboard = Bitboard::new_const(0x0808_0808_0808_0808);
    pub const FILE_E: Bitboard = Bitboard::new_const(0x1010_1010_1010_1010);
    pub const FILE_F: Bitboard = Bitboard::new_const(0x2020_2020_2020_2020);
    pub const FILE_G: Bitboard = Bitboard::new_const(0x4040_4040_4040_4040);
    pub const FILE_H: Bitboard = Bitboard::new_const(0x8080_8080_8080_8080);


// Diagonals
    // Diagonals oriented A1 → H8 (step = +9)
        pub const DIAG_A1_H8: Bitboard = Bitboard::new_const(0x8040_2010_0804_0201);
        pub const DIAG_B1_G7: Bitboard = Bitboard::new_const(0x0080_4020_1008_0402);
        pub const DIAG_C1_F6: Bitboard = Bitboard::new_const(0x0000_8040_2010_0804);
        pub const DIAG_D1_E5: Bitboard = Bitboard::new_const(0x0000_0080_4020_1008);
        pub const DIAG_E1_D4: Bitboard = Bitboard::new_const(0x0000_0000_8040_2010);
        pub const DIAG_F1_C3: Bitboard = Bitboard::new_const(0x0000_0000_0080_4020);
        pub const DIAG_G1_B2: Bitboard = Bitboard::new_const(0x0000_0000_0000_8040);
        pub const DIAG_A2_H7: Bitboard = Bitboard::new_const(0x4020_1008_0402_0100);
        pub const DIAG_A3_G6: Bitboard = Bitboard::new_const(0x2010_0804_0201_0000);
        pub const DIAG_A4_F5: Bitboard = Bitboard::new_const(0x1008_0402_0100_0000);
        pub const DIAG_A5_E4: Bitboard = Bitboard::new_const(0x0804_0201_0000_0000);
        pub const DIAG_A6_D3: Bitboard = Bitboard::new_const(0x0402_0100_0000_0000);
        pub const DIAG_A7_C2: Bitboard = Bitboard::new_const(0x0201_0000_0000_0000);

    // Diagonals oriented H1 → A8 (step = +7)
        pub const DIAG_H1_A8: Bitboard = Bitboard::new_const(0x0102_0408_1020_4080);
        pub const DIAG_G1_A7: Bitboard = Bitboard::new_const(0x0001_0204_0810_2040);
        pub const DIAG_F1_A6: Bitboard = Bitboard::new_const(0x0000_0102_0408_1020);
        pub const DIAG_E1_A5: Bitboard = Bitboard::new_const(0x0000_0001_0204_0810);
        pub const DIAG_D1_A4: Bitboard = Bitboard::new_const(0x0000_0000_0102_0408);
        pub const DIAG_C1_A3: Bitboard = Bitboard::new_const(0x0000_0000_0001_0204);
        pub const DIAG_B1_A2: Bitboard = Bitboard::new_const(0x0000_0000_0000_0102);
        pub const DIAG_H2_B8: Bitboard = Bitboard::new_const(0x0204_0810_2040_8000);
        pub const DIAG_H3_C8: Bitboard = Bitboard::new_const(0x0408_1020_4080_0000);
        pub const DIAG_H4_D8: Bitboard = Bitboard::new_const(0x0810_2040_8000_0000);
        pub const DIAG_H5_E8: Bitboard = Bitboard::new_const(0x1020_4080_0000_0000);
        pub const DIAG_H6_F8: Bitboard = Bitboard::new_const(0x2040_8000_0000_0000);
        pub const DIAG_H7_G8: Bitboard = Bitboard::new_const(0x4080_0000_0000_0000);

// Important positions
    pub const CORNERS: Bitboard = Bitboard::new_const((1 <<  0) | (1 <<  7) | (1 << 56) | (1 << 63));
    pub const CENTER_4: Bitboard = Bitboard::new_const(0x0000_0018_1800_0000); // The four central squares
    pub const BORDER: Bitboard = Bitboard::new_const(0xFF81_8181_8181_81FF); // The entire border of the board (upper, lower, and sides)
    pub const KINGS: Bitboard = Bitboard::new_const(0x1000000000000010); // only the king squares



