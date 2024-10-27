use bitset_rs::Bitset;
use bitset_rs::Bitvec;

fn main() {
    let bitv = Bitvec::<2, u64>::ones();
    println!("{bitv:?}");
    println!();
    for i in 0..130 {
        let b2 = bitv.shift_left(i);
        println!("{i:>3}: {b2:?}")
    }
    for i in 0..130 {
        let b2 = bitv.shift_right(i);
        println!("{i:>3}: {b2:?}")
    }
}
