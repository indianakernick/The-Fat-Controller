use std::{iter::Iterator, marker::PhantomData, fmt::{Display, Debug}};

/// An iterator for the variants of an [`Enum`].
///
/// # Example
///
/// ```
/// use tfc::{CommandCode, Enum};
///
/// for var in CommandCode::iter() {
///    println!("{}", var.display_name());
/// }
/// ```
pub struct EnumIterator<E: Enum> {
    index: u8,
    phantom: PhantomData<E>,
}

impl<E: Enum> Iterator for EnumIterator<E> {
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

/// An enum with limited reflection capabilities.
///
/// The name of the enum itself and its variants is available. An iterator over
/// the variants is also provided. The three enums that implement this trait
/// are:
///  - [`CommandCode`](crate::CommandCode)
///  - [`Key`](crate::Key)
///  - [`MouseButton`](crate::MouseButton)
///
/// # Example
///
/// ```
/// use tfc::{Key, Enum};
///
/// assert_eq!(Key::NAME, "Key");
/// assert_eq!(Key::PlayPause.identifier_name(), "PlayPause");
/// assert_eq!(Key::PlayPause.display_name(), "Play/Pause");
/// ```
pub trait Enum: Copy + Clone + Eq + PartialEq + Display + Debug {
    /// The name of the enum.
    const NAME: &'static str;

    /// The number of variants in the enum.
    const COUNT: u8;

    /// The display name of this enum variant.
    ///
    /// This is the name that is appropriate for showing to end users. It may
    /// contain spaces or other symbols and is in Title Case. It is used by the
    /// [`Display`] implementation.
    fn display_name(&self) -> &'static str;

    /// The identifier name of this enum variant.
    ///
    /// This is the raw identifier name of the enum variant in PascalCase. It is
    /// used by the [`Debug`] implementation.
    fn identifier_name(&self) -> &'static str;

    /// Create an instance of the enum from a `u8`.
    ///
    /// `None` is returned if the given byte is out of range (i.e. `>= COUNT`).
    fn from_u8(byte: u8) -> Option<Self>;

    /// Get an iterator over the variants of the enum.
    fn iter() -> EnumIterator<Self> {
        EnumIterator::<Self> {
            index: 0, phantom: PhantomData
        }
    }
}

macro_rules! count {
    () => { 0 };
    ($first:tt $($rest:tt)*) => { 1 + count!($($rest)*) };
}

macro_rules! enumeration {
    (
        $name:ident,
        $description:literal,
        [$(($identifier_name:ident, $display_name:literal)),+$(,)?]
    ) => {
        use crate::Enum;

        #[doc = $description]
        ///
        /// This implements the [`Enum`] trait.
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

        impl Enum for $name {
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
