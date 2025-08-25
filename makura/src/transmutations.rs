use core::fmt::Debug;
use core::ops::{Add, BitOrAssign, Mul, Shl, ShlAssign};

fn shift<T, const SHIFT: usize>(bytes: &[u8]) -> T
where
    T: Default + TryFrom<usize> + BitOrAssign + TryFrom<u8> + ShlAssign + Copy,
    <T as TryFrom<usize>>::Error: Debug,
    <T as TryFrom<u8>>::Error: Debug,
{
    let size = bytes.len();
    let mut counter = size;
    let mut masked = T::default();
    let shift: T = SHIFT.try_into().unwrap();
    loop {
        masked |= bytes[size - counter].try_into().unwrap();

        counter -= 1;
        if counter == 0 {
            break;
        }

        masked <<= shift;
    }

    masked
}
