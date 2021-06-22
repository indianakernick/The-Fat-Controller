use std::{iter::Iterator, marker::PhantomData, fmt::{Display, Debug}};

/// An iterator for the variants of an [`Enumeration`].
pub struct EnumerationIterator<E: Enumeration> {
    index: u8,
    phantom: PhantomData<E>,
}

impl<E: Enumeration> Iterator for EnumerationIterator<E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = E::from_u8(self.index);
        if ret.is_some() {
            self.index += 1;
        }
        ret
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (E::COUNT - self.index) as usize;
        (size, Some(size))
    }
}

pub trait Enumeration: Copy + Clone + Eq + PartialEq + Display + Debug {
    /// The name of the enumeration
    const NAME: &'static str;

    /// The number of variants in the enumeration
    const COUNT: u8;

    /// Get the display name of this enumeration
    fn display_name(&self) -> &'static str;

    /// Get the identifier name of this enumeration
    fn identifier_name(&self) -> &'static str;

    /// Create an instance of the enumeration from a `u8`
    fn from_u8(byte: u8) -> Option<Self>;

    /// Get an iterator over each variant of the enumeration
    fn iter() -> EnumerationIterator<Self> {
        EnumerationIterator::<Self> {
            index: 0, phantom: PhantomData
        }
    }
}

macro_rules! count {
    () => { 0 };
    ($first:tt $($rest:tt)*) => { 1 + count!($($rest)*) };
}

macro_rules! enumeration {
    ($name:ident, [$(($identifier_name:ident, $display_name:literal)),+$(,)?]) => {
        use crate::Enumeration;

        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub enum $name {
            $($identifier_name),*
        }

        impl $name {
            const DISPLAY_NAMES: [&'static str; Self::COUNT as usize] = [
                $($display_name),*
            ];

            const IDENTIFIER_NAMES: [&'static str; Self::COUNT as usize] = [
                $(stringify!($identifier_name)),*
            ];
        }

        impl Enumeration for $name {
            const NAME: &'static str = stringify!($name);
            const COUNT: u8 = count!($($identifier_name)*);

            fn display_name(&self) -> &'static str {
                Self::DISPLAY_NAMES[*self as u8 as usize]
            }

            fn identifier_name(&self) -> &'static str {
                Self::IDENTIFIER_NAMES[*self as u8 as usize]
            }

            fn from_u8(byte: u8) -> Option<Self> {
                match byte {
                    $(b if b == Self::$identifier_name as u8 => Some(Self::$identifier_name)),*,
                    _ => None,
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result {
                f.write_str(self.display_name())
            }
        }

        // derive(Debug) is very inefficient (not that it really matters)
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.identifier_name())
            }
        }
    }
}
