use bevy::{app::FixedPostUpdate};
use derive_more::*;
use wacky_bag_fixed::vec_fix::VecFix;

use crate::stat_component::determining_apply_changes::determining_apply_changes;



#[derive(Into,Deref,DerefMut,Add,AddAssign,Mul,MulAssign,Clone, Copy)]
pub struct Position<const DIM:usize>(pub VecFix<DIM>);

impl<const DIM:usize> Default for Position<DIM> {
	fn default() -> Self {
		Self(VecFix::from_fn(|_,_|Default::default()))
	}
}

pub struct Plugin<const DIM:usize>;

impl<const DIM:usize> bevy::app::Plugin for Plugin<DIM> {
	fn build(&self, app: &mut bevy::app::App) {
		app.add_systems(FixedPostUpdate, determining_apply_changes::<Position<DIM>>);
	}
}