use std::ops::AddAssign;

use bevy::reflect::Reflect;

use crate::stat_component::{change::Change, stat::Stat};
#[derive(Reflect)]
pub struct StatPack<T>(Stat<T>,Change<T>);

impl<T: num_traits::Zero> Default for StatPack<T> {
    fn default() -> Self {
		Self(Default::default(), Default::default())
	}
}

impl<T> StatPack<T> 
    where T:AddAssign+num_traits::Zero
{
    pub fn new(v:T)->Self{Self(Stat(v),Change::default())}
    pub fn get(&self)->&T{&self.0}
    pub fn add_change(&self,change:T){self.1.add_change(change);}
    pub fn apply_change(&mut self){
        self.0.0+=self.1.get_and_reset();
    }
}