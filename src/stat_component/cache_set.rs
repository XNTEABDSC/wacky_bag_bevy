use std::{mem::replace, ops::DerefMut, sync::Mutex};

use bevy::{ecs::{component::Component, query::QueryFilter, system::Query}, reflect::Reflect};
use num_traits::Zero;
use wacky_bag::utils::default_of::default;

use crate::stat_component::stat::Stat;

#[derive(Component,Debug,Reflect)]
pub struct CacheSet<T>(pub Mutex<Option<T>>);

impl<T> Default for CacheSet<T>
{
	fn default() -> Self {
		Self(Mutex::new(default()))
	}
}

pub fn set_cache_set<T>(a:&mut Stat<T>,b:&mut CacheSet<T>){
	if let Some(v)=replace(b.0.lock().unwrap().deref_mut(), None) {
		a.0=v;
	}
}

pub fn set_cache_set_system<T,Filter>(mut q:Query<(&mut Stat<T>,&mut CacheSet<T>),Filter>)
where 
	T:Send+Sync+'static,
	Filter:QueryFilter,
{
	q.par_iter_mut().for_each(|(mut a,mut b)|{
		set_cache_set(&mut a, &mut b);
	});
}