pub trait Enumeration: Clone + Copy + Default {
    fn encode(&self) -> i32;

    fn decode(value: i32) -> EnumValue<Self>
    where
        Self: Sized;
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EnumValue<E> {
    Known(E),
    Unknown(i32),
}

impl<E> EnumValue<E>
where
    E: Enumeration,
{
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
