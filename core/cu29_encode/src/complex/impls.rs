//! Implementations of [`Message`] for references, that delegate to the referent.

use super::{Desc, Message, MessageFields};

impl<Msg: Message> Message for &Msg {
    const NAME: &'static str = Msg::NAME;
}
impl<'this, Msg: MessageFields<'this>> MessageFields<'this> for &Msg {
    type Fields = Msg::Fields;
    const DESCRIPTOR: Desc<Self::Fields> = Msg::DESCRIPTOR;
    fn as_fields(&'this self) -> Self::Fields {
        (**self).as_fields()
    }
}

impl<Msg: Message> Message for &mut Msg {
    const NAME: &'static str = Msg::NAME;
}
impl<'this, Msg: MessageFields<'this>> MessageFields<'this> for &mut Msg {
    type Fields = Msg::Fields;
    const DESCRIPTOR: Desc<Self::Fields> = Msg::DESCRIPTOR;
    fn as_fields(&'this self) -> Self::Fields {
        (**self).as_fields()
    }
}
