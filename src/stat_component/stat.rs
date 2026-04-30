use derive_more::{From,Deref,DerefMut};


use bevy::prelude::Component;

use bevy::prelude::Reflect;
use num_traits::Zero;

#[derive( Component,From,Deref,DerefMut,Reflect,Clone,Copy)]
pub struct Stat<T>(pub T);

impl<T:Zero> Default for Stat<T> {
	fn default() -> Self {
		Self(T::zero())
	}
}