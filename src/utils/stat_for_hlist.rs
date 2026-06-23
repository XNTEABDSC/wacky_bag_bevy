use std::{marker::PhantomData, ops::{AddAssign, DerefMut, Neg}};

use bevy::ecs::query::With;
use frunk::Func;
use num_traits::Zero;
use wacky_bag::{utils::{default_of::default, }};
// use wacky_bag::utils::output_func::BijectiveFunc;
use wacky_bag_hlist::{new_struct_func, type_fn::{ReverseFunc, TypeFunc}};
use crate::stat_component::{change::{Change, transfer_changes}, determining::Determining, stat_apply_change::{change_apply_change, stat_apply_change}, stat::Stat};


new_struct_func!{
	pub MapToStat
	impl<T>:
	(T) <-> (Stat<T>)
	|i|Stat(i),
	|o|o.0
}

new_struct_func!{
	pub MapToChange
	impl<T>:
	(T) <-> (Change<T>)
	|i|Change(i.into())
}

new_struct_func! {
	pub MapToDetermining
	impl<T>:
	(T) <-> (Determining<T>)
	|_i|default()
}

new_struct_func! {
	pub MapToWith
	impl<T>:
	(T) <-> (With<T>)
}

pub type MapFromStat=ReverseFunc<MapToStat>;

new_struct_func!{
	pub MapFromStatRef
	impl<'a,T> {where T:'a}:
	(&'a Stat<T>) <-> (&'a T)
	|i|&i.0
}

new_struct_func!{
	pub MapFromStatMut
	impl<'a,T> {where T:'a}:
	(&'a mut Stat<T>) <-> (&'a mut T)
	|i|&mut i.0
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

#[derive(Clone, Copy,Debug)]
pub struct Select2ChangeRef<'a>(pub PhantomData<&'a ()>);
impl<'a> Default for Select2ChangeRef<'a>{
	fn default() -> Self {
		Self(Default::default())
	}
}
impl<'a,T> TypeFunc<T> for Select2ChangeRef<'a> 
	where T:'a
{
	type Output = (&'a Change<T>,&'a Change<T>);
}

new_struct_func!{
	pub HAddChange
	impl<'a,T> {where T:std::ops::AddAssign}:
	((T,&'a Change<T>)) |i|i.1.add_change(i.0)
}

new_struct_func!{
	pub HApplyChange
	impl<'a,T> {where T:std::ops::AddAssign+Zero}:
	((&'a mut Change<T>,&'a mut Stat<T>))
	|i|{stat_apply_change(i.0,i.1);}
}

new_struct_func!{
	pub HChangeGetAndReset
	impl<'a,T>{where T:std::ops::AddAssign+Zero}:
	(&'a mut Change<T>) -> (T)
	|i|i.get_and_reset()
}

new_struct_func!{
	pub HChangeAdd
	impl<'a,T> {where T:std::ops::AddAssign+Zero}:
	((T,&'a Change<T>))|i|{i.1.add_change(i.0);}
}

new_struct_func!{
	pub HStatSet
	impl<'a,T,SR> {where SR:DerefMut<Target = Stat<T>>}:
	((T,SR))
	|mut i|{
		let s=i.1.deref_mut();
		s.0=i.0;
	}
}

new_struct_func!{
	pub HChangeTransfer
	impl<'a,T,TSrc,TTar> 
	{where TTar:AddAssign<T>,
		T:Neg+Clone,
		TSrc:AddAssign< <T as Neg>::Output >}:
	((T,(&'a Change<TSrc>,&'a Change<TTar>)))
	|i|transfer_changes(i.0, i.1.0, i.1.1)
}

new_struct_func!{
	pub HChangeApplyChange
	impl<'a,T> {where T:std::ops::AddAssign+Zero}:
	((&'a mut Change<T>,&'a Change<T>))
	|i|change_apply_change(i.0,i.1)
}

#[derive(Debug,Default,Clone, Copy)]
pub struct MapTakeStatChange;
impl<'a,T> Func<&'a Stat<T>> for MapTakeStatChange
	where T:Clone
{
	type Output=Stat<T>;

	fn call(i: &'a Stat<T>) -> Self::Output {
		i.clone()
	}
}

impl<'a,T> Func<&'a Change<T>> for MapTakeStatChange
	where T:Zero
{
	type Output=Change<T>;

	fn call(_i: &'a Change<T>) -> Self::Output {
		Change::default()
	}
}