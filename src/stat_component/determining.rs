
use std::marker::PhantomData;

use bevy::prelude::Component;

/// A marker component that marks `T` as a variable that `Pos<T>` should and should only be changed by `Change<T>` at `determining_apply_changes<T>` `FixedPostUpdate`
/// 
#[derive( Default,Component)]
pub struct Determining<T>(PhantomData<T>);