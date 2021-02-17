use std::num;

// This seems like something that should be in the standard library.

pub trait NonZero {
    type Type;
}

impl NonZero for u8 {
    type Type = num::NonZeroU8;
}

impl NonZero for u16 {
    type Type = num::NonZeroU16;
}

impl NonZero for u32 {
    type Type = num::NonZeroU32;
}

impl NonZero for u64 {
    type Type = num::NonZeroU64;
}

impl NonZero for u128 {
    type Type = num::NonZeroU128;
}

impl NonZero for usize {
    type Type = num::NonZeroUsize;
}

impl NonZero for i8 {
    type Type = num::NonZeroI8;
}

impl NonZero for i16 {
    type Type = num::NonZeroI16;
}

impl NonZero for i32 {
    type Type = num::NonZeroI32;
}

impl NonZero for i64 {
    type Type = num::NonZeroI64;
}

impl NonZero for i128 {
    type Type = num::NonZeroI128;
}

impl NonZero for isize {
    type Type = num::NonZeroIsize;
}
