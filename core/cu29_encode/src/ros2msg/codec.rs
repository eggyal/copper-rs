use super::{
    encoding::{Alt, Cons, EncodableAlt, EncodableList, MessageExt, Nil, Term, Value},
    format::*,
    Ros2Msg,
};
use bincode::{enc::Encoder, error::EncodeError, Encode};
use std::borrow::Borrow;

impl<T: Value<Ros2Msg>, const N: usize> Encode for StaticArray<'_, T, N> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.0.as_slice().encodable().encode(encoder)
    }
}

impl<T: Value<Ros2Msg>, const N: usize> Encode for BoundedArray<'_, T, N> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.data.as_slice().encodable().encode(encoder)
    }
}

impl<T: Value<Ros2Msg>, I: Clone + Iterator<Item: Borrow<T>>> Encode for UnboundedArray<'_, I, T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        for element in self.0.clone() {
            element.borrow().encodable().encode(encoder)?;
        }
        Ok(())
    }
}

impl<D: Value<Ros2Msg>> Encode for Defer<D> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.0.encodable().encode(encoder)
    }
}

impl<M: MessageExt<Ros2Msg>> Encode for MessageType<'_, M> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.0.as_encodable_fields().encode(encoder)
    }
}

impl<A: Encode, B: Encode> Encode for EncodableAlt<Alt<A, B>, Ros2Msg> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match &self.0 {
            Alt::A(a) => a.encode(encoder),
            Alt::B(b) => b.encode(encoder),
        }
    }
}
impl Encode for EncodableAlt<Term, Ros2Msg> {
    fn encode<E: Encoder>(&self, _: &mut E) -> Result<(), EncodeError> {
        Ok(())
    }
}

impl<Head: Value<Ros2Msg>, Tail: Encode> Encode for EncodableList<Cons<Head, Tail>, Ros2Msg> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.0.head.encodable().encode(encoder)?;
        self.0.tail.encode(encoder)
    }
}
impl Encode for EncodableList<Nil, Ros2Msg> {
    fn encode<E: Encoder>(&self, _: &mut E) -> Result<(), EncodeError> {
        Ok(())
    }
}
