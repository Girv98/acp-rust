pub mod dumb7fill {
    use std::process::id;

    use super::super::{FILE_A, FILE_H};

    // /// "South Occluded Fill"
    // /// Includes starting square(s) but does not include threated squares
    // pub fn rook_south_occl(sq: u64, empties: u64) -> u64 {
    //     let mut gen = sq;
    //     let mut flood: u64 = gen;
    //     flood |= gen; gen = (gen >> 8) & empties;
    //     flood |= gen; gen = (gen >> 8) & empties;
    //     flood |= gen; gen = (gen >> 8) & empties;
    //     flood |= gen; gen = (gen >> 8) & empties;
    //     flood |= gen; gen = (gen >> 8) & empties;
    //     flood |= gen; gen = (gen >> 8) & empties;
    //     flood |=            (gen >> 8) & empties;
    //
    //     flood
    // }

    // pub fn rook_north_occl(sq: u64, empties: u64) -> u64 {
    //     let mut gen = sq;
    //     let mut flood: u64 = gen;
    //     flood |= gen; gen = (gen << 8) & empties;
    //     flood |= gen; gen = (gen << 8) & empties;
    //     flood |= gen; gen = (gen << 8) & empties;
    //     flood |= gen; gen = (gen << 8) & empties;
    //     flood |= gen; gen = (gen << 8) & empties;
    //     flood |= gen; gen = (gen << 8) & empties;
    //     flood |=            (gen << 8) & empties;
    //
    //     flood
    // }

    // pub fn rook_east_occl(sq: u64, empties: u64) -> u64 {
    //     let mut gen = sq;
    //     let mut flood: u64 = gen;
    //     let mut empties = empties;
    //     empties &= !FILE_A;
    //     flood |= gen; gen = (gen << 1) & empties;
    //     flood |= gen; gen = (gen << 1) & empties;
    //     flood |= gen; gen = (gen << 1) & empties;
    //     flood |= gen; gen = (gen << 1) & empties;
    //     flood |= gen; gen = (gen << 1) & empties;
    //     flood |= gen; gen = (gen << 1) & empties;
    //     flood |=            (gen << 1) & empties;
    //
    //     flood & !FILE_A
    // }

    pub fn rook_south_attacks(sq: u64, empties: u64) -> u64 {
        let mut gen = sq;
        let mut flood = gen;
        flood |= gen; gen = (gen >> 8) & empties;
        flood |= gen; gen = (gen >> 8) & empties;
        flood |= gen; gen = (gen >> 8) & empties;
        flood |= gen; gen = (gen >> 8) & empties;
        flood |= gen; gen = (gen >> 8) & empties;
        flood |=            (gen >> 8) & empties;

        flood >> 8
    }

    pub fn rook_north_attacks(sq: u64, empties: u64) -> u64 {
        let mut gen = sq;
        let mut flood = gen;

        flood |= gen; gen = (gen << 8) & empties;
        flood |= gen; gen = (gen << 8) & empties;
        flood |= gen; gen = (gen << 8) & empties;
        flood |= gen; gen = (gen << 8) & empties;
        flood |= gen; gen = (gen << 8) & empties;
        flood |=            (gen << 8) & empties;

        flood << 8
    }

    pub fn rook_east_attacks(sq: u64, empties: u64) -> u64 {
        let mut gen = sq;
        let mut flood = gen;
        let mut empties = empties;
        empties &= !FILE_A;

        flood |= gen; gen = (gen << 1) & empties;
        flood |= gen; gen = (gen << 1) & empties;
        flood |= gen; gen = (gen << 1) & empties;
        flood |= gen; gen = (gen << 1) & empties;
        flood |= gen; gen = (gen << 1) & empties;
        flood |=            (gen << 1) & empties;

        (flood << 1) & !FILE_A
    }

    pub fn rook_west_attacks(sq: u64, empties: u64) -> u64 {
        let mut gen = sq;
        let mut flood = gen;
        let mut empties = empties;
        empties &= !FILE_H;

        flood |= gen; gen = (gen >> 1) & empties;
        flood |= gen; gen = (gen >> 1) & empties;
        flood |= gen; gen = (gen >> 1) & empties;
        flood |= gen; gen = (gen >> 1) & empties;
        flood |= gen; gen = (gen >> 1) & empties;
        flood |=            (gen >> 1) & empties;

        (flood >> 1) & !FILE_H
    }

    pub fn bish_north_west_attacks(sq: u64, empties: u64) -> u64 { todo!() }
    pub fn bish_north_east_attacks(sq: u64, empties: u64) -> u64 { todo!() }
    pub fn bish_south_west_attacks(sq: u64, empties: u64) -> u64 { todo!() }
    pub fn bish_south_east_attacks(sq: u64, empties: u64) -> u64 { todo!() }

    // pub fn north_east_dumb7fill_attacks(sq: u64, empties: u64) -> u64 {
    //     let mut gen = sq;
    //     let mut flood: u64 = gen;
    //     let mut empties = empties;
    //     empties &= !FILE_A;
    // }

    #[cfg(test)]
    mod test {
        use super::*;
        use crate::core::utils::*;
        use crate::core::*;

        #[test]
        fn south() {
            let x = rook_south_attacks(RANK_7, !RANK_4);
            let mask = RANK_6 | RANK_5 | RANK_4;
            assert_eq!(mask, x, "\n{}", pp_bb(x));
        }

        #[test]
        fn north() {
            let x = rook_north_attacks(RANK_2, !RANK_7);
            let mask = RANK_3 | RANK_4 | RANK_5 | RANK_6 | RANK_7;
            assert_eq!(mask, x, "\n{}", pp_bb(x));
        }

        #[test]
        fn east() {
            let x = rook_east_attacks(FILE_B, !FILE_F);
            let mask = FILE_C | FILE_D | FILE_E | FILE_F;
            assert_eq!(mask, x, "\n{}", pp_bb(x));
        }

        #[test]
        fn west() {
            let x = rook_west_attacks(FILE_F, !FILE_B);
            let mask = FILE_B | FILE_C | FILE_D | FILE_E;
            assert_eq!(mask, x, "\n{}", pp_bb(x));
        }
    }
}
