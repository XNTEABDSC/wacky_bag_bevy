use std::ops::AddAssign;

use bevy::reflect::Reflect;

use crate::stat_component::{change::Change, stat::Stat};
#[derive(Default,Reflect)]
pub struct StatPack<T>(Stat<T>,Change<T>);

impl<T> StatPack<T> 
    where T:AddAssign+Default
{
    pub fn new(v:T)->Self{Self(Stat(v),Change::default())}
    pub fn get(&self)->&T{&self.0}
    pub fn add_change(&self,change:T){self.1.add_change(change);}
    pub fn apply_change(&mut self){
        self.0.0+=self.1.get_and_reset();
    }
}