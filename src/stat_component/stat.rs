use derive_more::{From,Deref,DerefMut};


use bevy::prelude::Component;

use bevy::prelude::Reflect;

#[derive( Default,Component,From,Deref,DerefMut,Reflect)]
pub struct Stat<T>(pub T);

