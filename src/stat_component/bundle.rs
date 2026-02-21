use bevy::ecs::bundle::Bundle;

use crate::stat_component::{change::Change, determining::Determining, stat::Stat};

#[derive(Bundle,Default)]
pub struct StatBundle<T:Send+Sync+'static>{
    pub stat:Stat<T>,
    pub change:Change<T>,
}


impl<T> StatBundle<T>
    where T:Send+Sync+'static+Default
{
    pub fn new(stat:T)->Self{
        Self { stat: Stat(stat), change: Default::default()}
    }
}

#[derive(Bundle,Default)]
pub struct DeterminingStatBundle<T:Send+Sync+'static>{
    pub stat:Stat<T>,
    pub change:Change<T>,
    pub determining:Determining<T>,
}

impl<T> DeterminingStatBundle<T>
    where T:Send+Sync+'static+Default
{
    pub fn new(stat:T)->Self{
        Self { stat: Stat(stat), change: Default::default(), determining: Default::default() }
    }
}