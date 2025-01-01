use super::{
    encoding::{Alt, ComplexTypeExt, Cons, End, NameableType, Nil, Encodes},
    format::*,
    Ros2,
    Ros2Msg,
};
use bincode::{enc::Encoder, error::EncodeError, Encode};
use std::borrow::Borrow;

impl<T: NameableType, const N: usize> Encode for StaticArray<'_, T, N> where Ros2Msg: Encodes<[T]> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Ros2Msg::encodable(self.0.as_slice()).encode(encoder)
    }
}

impl<T: NameableType, const N: usize> Encode for BoundedArray<'_, T, N> where Ros2Msg: Encodes<[T]> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Ros2Msg::encodable(&self.data).encode(encoder)
    }
}

impl<T: NameableType, I: Clone + Iterator<Item: Borrow<T>>> Encode for UnboundedArray<'_, I, T> where Ros2Msg: Encodes<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        for element in self.0.clone() {
            Ros2Msg::encodable(element.borrow()).encode(encoder)?;
        }
        Ok(())
    }
}

// impl<Head: Value<Ros2Msg>, Tail: Value<Ros2Msg>> Encode for Cons<Head, Tail> {
//     fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
//         self.head.encodable().encode(encoder)?;
//         self.tail.encodable().encode(encoder)
//     }
// }

impl<M: ComplexTypeExt<Ros2Msg>> Encode for MessageType<'_, M> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        if let Some(discriminant) = self.0.discriminant() {
            discriminant.encode(encoder)?;
        }
        self.0.as_encodable_content().encode(encoder)
    }
}

impl<Head: NameableType, Tail: Encode> Encode for Ros2<Cons<Head, Tail>> where Ros2Msg: Encodes<Head> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Ros2Msg::encodable(&self.0.head).encode(encoder)?;
        self.0.tail.encode(encoder)
    }
}
impl Encode for Ros2<Nil> {
    fn encode<E: Encoder>(&self, _: &mut E) -> Result<(), EncodeError> {
        Ok(())
    }
}

impl<A: Encode, B: Encode> Encode for Ros2<Alt<A, B>> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match &self.0 {
            Alt::Head(a) => a.encode(encoder),
            Alt::Tail(b) => b.encode(encoder),
        }
    }
}
impl Encode for Ros2<End> {
    fn encode<E: Encoder>(&self, _: &mut E) -> Result<(), EncodeError> {
        unreachable!()
    }
}
