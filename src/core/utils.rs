///  Unused functions

use super::*;

fn bb_to_str(bb: u64) -> String {
    format!("{:0>64}", format!("{bb:b}")) // convert to string add leading zeros if necessary
}

fn rank_to_str(r: u8) -> String {
    format!("{:0>8}", format!("{r:b}")) // convert to string add leading zeros if necessary
}

fn rev_rank(mut b: u8) -> u8 {
    b = (b & 0xF0) >> 4 | (b & 0x0F) << 4;
    b = (b & 0xCC) >> 2 | (b & 0x33) << 2;
    b = (b & 0xAA) >> 1 | (b & 0x55) << 1;

    b
}

pub fn pp_bb(bb: u64) -> String {
    let ranks = [
        rank_to_str(rev_rank(((bb & RANK_8) >> (64-8))   as u8)),
        rank_to_str(rev_rank(((bb & RANK_7) >> (64-8*2)) as u8)),
        rank_to_str(rev_rank(((bb & RANK_6) >> (64-8*3)) as u8)),
        rank_to_str(rev_rank(((bb & RANK_5) >> (64-8*4)) as u8)),
        rank_to_str(rev_rank(((bb & RANK_4) >> (64-8*5)) as u8)),
        rank_to_str(rev_rank(((bb & RANK_3) >> (64-8*6)) as u8)),
        rank_to_str(rev_rank(((bb & RANK_2) >> (64-8*7)) as u8)),
        rank_to_str(rev_rank((bb & RANK_1) as u8))
    ];

    let mut output = String::new();
    let mut letters = vec!['7', '6', '5', '4', '3', '2', '1'].into_iter();
    output.push('8');

    for r in ranks {
        let spaced_r: String = r.chars().flat_map(|c| {
            Some(' ').into_iter().chain(std::iter::once(if c == '1' {'1'} else {'·'}))
        }).collect();

        output.push_str(&spaced_r);
        if let Some(lt) = letters.next() {
            output.push_str(&format!("{}{}", "\n", lt))
        }
    }

    output.push_str("\n~ A B C D E F G H");

    output
}

// pub fn pretty_print_bb(bb: u64) {
//     let bb_str = bb_to_str(bb);
//
//     println!("{bb_str}");
//
//     let mut asdf = String::new();
//     for (n, c) in bb_str.chars().enumerate() {
//         if n != 0 && n % 8 == 0 {
//             asdf.push_str("\n");
//         }
//         asdf.push(c);
//     }
//
//     let x = asdf.split("\n");
//
//     let mut letters = vec!["7 ", "6 ", "5 ", "4 ", "3 ", "2 ", "1 "].into_iter();
//
//     let mut s = "8 ".to_string();
//
//     for st in x {
//
//         let zxc = st.chars().rev().enumerate()
//             .flat_map(|(i, c)| {
//                 (i != 0).then(|| ' ').into_iter().chain(std::iter::once(c))
//             }).collect::<String>();
//
//
//         s.push_str(&zxc);
//         if let Some(lttr) = letters.next() {
//             s.push_str(&format!("{}{}", "\n", lttr))
//         }
//     }
//
//     s.push_str("\n  A B C D E F G H");
//
//     println!("{s}")
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn rev_rank_test() {
        assert_eq!(0b11110000, rev_rank(0b00001111));
        assert_eq!(0b10100100, rev_rank(0b00100101));
        assert_eq!(0b10101010, rev_rank(0b01010101));
        assert_eq!(0b01010101, rev_rank(0b10101010));
    }


}