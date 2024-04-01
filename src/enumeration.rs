//! Definition of the `Enumeration` trait and related types.

/// An enumeration of several variants with distinct integer discriminants.
pub trait Enumeration: Clone + Copy + Default {
    /// Encodes this enumeration into an integer.
    fn encode(&self) -> i32;

    /// Decodes an integer value into an enumeration value for this enumeration.
    ///
    /// Will return `EnumValue::Known` if the value maps to a known enum variant, and
    /// `EnumValue::Unknown` otherwise.
    fn decode(value: i32) -> EnumValue<Self>
    where
        Self: Sized;
}

/// An enum value, encoding the fact that we might have gotten a value from a sender that we don't
/// yet know about.
///
/// It is usually a good idea to program defensively and handle this case explicitly.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EnumValue<E> {
    /// A known value of type `E`.
    Known(E),
    /// An unknown enum variant that isn't present on the type `E`, with the given discriminant.
    Unknown(i32),
}

impl<E> EnumValue<E>
where
    E: Enumeration,
{
    /// Converts this enum value into its raw integer discriminant representation.
    pub fn to_raw(&self) -> i32 {
        match *self {
            EnumValue::Known(v) => v.encode(),
            EnumValue::Unknown(v) => v,
        }
    }
}

impl<E> Default for EnumValue<E>
where
    E: Default,
{
    fn default() -> Self {
        Self::Known(Default::default())
    }
}
