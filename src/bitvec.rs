use std::{
    array::from_fn,
    fmt::{Binary, Debug},
};

use crate::bitset::Bitset;

#[derive(Clone, Copy)]
pub struct Bitvec<const N: usize, T: Bitset> {
    bitsets: [T; N],
}

impl<const N: usize, T: Bitset> Bitvec<N, T> {
    pub fn new(bitsets: [T; N]) -> Bitvec<N, T> {
        Bitvec { bitsets }
    }

    pub fn slice(&self) -> &[T; N] {
        &self.bitsets
    }
}

impl<const N: usize, T: Bitset + Binary> Debug for Bitvec<N, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bitvector[ ")?;
        for i in (0..N).rev() {
            write!(f, "{:0>width$b} ", self.bitsets[i], width = T::length())?;
        }
        write!(f, "]")
    }
}

impl<const N: usize, T: Bitset + Binary> Bitset for Bitvec<N, T> {
    fn ones() -> Self {
        Bitvec::new([T::ones(); N])
    }

    fn zeroes() -> Self {
        Bitvec::new([T::zeroes(); N])
    }

    fn and(&self, other: Self) -> Self {
        Bitvec::new(from_fn(|i| self.bitsets[i].and(other.bitsets[i])))
    }

    fn nand(&self, other: Self) -> Self {
        Bitvec::new(from_fn(|i| self.bitsets[i].nand(other.bitsets[i])))
    }

    fn shift_left(&self, amount: usize) -> Self {
        let mut clone = self.clone();
        clone.shift_left_mut(amount);
        return clone;
    }

    fn shift_right(&self, amount: usize) -> Self {
        let mut clone = self.clone();
        clone.shift_right_mut(amount);
        return clone;
    }

    fn or(&self, other: Self) -> Self {
        Bitvec::new(from_fn(|i| self.bitsets[i].or(other.bitsets[i])))
    }

    fn xor(&self, other: Self) -> Self {
        Bitvec::new(from_fn(|i| self.bitsets[i].xor(other.bitsets[i])))
    }

    fn not(&self) -> Self {
        Bitvec::new(from_fn(|i| self.bitsets[i].not()))
    }

    fn nor(&self, other: Self) -> Self {
        Bitvec::new(from_fn(|i| self.bitsets[i].nor(other.bitsets[i])))
    }

    fn and_mut(&mut self, other: &Self) {
        for i in 0..N {
            self.bitsets[i].and_mut(&other.bitsets[i]);
        }
    }

    fn nand_mut(&mut self, other: &Self) {
        for i in 0..N {
            self.bitsets[i].nand_mut(&other.bitsets[i]);
        }
    }

    fn shift_left_mut(&mut self, amount: usize) {
        let q = amount / T::length();
        let r = amount % T::length();
        for idx in (0..N).rev() {
            self.bitsets[idx] = if q > idx {
                T::zeroes()
            } else if idx - q == 0 {
                self.bitsets[idx - q].shift_left(r)
            } else {
                self.bitsets[idx - q]
                    .shift_left(r)
                    .or(self.bitsets[idx - q - 1].shift_right(T::length() - r))
            };
        }
    }

    fn shift_right_mut(&mut self, amount: usize) {
        let q = amount / T::length();
        let r = amount % T::length();
        for idx in 0..N {
            self.bitsets[idx] = if N <= idx + q {
                T::zeroes()
            } else if idx + q == N - 1 {
                self.bitsets[idx + q].shift_right(r)
            } else {
                self.bitsets[idx + q]
                    .shift_right(r)
                    .or(self.bitsets[idx + q + 1].shift_left(T::length() - r))
            };
        }
    }

    fn or_mut(&mut self, other: &Self) {
        for i in 0..N {
            self.bitsets[i].or_mut(&other.bitsets[i]);
        }
    }

    fn xor_mut(&mut self, other: &Self) {
        for i in 0..N {
            self.bitsets[i].xor_mut(&other.bitsets[i]);
        }
    }

    fn not_mut(&mut self) {
        for i in 0..N {
            self.bitsets[i].not_mut();
        }
    }

    fn nor_mut(&mut self, other: &Self) {
        for i in 0..N {
            self.bitsets[i].nor_mut(&other.bitsets[i]);
        }
    }

    fn set(&mut self, field: usize, flag: bool) {
        let q = field / T::length();
        let r = field % T::length();
        self.bitsets[q].set(r, flag);
    }

    fn get(&self, field: usize) -> bool {
        let q = field / T::length();
        let r = field % T::length();
        self.bitsets[q].get(r)
    }

    fn length() -> usize {
        N * T::length()
    }
}

pub type Bitvec512 = Bitvec<8, u64>;
