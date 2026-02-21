#[macro_export]
macro_rules! system_set_for_t {
    ($SystemSet:ident) => {


#[derive(bevy::prelude::SystemSet)]
pub struct $SystemSet<T>(pub std::marker::PhantomData<T>);

impl<T> Copy for $SystemSet<T> {}

impl<T> std::fmt::Debug for $SystemSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple( std::any::type_name::<Self>() ).field(&self.0).finish()
    }
}

impl<T> std::hash::Hash for $SystemSet<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> std::cmp::Eq for $SystemSet<T> {}

impl<T> std::cmp::PartialEq for $SystemSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> std::clone::Clone for $SystemSet<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> std::default::Default for $SystemSet<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}


    };
}

