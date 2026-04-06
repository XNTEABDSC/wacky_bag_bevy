use std::marker::PhantomData;

use bevy::{app::FixedPostUpdate};
use derive_more::*;
use nalgebra::{RealField, SVector, Scalar};
use num_traits::Zero;
// use wacky_bag_fixed::vec_fix::VecFix;

use crate::stat_component::determining_apply_changes::determining_apply_changes;



#[derive(Into,Deref,DerefMut,Add,AddAssign,Mul,MulAssign,Clone, Copy)]
pub struct Position<Num,const DIM:usize>(pub SVector<Num,DIM>);

impl<Num:Clone+Zero+Scalar,const DIM:usize> Default for Position<Num,DIM> {
	fn default() -> Self {
		Self(SVector::from_fn(|_,_|Num::zero()))
	}
}
pub struct Plugin<Num,const DIM:usize>(PhantomData<Num>);

impl<Num: Default, const DIM: usize> Default for Plugin<Num, DIM> {
    fn default() -> Self {
		Self(Default::default())
	}
}

impl<Num:RealField+Clone,const DIM:usize> bevy::app::Plugin for Plugin<Num,DIM> {
	fn build(&self, app: &mut bevy::app::App) {
		app.add_systems(FixedPostUpdate, determining_apply_changes::<Position<Num,DIM>>);
	}
}