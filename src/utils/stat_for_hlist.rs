use std::{marker::PhantomData, ops::{AddAssign, DerefMut, Neg}};

use bevy::ecs::query::With;
use frunk::Func;
use num_traits::Zero;
use wacky_bag::{new_struct_func, utils::{default_of::default, type_fn::{BijectiveFunc, BijectiveTypeFunc, ReverseFunc, TypeFunc}}};
// use wacky_bag::utils::output_func::BijectiveFunc;

use crate::stat_component::{change::{Change, transfer_changes}, determining::Determining, stat_apply_change::{change_apply_change, stat_apply_change}, stat::Stat};


new_struct_func!{
	pub MapToStat
	impl<T>:
	(T) <-> (Stat<T>)
	|i|Stat(i),
	|o|o.0
}

// #[derive(Debug,Default,Clone, Copy)]
// pub struct MapToStat;

// impl<T> Func<T> for MapToStat {
// 	type Output=Stat<T>;

// 	fn call(i: T) -> Self::Output {
// 		Stat(i)
// 	}
// }

// impl<T> TypeFunc<T> for MapToStat {
// 	type Output=Stat<T>;
// }
// impl<T> BijectiveTypeFunc<Stat<T>> for MapToStat {
// 	type Input=T;
// }
// impl<T> BijectiveFunc<Stat<T>> for MapToStat {
// 	type Input=T;

// 	fn inv_call(output:Stat<T>)->Self::Input {
// 		output.0
// 	}
// }

new_struct_func!{
	pub MapToChange
	impl<T>:
	(T) <-> (Change<T>)
	|i|Change(i.into())
}


// #[derive(Debug,Default,Clone, Copy)]
// pub struct MapToChange;

// impl<T> Func<T> for MapToChange {
// 	type Output=Change<T>;

// 	fn call(i: T) -> Self::Output {
// 		Change(i.into())
// 	}
// }

// impl<T> TypeFunc<T> for MapToChange {
// 	type Output=Change<T>;
// }
// impl<T> BijectiveTypeFunc<Change<T>> for MapToChange {
// 	type Input=T;
// }

new_struct_func! {
	pub MapToDetermining
	impl<T>:
	(T) <-> (Determining<T>)
	|_i|default()
}

// #[derive(Debug,Default,Clone, Copy)]
// pub struct MapToDetermining;

// impl<T> TypeFunc<T> for MapToDetermining {
// 	type Output=Determining<T>;
// }

// impl<T> BijectiveTypeFunc<Determining<T>> for MapToDetermining {
// 	type Input=T;
// }

// impl<T> Func<T> for MapToDetermining 
// {
// 	type Output=Determining<T>;

// 	fn call(_i: T) -> Self::Output {
// 		Determining::default()
// 	}
// }
new_struct_func! {
	pub MapToWith
	impl<T>:
	(T) <-> (With<T>)
}
// pub struct MapToWith;

// impl<T> TypeFunc<T> for MapToWith {
// 	type Output=With<T>;
// }

// impl<T> BijectiveTypeFunc<With<T>> for MapToWith {
// 	type Input=T;
// }

pub type MapFromStat=ReverseFunc<MapToStat>;

new_struct_func!{
	pub MapFromStatRef
	impl<'a,T> {where T:'a}:
	(&'a Stat<T>) <-> (&'a T)
	|i|&i.0
}

// /// `&Stat<T>` <-> `&T`
// /// `a:&Stat<T>` -> `b:&T`
// #[derive(Debug,Default,Clone, Copy)]
// pub struct MapFromStatRef;

// impl<'a,T> Func<&'a Stat<T>> for MapFromStatRef {
// 	type Output=&'a T;

// 	fn call(i: &'a Stat<T>) -> Self::Output {
// 		&i.0
// 	}
// }

// impl<'a,T> TypeFunc<&'a Stat<T>> for MapFromStatRef {
// 	type Output=&'a T;
// }
// impl<'a,T> BijectiveTypeFunc<&'a T> for MapFromStatRef {
// 	type Input=&'a Stat<T>;
// }

new_struct_func!{
	pub MapFromStatMut
	impl<'a,T> {where T:'a}:
	(&'a mut Stat<T>) <-> (&'a mut T)
	|i|&mut i.0
}

// #[derive(Debug,Default,Clone, Copy)]
// pub struct MapFromStatMut;

// impl<'a,T> Func<&'a mut Stat<T>> for MapFromStatMut {
// 	type Output=&'a mut T;

// 	fn call(i: &'a mut Stat<T>) -> Self::Output {
// 		&mut i.0
// 	}
// }

// impl<'a,T> TypeFunc<&'a mut Stat<T>> for MapFromStatMut {
// 	type Output=&'a mut T;
// }
// impl<'a,T> BijectiveTypeFunc<&'a mut T> for MapFromStatMut {
// 	type Input=&'a mut Stat<T>;
// }

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

// #[derive(Debug,Default,Clone, Copy)]
// pub struct HAddChange;
// impl<'a,T> Func<(T,&'a Change<T>)> for HAddChange 
// 	where T:std::ops::AddAssign
// {
// 	type Output=();

