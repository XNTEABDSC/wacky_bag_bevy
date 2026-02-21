use bevy::{app::{FixedPostUpdate, FixedUpdate}, ecs::system::{Query, Res}, time::{Fixed, Time}};
use derive_more::*;
use physics_basic::num::Num;
use wacky_bag_fixed::vec_fix::VecFix;
use crate::{physics::position::Position, stat_component::{change::Change, determining_apply_changes::determining_apply_changes, stat::Stat}};

#[derive(Into,Deref,DerefMut,Add,AddAssign,Mul,MulAssign,Clone, Copy)]
pub struct Velocity<const DIM:usize>(pub VecFix<DIM>);
impl<const DIM:usize> Default for Velocity<DIM> {
	fn default() -> Self {
		Self(VecFix::from_fn(|_,_|Default::default()))
	}
}
pub struct Plugin<const DIM:usize>;

pub fn move_by_velocity<const DIM:usize>(mut query:Query<(&Change<Position<DIM>>,&Stat<Velocity<DIM>>)>,time:Res<Time<Fixed>>) 
{
    query.iter_mut().for_each(|(p,v)|{
        p.add_change(Position(v.0.0*(Num::from_num( time.delta_secs() ))));
    });
}

impl<const DIM:usize> bevy::app::Plugin for Plugin<DIM> {
	fn build(&self, app: &mut bevy::app::App) {
		app.add_systems(FixedPostUpdate, determining_apply_changes::<Velocity<DIM>>);
		app.add_systems(FixedUpdate, move_by_velocity::<DIM>);
	}
}