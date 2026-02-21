use bevy::tasks::{ComputeTaskPool, ParallelSlice, ParallelSliceMut};



pub trait ParSimpleMap<T:Sync>{
    fn par_simple_map<'a,F,R>(&'a self,f:F)->Vec<Vec<R>>
        where F:Sync+Send+Fn(&T)->R,
            R: Send + 'static,
    ;
}


impl<TCollection,T> ParSimpleMap<T> for TCollection
    where TCollection:ParallelSlice<T>,
        T:Sync
{
    fn par_simple_map<'a,F,R>(&'a self,f:F)->Vec<Vec<R>>
        where F:Sync+Send+Fn(&T)->R,
            R: Send + 'static,

    {
        self.par_splat_map(ComputeTaskPool::get(), None, move |_,ts|{
            ts.iter().map(&f).collect::<Vec<R>>()
        })
    }
}


pub trait ParSimpleMapMut<T:Sync>{
    fn par_simple_map_mut<'a,F,R>(&'a mut self,f:F)->Vec<Vec<R>>
        where F:Sync+Send+Fn(&mut T)->R,
            R: Send + 'static,
    ;
}

impl<TCollection,T> ParSimpleMapMut<T> for TCollection
    where TCollection:ParallelSliceMut<T>,
        T:Sync+Send
{
    fn par_simple_map_mut<'a,F,R>(&'a mut self,f:F)->Vec<Vec<R>>
        where F:Sync+Send+Fn(&mut T)->R,
            R: Send + 'static,

    {
        self.par_splat_map_mut(ComputeTaskPool::get(), None, move |_,ts|{
            ts.iter_mut().map(&f).collect::<Vec<R>>()
        })
    }
}