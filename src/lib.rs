use std::{
    array::from_fn,
    fmt::{Binary, Debug},
};

pub trait Bitset: Copy {
    // Constructors
    fn ones() -> Self;
    fn zeroes() -> Self;

    // Immutable operations
    fn and(&self, other: Self) -> Self;
    fn nand(&self, other: Self) -> Self;
    fn shift_left(&self, amount: usize) -> Self;
    fn shift_right(&self, amount: usize) -> Self;
    fn or(&self, other: Self) -> Self;
    fn not(&self) -> Self;
    fn nor(&self, other: Self) -> Self;

    // In place operations
    fn and_mut(&mut self, other: &Self);
    fn nand_mut(&mut self, other: &Self);
    fn shift_left_mut(&mut self, amount: usize);
    fn shift_right_mut(&mut self, amount: usize);
    fn or_mut(&mut self, other: &Self);
    fn not_mut(&mut self);
    fn nor_mut(&mut self, other: &Self);

    // Getter + Setter
    fn get(&self, field: usize) -> bool;
    fn set(&mut self, field: usize, flag: bool);
    fn length() -> usize;
}

impl Bitset for u64 {
    fn ones() -> Self {
        !0u64
    }

    fn zeroes() -> Self {
        0
    }

    fn and(&self, other: Self) -> Self {
        self & other
    }

    fn nand(&self, other: Self) -> Self {
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

    fn or(&self, other: Self) -> Self {
        self | other
    }

    fn not(&self) -> Self {
        !self
    }

    fn nor(&self, other: Self) -> Self {
        !(self | other)
    }

    fn and_mut(&mut self, other: &Self) {
        *self &= other;
    }

    fn nand_mut(&mut self, other: &Self) {
        *self = !(*self & *other);
    }

    fn shift_left_mut(&mut self, amount: usize) {
        *self = if amount > 63 { 0 } else { *self << amount };
    }

    fn shift_right_mut(&mut self, amount: usize) {
        *self = if amount > 63 { 0 } else { *self >> amount };
    }

    fn or_mut(&mut self, other: &Self) {
        *self |= *other;
    }

    fn not_mut(&mut self) {
        *self = !*self;
    }

    fn nor_mut(&mut self, other: &Self) {
        *self = !(*self | *other);
    }

    fn get(&self, field: usize) -> bool {
        (self >> field) & 1 != 0
    }

    fn set(&mut self, field: usize, flag: bool) {
        if flag {
            *self |= 1 << field;
        } else {
            *self &= !(1 << field);
        }
    }

    fn length() -> usize {
        64
    }
}

#[derive(Clone, Copy)]
pub struct Bitvec<const N: usize, T: Bitset> {
    bitsets: [T; N],
}

impl<const N: usize, T: Bitset> Bitvec<N, T> {
    fn new(bitsets: [T; N]) -> Bitvec<N, T> {
        Bitvec { bitsets }
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
                self.bitsets[idx - q].shift_right(r)
            } else {
                self.bitsets[idx - q]
                    .shift_right(r)
                    .or(self.bitsets[idx - q - 1].shift_left(T::length() - r))
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
                self.bitsets[idx + q].shift_left(r)
            } else {
                self.bitsets[idx + q]
                    .shift_left(r)
                    .or(self.bitsets[idx + q + 1].shift_right(T::length() - r))
            };
        }
    }

    fn or_mut(&mut self, other: &Self) {
        for i in 0..N {
            self.bitsets[i].or_mut(&other.bitsets[i]);
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
        let q = field / N;
        let r = field % N;
        self.bitsets[q].set(r, flag);
    }

    fn get(&self, field: usize) -> bool {
        let q = field / N;
        let r = field % N;
        self.bitsets[q].get(r)
    }

    fn length() -> usize {
        N * T::length()
    }
}

pub type Bitvec512 = Bitvec<8, u64>;
