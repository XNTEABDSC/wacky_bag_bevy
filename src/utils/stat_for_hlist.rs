use std::marker::PhantomData;

use frunk::Func;
use wacky_bag::utils::type_fn::{OneOneMappingFunc, OneOneMappingTypeFunc, ReverseFunc, TypeFunc};
// use wacky_bag::utils::output_func::OneOneMappingFunc;

use crate::stat_component::{change::Change, determining_apply_changes::stat_apply_change, stat::Stat};


pub struct MapToStat;

impl<T> Func<T> for MapToStat {
	type Output=Stat<T>;

	fn call(i: T) -> Self::Output {
		Stat(i)
	}
}

impl<T> TypeFunc<T> for MapToStat {
	type Output=Stat<T>;
}
impl<T> OneOneMappingTypeFunc<Stat<T>> for MapToStat {
	type Input=T;
}
impl<T> OneOneMappingFunc<Stat<T>> for MapToStat {
	type Input=T;

	fn inv_call(output:Stat<T>)->Self::Input {
		output.0
	}
}

pub struct MapToChange;

impl<T> Func<T> for MapToChange {
	type Output=Change<T>;

	fn call(i: T) -> Self::Output {
		Change(i.into())
	}
}

impl<T> TypeFunc<T> for MapToChange {
	type Output=Change<T>;
}
impl<T> OneOneMappingTypeFunc<Change<T>> for MapToChange {
	type Input=T;
}

pub type MapFromStat=ReverseFunc<MapToStat>;


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

pub struct MapFromStatMut;

impl<'a,T> Func<&'a mut Stat<T>> for MapFromStatMut {
	type Output=&'a mut T;

	fn call(i: &'a mut Stat<T>) -> Self::Output {
		&mut i.0
	}
}

impl<'a,T> TypeFunc<&'a mut Stat<T>> for MapFromStatMut {
	type Output=&'a mut T;
}
impl<'a,T> OneOneMappingTypeFunc<&'a mut T> for MapFromStatMut {
	type Input=&'a mut Stat<T>;
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
pub struct HApplyChange;
impl<'a,T> Func<(&'a mut Change<T>,&'a mut Stat<T>)> for HApplyChange 
	where T:std::ops::AddAssign+Default
{
	type Output=();

	fn call(i: (&'a mut Change<T>,&'a mut Stat<T>)) -> Self::Output {
		//*i.1.0+=i.0.get_and_reset();
		stat_apply_change(i.1,i.0);
	}
}
// struct MapFromChange;

// impl<T> Func<T> for MapToChange {
// 	type Output=Change<T>;

// 	fn call(i: T) -> Self::Output {
// 		Change(i.into())
// 	}
// }