// 	fn call(i: (T,&'a Change<T>)) -> Self::Output {
// 		i.1.add_change(i.0);
// 	}
// }

new_struct_func!{
	pub HApplyChange
	impl<'a,T> {where T:std::ops::AddAssign+Zero}:
	((&'a mut Change<T>,&'a mut Stat<T>))
	|i|{stat_apply_change(i.0,i.1);}
}

// #[derive(Debug,Default,Clone, Copy)]
// pub struct HApplyChange;
// impl<'a,T> Func<(&'a mut Change<T>,&'a mut Stat<T>)> for HApplyChange 
// 	where T:std::ops::AddAssign+Zero
// {
// 	type Output=();

// 	fn call(i: (&'a mut Change<T>,&'a mut Stat<T>)) -> Self::Output {
// 		//*i.1.0+=i.0.get_and_reset();
// 		stat_apply_change(i.0,i.1);
// 	}
// }

new_struct_func!{
	pub HChangeGetAndReset
	impl<'a,T>{where T:std::ops::AddAssign+Zero}:
	(&'a mut Change<T>) -> (T)
	|i|i.get_and_reset()
}

// #[derive(Debug,Default,Clone, Copy)]
// pub struct HChangeGetAndReset;
// impl<'a,T> Func<&'a mut Change<T>> for HChangeGetAndReset 
// 	where T:std::ops::AddAssign+Zero
// {
// 	type Output=T;

// 	fn call(i: &'a mut Change<T>) -> Self::Output {
// 		i.get_and_reset()
// 	}
// }

new_struct_func!{
	pub HChangeAdd
	impl<'a,T> {where T:std::ops::AddAssign+Zero}:
	((T,&'a Change<T>))|i|{i.1.add_change(i.0);}
}

// #[derive(Debug,Default,Clone, Copy)]
// pub struct HChangeAdd;
// impl<'a,T> Func<(T,&'a Change<T>)> for HChangeAdd 
// 	where T:std::ops::AddAssign+Zero
// {
// 	type Output=();

// 	fn call(i: (T,&'a Change<T>)) -> Self::Output {
// 		//*i.1.0+=i.0.get_and_reset();
// 		i.1.add_change(i.0);
// 	}
// }
new_struct_func!{
	pub HStatSet
	impl<'a,T,SR> {where SR:DerefMut<Target = Stat<T>>}:
	((T,SR))
	|mut i|{
		let s=i.1.deref_mut();
		s.0=i.0;
	}
}

// #[derive(Debug,Default,Clone, Copy)]
// pub struct HStatSet;
// impl<'a,T,SR> Func<(T,SR)> for HStatSet 
// 	where
	
// 	SR:DerefMut<Target = Stat<T>>
// {
// 	type Output=();

// 	fn call(mut i: (T,SR)) -> Self::Output {
// 		//*i.1.0+=i.0.get_and_reset();
// 		// i.1.add_change(i.0);
// 		let s=i.1.deref_mut();
// 		s.0=i.0;
// 	}
// }

new_struct_func!{
	pub HChangeTransfer
	impl<'a,T,TSrc,TTar> 
	{where TTar:AddAssign<T>,
		T:Neg+Clone,
		TSrc:AddAssign< <T as Neg>::Output >}:
	((T,(&'a Change<TSrc>,&'a Change<TTar>)))
	|i|transfer_changes(i.0, i.1.0, i.1.1)
}

// #[derive(Debug,Default,Clone, Copy)]
// pub struct HChangeTransfer;
// impl<'a,T,TSrc,TTar> Func<(T,(&'a Change<TSrc>,&'a Change<TTar>))> for HChangeTransfer
// 	where TTar:AddAssign<T>,
// 		T:Neg+Clone,
// 		TSrc:AddAssign< <T as Neg>::Output >
// {
// 	type Output = ();
	
// 	fn call(i: (T,(&'a Change<TSrc>,&'a Change<TTar>))) -> Self::Output {
// 		transfer_changes(i.0, i.1.0, i.1.1);
// 	}
	
// }

new_struct_func!{
	pub HChangeApplyChange
	impl<'a,T> {where T:std::ops::AddAssign+Zero}:
	((&'a mut Change<T>,&'a Change<T>))
	|i|change_apply_change(i.0,i.1)
}

// #[derive(Debug,Default,Clone, Copy)]
// pub struct HChangeApplyChange;
// impl<'a,T> Func<(&'a mut Change<T>,&'a Change<T>)> for HChangeApplyChange 
// 	where T:std::ops::AddAssign+Zero
// {
// 	type Output=();

// 	fn call(i: (&'a mut Change<T>,&'a Change<T>)) -> Self::Output {
// 		//*i.1.0+=i.0.get_and_reset();
// 		change_apply_change(i.0,i.1);
// 	}
// }

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


// struct MapFromChange;

// impl<T> Func<T> for MapToChange {
// 	type Output=Change<T>;

// 	fn call(i: T) -> Self::Output {
// 		Change(i.into())
// 	}
// }