

use std::{mem, sync::Mutex};

use bevy::prelude::Component;

#[derive(Component,Debug,Default)]
pub struct Change<T>(pub Mutex<T>);

impl<T> Change<T>
{
    pub fn add_change(&self,change:T)
        where 
            // TInner:std::ops::AddAssign<TInner>,
            // T:Into<TInner> + Deref<Target = TInner> + DerefMut,
            // TInner:Into<T>,
            T:std::ops::AddAssign
    {
        // let mut a=self.0.lock().unwrap();
        // *a.deref_mut().deref_mut()+=change.into();
        *self.0.lock().unwrap()+=change;
    }
    // pub fn add_change_raw<TInner>(&self,change:TInner)
    //     where 
    //         TInner:std::ops::AddAssign<TInner>,
    //         T:Into<TInner> + Deref<Target = TInner> + DerefMut,
    //         TInner:Into<T>,
    // {
    //     let mut a=self.0.lock().unwrap();
    //     *a.deref_mut().deref_mut()+=change;
    // }
    pub fn get_and_reset(&mut self)->T
        where T:Default
    {
        let mut b=self.0.lock().unwrap();
        mem::replace(&mut b,Default::default())
    }
}

