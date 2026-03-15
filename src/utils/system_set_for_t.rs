#[macro_export]
macro_rules! system_set_for_t {
    ($SystemSet:ident) => {
#[derive(bevy::prelude::SystemSet)]
pub struct $SystemSet<T>(pub std::marker::PhantomData<T>);

wacky_bag::impl_phantom!{$SystemSet}
    };
}
