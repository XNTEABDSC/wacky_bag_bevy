use bevy::ecs::bundle::Bundle;
use num_traits::Zero;

use crate::stat_component::{change::Change, determining::Determining, stat::Stat};

#[derive(Bundle)]
pub struct StatBundle<T:Send+Sync+'static>{
    pub stat:Stat<T>,
    pub change:Change<T>,
}

impl<T> Default for StatBundle<T>
	where T:Send+Sync+'static+Zero
{
	fn default() -> Self {
		Self { stat: Default::default(), change: Default::default() }
	}
}

impl<T> StatBundle<T>
    where T:Send+Sync+'static+Zero
{
    pub fn new(stat:T)->Self{
        Self { stat: Stat(stat), change: Default::default()}
    }
}

#[derive(Bundle)]
pub struct DeterminingStatBundle<T:Send+Sync+'static>{
    pub stat:Stat<T>,
    pub change:Change<T>,
    pub determining:Determining<T>,
}

impl<T: Send + Sync + 'static + Zero> Default for DeterminingStatBundle<T> {
    fn default() -> Self {
		Self { stat: Default::default(), change: Default::default(), determining: Default::default() }
	}
}

impl<T> DeterminingStatBundle<T>
    where T:Send+Sync+'static+Zero
{
    pub fn new(stat:T)->Self{
        Self { stat: Stat(stat), change: Default::default(), determining: Default::default() }
    }
}