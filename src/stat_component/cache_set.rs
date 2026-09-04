use std::{mem::replace, ops::{Deref, DerefMut}, sync::Mutex};

use bevy::{ecs::{component::{Component, Mutable}, query::QueryFilter, schedule::{IntoScheduleConfigs, ScheduleConfigs}, system::{Query, ScheduleSystem}}, reflect::Reflect};
use derive_more::Display;
use frunk::{HList, HNil};
use num_traits::Zero;
use wacky_bag::utils::default_of::default;

use crate::{stat_component::stat::Stat, system::processing_system::ScheduleConfigsProcessing};

/// may be T with mutex
/// 
/// whether mutex must be used is not sure.
#[derive(Component,Debug,Reflect)]
pub struct CacheSet<T>(pub Mutex<Option<T>>);

/// [`std::sync::MutexGuard`] but [`Option<T>`] must be [`Some`]
#[derive(Debug,Display)]
pub struct CacheSetGuard<'a,T>(std::sync::MutexGuard<'a,Option<T>>);

impl<'a,T> Deref for CacheSetGuard<'a,T> {
	type Target=T;

	fn deref(&self) -> &Self::Target {
		self.0.deref().as_ref().unwrap()
	}
}

impl<'a,T> DerefMut for CacheSetGuard<'a,T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.0.deref_mut().as_mut().unwrap()
	}
}

impl<T> Default for CacheSet<T>
{
	fn default() -> Self {
		Self(Mutex::new(default()))
	}
}

impl<T> CacheSet<T> {
	/// get the value or call f to get the value
	pub fn get_or_else_set<F>(&mut self,f:F)->impl Deref<Target = T>+DerefMut
	where F:FnOnce()->T
	{
		let mut g=self.0.lock().unwrap();
		if g.deref_mut().is_none() {
			*g=Some(f());
		}
		// return std::sync::MutexGuard::map(g, |v|v.as_mut().unwrap());
		return CacheSetGuard(g);
	}

	/// get the value or call f to get the value
	/// 
	/// returns [`std::sync::MutexGuard`]
	pub fn get_or_else_set_ref<F>(&self,f:F)->impl Deref<Target = T>+DerefMut
	where F:FnOnce()->T
	{
		let mut g=self.0.lock().unwrap();
		if g.deref_mut().is_none() {
			*g=Some(f());
		}
		// return std::sync::MutexGuard::map(g, |v|v.as_mut().unwrap());
		
		return CacheSetGuard(g);
	}

	pub fn set_and_get(&mut self,v:T)->impl Deref<Target = T>+DerefMut{
		let mut g=self.0.lock().unwrap();
		// *self.0.lock().unwrap()=Some(v);
		*g=Some(v);
		return CacheSetGuard(g);
	}
	
	pub fn set_and_get_ref(&self,v:T)->impl Deref<Target = T>+DerefMut{
		let mut g=self.0.lock().unwrap();
		// *self.0.lock().unwrap()=Some(v);
		*g=Some(v);
		return CacheSetGuard(g);
	}

	pub fn take(&mut self)->Option<T>{
		replace(self.0.lock().unwrap().deref_mut(), None)
	}
	
	pub fn take_ref(&self)->Option<T>{
		replace(self.0.lock().unwrap().deref_mut(), None)
	}

}

pub fn try_get_cache<T>(a:&mut T,b:&mut CacheSet<T>)->bool{
	if let Some(v)=b.take() {
		*a=v;
		return true;
	}else {
		return false;
	}
}

/// for each [`try_get_cache`]
pub fn try_get_cache_system<T,Filter>(mut q:Query<(&mut T,&mut CacheSet<T>),Filter>)
where 
	T:Send+Sync+'static+Component<Mutability = Mutable>,
	Filter:QueryFilter+'static,
{
	q.par_iter_mut().for_each(|(mut a,mut b)|{
		try_get_cache(a.deref_mut(), b.deref_mut());
	});
}

/// [`try_get_cache_system`] with processing_config `< HList!(CacheSet<T>), HList!(T), HNil>`
/// 
/// 
pub fn set_cache_set_system_cfg<T,Filter>()->ScheduleConfigs<ScheduleSystem>
where 
	T:Send+Sync+'static+Component<Mutability = Mutable>,
	Filter:QueryFilter+'static,
{
	try_get_cache_system::<T,Filter>.into_configs()
	.config_processing::<
		HList!(CacheSet<T>),
		HList!(T),
		HNil
	>()
}