use std::{
    array::from_fn,
    fmt::{Binary, Debug},
    mem::size_of,
};

pub trait Bitset: Copy {
    fn ones() -> Self;
    fn zeroes() -> Self;
    fn and(&self, other: &Self) -> Self;
    fn nand(&self, other: &Self) -> Self;
    fn shift_left(&self, amount: usize) -> Self;
    fn shift_right(&self, amount: usize) -> Self;
    fn or(&self, other: &Self) -> Self;
    fn not(&self) -> Self;
    fn nor(&self, other: &Self) -> Self;
    fn set(&mut self, field: usize, flag: bool);
    fn get(&self, field: usize) -> bool;
}

impl Bitset for u64 {
    fn ones() -> Self {
        !0u64
    }

    fn zeroes() -> Self {
        0
    }

    fn and(&self, other: &Self) -> Self {
        self & other
    }

    fn nand(&self, other: &Self) -> Self {
        !(self & other)
    }

    fn shift_left(&self, amount: usize) -> Self {
        if amount > 63 {
            0
        } else {
            self << amount
        }
    }

    fn shift_right(&self, amount: usize) -> Self {
        if amount > 63 {
            0
        } else {
            self >> amount
        }
    }

    fn or(&self, other: &Self) -> Self {
        self | other
    }

    fn not(&self) -> Self {
        !self
    }

    fn nor(&self, other: &Self) -> Self {
        !(self | other)
    }

    fn set(&mut self, field: usize, flag: bool) {
        if flag {
            *self |= 1 << field
        } else {
            *self &= !(1 << field)
        }
    }

    fn get(&self, field: usize) -> bool {
        (self >> field) & 1 != 0
    }
}

#[derive(Clone, Copy)]
pub struct Bitvec<const N: usize, T: Bitset> {
    bitsets: [T; N],
    t_bits: usize,
}

impl<const N: usize, T: Bitset> Bitvec<N, T> {
    fn new(bitsets: [T; N]) -> Bitvec<N, T> {
        Bitvec {
            bitsets,
            t_bits: (size_of::<T>() * 8),
        }
    }
}

impl<const N: usize, T: Bitset + Binary> Debug for Bitvec<N, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bitvector[ ")?;
        for i in 0..N {
            write!(f, "{:0>64b} ", self.bitsets[i])?;
        }
        write!(f, "]")
    }
}

impl<const N: usize, T: Bitset> Bitset for Bitvec<N, T> {
    fn ones() -> Self {
        Bitvec::new([T::ones(); N])
    }

    fn zeroes() -> Self {
        Bitvec::new([T::zeroes(); N])
    }

    fn and(&self, other: &Self) -> Self {
        Bitvec::new(from_fn(|i| self.bitsets[i].and(&other.bitsets[i])))
    }

    fn nand(&self, other: &Self) -> Self {
        Bitvec::new(from_fn(|i| self.bitsets[i].nand(&other.bitsets[i])))
    }

    fn shift_left(&self, amount: usize) -> Self {
        let q = amount / self.t_bits;
        let r = amount % self.t_bits;
        let mut ret = Self::zeroes();
        for idx in (0..N).rev() {
            if q > idx {
                continue;
            }

            let other = if idx - q == 0 {
                &T::zeroes()
            } else {
                &self.bitsets[idx - q - 1].shift_left(self.t_bits - r)
            };
            ret.bitsets[idx] = self.bitsets[idx - q].shift_right(r).or(&other);
        }
        ret
    }

    fn shift_right(&self, amount: usize) -> Self {
        let q = amount / self.t_bits;
        let r = amount % self.t_bits;
        let mut ret = Self::zeroes();
        for idx in 0..N {
            if N <= idx + q {
                continue;
            }

            let other = if idx + q == N - 1 {
                &T::zeroes()
            } else {
                &self.bitsets[idx + q + 1].shift_right(self.t_bits - r)
            };
            ret.bitsets[idx] = self.bitsets[idx + q].shift_left(r).or(&other);
        }
        ret
    }

    fn or(&self, other: &Self) -> Self {
        Bitvec::new(from_fn(|i| self.bitsets[i].or(&other.bitsets[i])))
    }

    fn not(&self) -> Self {
        Bitvec::new(from_fn(|i| self.bitsets[i].not()))
    }

    fn nor(&self, other: &Self) -> Self {
        Bitvec::new(from_fn(|i| self.bitsets[i].nor(&other.bitsets[i])))
    }

    fn set(&mut self, field: usize, flag: bool) {
        let q = field / N;
        let r = field % N;
        self.bitsets[q].set(r, flag);
    }

    fn get(&self, field: usize) -> bool {
        let q = field / N;
        let r = field % N;
        self.bitsets[q].get(r)
    }
}

pub type Bitvec512 = Bitvec<8, u64>;
