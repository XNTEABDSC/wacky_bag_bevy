use std::marker::PhantomData;

use frunk::Func;
use wacky_bag::utils::type_fn::{OneOneMappingTypeFunc, TypeFunc};
// use wacky_bag::utils::output_func::OneOneMappingFunc;

use crate::stat_component::{change::Change, stat::Stat};


pub struct MapToStat;

impl<T> Func<T> for MapToStat {
	type Output=Stat<T>;

	fn call(i: T) -> Self::Output {
		Stat(i)
	}
}

pub struct MapToChange;

impl<T> Func<T> for MapToChange {
	type Output=Change<T>;

	fn call(i: T) -> Self::Output {
		Change(i.into())
	}
}

pub struct MapFromStat;


impl<T> Func<Stat<T>> for MapFromStat {
	type Output=T;

	fn call(i:Stat<T>) -> Self::Output {
		i.0
	}
}

impl<T> TypeFunc<Stat<T>> for MapFromStat {
	// type Output=T;
	
	type Output=T;
}
impl<T> OneOneMappingTypeFunc<T> for MapFromStat {
	// type Output=T;
	
	type Input=Stat<T>;
}

pub struct MapFromStatRef;

impl<'a,T> Func<&'a Stat<T>> for MapFromStatRef {
	type Output=&'a T;

	fn call(i: &'a Stat<T>) -> Self::Output {
		&i.0
	}
}

impl<'a,T> TypeFunc<&'a Stat<T>> for MapFromStatRef {
	type Output=&'a T;
}
impl<'a,T> OneOneMappingTypeFunc<&'a T> for MapFromStatRef {
	type Input=&'a Stat<T>;
}

#[derive(Clone, Copy,Debug)]
pub struct SelectChangeRef<'a>(pub PhantomData<&'a ()>);
impl<'a> Default for SelectChangeRef<'a>{
	fn default() -> Self {
		Self(Default::default())
	}
}
impl<'a,T> TypeFunc<T> for SelectChangeRef<'a> 
	where T:'a
{
	type Output = &'a Change<T>;
}
pub struct HAddChange;
impl<'a,T> Func<(T,&'a Change<T>)> for HAddChange 
	where T:std::ops::AddAssign
{
	type Output=();

	fn call(i: (T,&'a Change<T>)) -> Self::Output {
		i.1.add_change(i.0);
	}
}
// struct MapFromChange;

// impl<T> Func<T> for MapToChange {
// 	type Output=Change<T>;

// 	fn call(i: T) -> Self::Output {
// 		Change(i.into())
// 	}
// }