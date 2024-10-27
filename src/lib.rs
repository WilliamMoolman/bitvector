mod bitset;
mod bitvec;

pub use bitset::*;
pub use bitvec::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u64_operations() {
        let op1 = 2456u64;
        let op2 = !1234u64;
        assert_eq!(op1.and(op2), op1 & op2);
        assert_eq!(op1.or(op2), op1 | op2);
        assert_eq!(op1.xor(op2), op1 ^ op2);
        assert_eq!(op1.nand(op2), !(op1 & op2));
        assert_eq!(op1.nor(op2), !(op1 | op2));
        assert_eq!(op1.not(), !op1);
        assert_eq!(op1.shift_left(6), op1 << 6);
        assert_eq!(op1.shift_right(6), op1 >> 6);
    }

    #[test]
    fn test_u64_mut_operations() {
        let mut op1a = 2456u64;
        let mut op1b = 2456u64;
        let op2 = !1234u64;

        op1a.and_mut(&op2);
        op1b &= op2;
        op1a.or_mut(&op2);
        op1b |= op2;
        op1a.xor_mut(&op2);
        op1b ^= op2;
        op1a.nand_mut(&op2);
        op1b = !(op1b & op2);
        op1a.nor_mut(&op2);
        op1b = !(op1b | op2);
        op1a.not_mut();
        op1b = !op1b;
        assert_eq!(op1a, op1b);
        op1a.shift_left_mut(6);
        op1b <<= 6;
        assert_eq!(op1a, op1b);
        op1a.shift_right_mut(10);
        op1b >>= 10;
        assert_eq!(op1a, op1b);
    }

    #[test]
    fn test_bitvec() {
        let op1 = 2456u32;
        let op2 = !1234u32;
        let int1 = (op1 as u64) << 32 | (op2 as u64);
        let int2 = (op2 as u64) << 32 | (op1 as u64);
        let bitvec1 = Bitvec::<2, u32>::new([op2, op1]);
        let bitvec2 = Bitvec::<2, u32>::new([op1, op2]);

        fn to_u64(bv: Bitvec<2, u32>) -> u64 {
            let [a, b] = bv.slice();
            (*b as u64) << 32 | (*a as u64)
        }

        assert_eq!(to_u64(bitvec1.and(bitvec2)), int1.and(int2));
        assert_eq!(to_u64(bitvec1.or(bitvec2)), int1.or(int2));
        assert_eq!(to_u64(bitvec1.xor(bitvec2)), int1.xor(int2));
        assert_eq!(to_u64(bitvec1.nand(bitvec2)), int1.nand(int2));
        assert_eq!(to_u64(bitvec1.nor(bitvec2)), int1.nor(int2));

        assert_eq!(to_u64(bitvec1.shift_left(5)), int1.shift_left(5));
        assert_eq!(to_u64(bitvec1.shift_right(6)), int1.shift_right(6));

        assert_eq!(to_u64(bitvec2.shift_left(5)), int2.shift_left(5));
        assert_eq!(to_u64(bitvec2.shift_right(6)), int2.shift_right(6));
    }
}
