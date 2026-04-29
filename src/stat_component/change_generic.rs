

use std::{marker::PhantomData, mem, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg}, sync::Mutex};

use bevy::prelude::Component;
use num_traits::{One, Zero};

#[derive(Component,Debug)]
pub struct ChangeGeneric<T,OP>(pub Mutex<T>,pub PhantomData<OP>);

impl<T,OP> Default for ChangeGeneric<T,OP>
	where OP:AlgebraicSystem<T>
{
	fn default() -> Self {
		Self(OP::unit().into(), Default::default())
	}
}

pub trait AlgebraicSystem<T> {
	fn unit()->T;
	fn apply_assign(a:&mut T,b:T);
	fn apply(a:T,b:T)->T;
	fn inverse(a:T)->T;
}

#[derive(Debug,Default)]
pub struct AlgebraicSystemAdd;
impl<T> AlgebraicSystem<T> for AlgebraicSystemAdd
	where T:Neg<Output = T>+Add+AddAssign+Zero
{
	fn unit()->T {
		T::zero()
	}

	fn apply_assign(a:&mut T,b:T) {
		*a+=b;
	}

	fn apply(a:T,b:T)->T {
		a+b
	}

	fn inverse(a:T)->T {
		-a
	}
}

#[derive(Debug,Default)]
pub struct AlgebraicSystemMul;
impl<T> AlgebraicSystem<T> for AlgebraicSystemMul
	where T:One+Mul+MulAssign+Div<Output = T>
{
	fn unit()->T {
		T::one()
	}

	fn apply_assign(a:&mut T,b:T) {
		*a*=b;
	}

	fn apply(a:T,b:T)->T {
		a*b
	}

	fn inverse(a:T)->T {
		T::one()/a
	}
}

impl<T,OP> ChangeGeneric<T,OP>
	where OP:AlgebraicSystem<T>
{
    pub fn add_change(&self,b:T)
    {
        // let mut a=self.0.lock().unwrap();
        // *a.deref_mut().deref_mut()+=change.into();
        // *self.0.lock().unwrap()+=change;
		let mut a=self.0.lock().unwrap();
		OP::apply_assign(&mut a, b);
    }
    pub fn get_and_reset(&mut self)->T
    {
        let mut b=self.0.lock().unwrap();
        mem::replace(&mut b,OP::unit())
    }

	pub fn apply_to(&mut self, b:&mut T){
		OP::apply_assign(b, self.get_and_reset());
	}
}

pub fn transfer_changes<T,OP>(delta:T,src:&ChangeGeneric<T,OP>,tar:&ChangeGeneric<T,OP>)
	where OP:AlgebraicSystem<T>,T:Clone
{
	tar.add_change(delta.clone());
	src.add_change(OP::inverse(delta));
}