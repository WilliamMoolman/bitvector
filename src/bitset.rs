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
    fn xor(&self, other: Self) -> Self;
    fn not(&self) -> Self;
    fn nor(&self, other: Self) -> Self;

    // In place operations
    fn and_mut(&mut self, other: &Self);
    fn nand_mut(&mut self, other: &Self);
    fn shift_left_mut(&mut self, amount: usize);
    fn shift_right_mut(&mut self, amount: usize);
    fn or_mut(&mut self, other: &Self);
    fn xor_mut(&mut self, other: &Self);
    fn not_mut(&mut self);
    fn nor_mut(&mut self, other: &Self);

    // Getter + Setter
    fn get(&self, field: usize) -> bool;
    fn set(&mut self, field: usize, flag: bool);
    fn length() -> usize;
}

macro_rules! impl_Bitset {
    ($T:ident) => {
        impl Bitset for $T {
            fn ones() -> Self {
                !0
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
                if amount >= Self::length() {
                    0
                } else {
                    self << amount
                }
            }

            fn shift_right(&self, amount: usize) -> Self {
                if amount >= Self::length() {
                    0
                } else {
                    self >> amount
                }
            }

            fn or(&self, other: Self) -> Self {
                self | other
            }

            fn xor(&self, other: Self) -> Self {
                self ^ other
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
                *self = if amount >= Self::length() {
                    0
                } else {
                    *self << amount
                };
            }

            fn shift_right_mut(&mut self, amount: usize) {
                *self = if amount >= Self::length() {
                    0
                } else {
                    *self >> amount
                };
            }

            fn or_mut(&mut self, other: &Self) {
                *self |= *other;
            }

            fn xor_mut(&mut self, other: &Self) {
                *self ^= *other;
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
                $T::BITS as usize
            }
        }
    };
}

impl_Bitset!(u128);
impl_Bitset!(usize);
impl_Bitset!(u64);
impl_Bitset!(u32);
impl_Bitset!(u16);
impl_Bitset!(u8);

impl_Bitset!(i128);
impl_Bitset!(i64);
impl_Bitset!(i32);
impl_Bitset!(i16);
impl_Bitset!(i8);
