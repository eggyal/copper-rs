use super::{
    encoding::{
        compound::LowerableCompoundExt,
        type_list::{Alt, Cons, End, Nil},
        Lowerable, Lowers,
    },
    format::*,
    Ros2List, Ros2Msg,
};
use bincode::{enc::Encoder, error::EncodeError, Encode};

impl<I: Clone + IntoIterator<Item: Lowerable>, const LEN: usize> Encode for StaticArray<I, LEN>
where
    Ros2Msg: Lowers<I::Item>,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        for element in self.0.clone() {
            Ros2Msg::lower(&element).encode(encoder)?;
        }
        Ok(())
    }
}
impl<I: Clone + IntoIterator<Item: Lowerable>, const MAX: usize> Encode for BoundedArray<I, MAX>
where
    Ros2Msg: Lowers<I::Item>,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        for element in self.iter.clone() {
            Ros2Msg::lower(&element).encode(encoder)?;
        }
        Ok(())
    }
}
impl<I: Clone + IntoIterator<Item: Lowerable>> Encode for UnboundedArray<I>
where
    Ros2Msg: Lowers<I::Item>,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        for element in self.0.clone() {
            Ros2Msg::lower(&element).encode(encoder)?;
        }
        Ok(())
    }
}

impl<M: LowerableCompoundExt<Ros2Msg>> Encode for MessageType<'_, M> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        if let Some(discriminant) = self.0.discriminant() {
            discriminant.encode(encoder)?;
        }
        self.0.as_encodable_content().encode(encoder)
    }
}

impl<Head: Lowerable, Tail: Encode> Encode for Ros2List<Cons<Head, Tail>>
where
    Ros2Msg: Lowers<Head>,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Ros2Msg::lower(&self.0.head).encode(encoder)?;
        self.0.tail.encode(encoder)
    }
}
impl Encode for Ros2List<Nil> {
    fn encode<E: Encoder>(&self, _: &mut E) -> Result<(), EncodeError> {
        Ok(())
    }
}

impl<A: Encode, B: Encode> Encode for Ros2List<Alt<A, B>> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match &self.0 {
            Alt::Head(a) => a.encode(encoder),
            Alt::Tail(b) => b.encode(encoder),
        }
    }
}
impl Encode for Ros2List<End> {
    fn encode<E: Encoder>(&self, _: &mut E) -> Result<(), EncodeError> {
        unreachable!()
    }
}
