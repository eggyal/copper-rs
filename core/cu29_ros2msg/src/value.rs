mod primitive_impls;
mod std_impls;

use super::{
    encoding::{
        self,
        compound::{LowerableCompoundExt, LowersCompound},
    },
    format::{self, MessageType},
    Ros2Msg,
};

impl<M: LowerableCompoundExt<Ros2Msg>> LowersCompound<M> for Ros2Msg {
    type LoweredCompound<'a>
        = MessageType<'a, M>
    where
        M: 'a;
    fn lower_compound(this: &M) -> MessageType<'_, M> {
        MessageType(this)
    }
}
