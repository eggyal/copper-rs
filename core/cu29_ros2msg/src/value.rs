mod primitive_impls;
mod std_impls;

use super::{
    encoding::{
        self,
        compound::{CompoundTypeExt, EncodesCompound},
    },
    format::{self, MessageType},
    Ros2Msg,
};

impl<M: CompoundTypeExt<Ros2Msg>> EncodesCompound<M> for Ros2Msg {
    type CompoundFormatType<'a>
        = MessageType<'a, M>
    where
        M: 'a;
    fn complex_encodable(this: &M) -> MessageType<'_, M> {
        MessageType(this)
    }
}
