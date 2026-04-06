use std::marker::PhantomData;

use bevy::{app::{FixedPostUpdate, FixedUpdate}, ecs::system::{Query, Res}, time::{Fixed, Time}};
use derive_more::*;
use nalgebra::{RealField, SVector, Scalar};
use num_traits::Zero;
use physics_basic::stats::TimePass;

use crate::{physics::position::Position, stat_component::{change::Change, determining_apply_changes::determining_apply_changes, stat::Stat}};

#[derive(Into,Deref,DerefMut,Add,AddAssign,Mul,MulAssign,Clone, Copy)]
pub struct Velocity<Num,const DIM:usize>(pub SVector<Num,DIM>);
impl<Num:Zero+Scalar,const DIM:usize> Default for Velocity<Num,DIM> {
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


pub fn move_by_velocity<Num:Sync+Send+'static+Copy+RealField,const DIM:usize>(mut query:Query<(&Change<Position<Num,DIM>>,&Stat<Velocity<Num,DIM>>,&Stat<TimePass<Num>>)>,time:Res<Time<Fixed>>) 
{
    query.iter_mut().for_each(|(p,v,dt)|{
        p.add_change(Position(v.0.0*dt.0.0));
    });
}

impl<Num,const DIM:usize> bevy::app::Plugin for Plugin<Num,DIM> 
	where Num:Sync+Send+'static+Copy+RealField
{
	fn build(&self, app: &mut bevy::app::App) {
		app.add_systems(FixedPostUpdate, determining_apply_changes::<Velocity<Num,DIM>>);
		app.add_systems(FixedUpdate, move_by_velocity::<Num,DIM>);
	}
}