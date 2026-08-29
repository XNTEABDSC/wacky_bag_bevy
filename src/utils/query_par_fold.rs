use bevy::{ecs::{query::{QueryData, QueryFilter, ROQueryItem}, system::Query}, utils::Parallel};



// type AsRefQueryParam<'w,'s,T>=<<T as QueryData>::ReadOnly as QueryData>::Item<'w,'s>;

pub trait QueryParFold<
    TQueryData,TResult,TFoldFn
>
where 
    TQueryData:QueryData,
    TFoldFn:Sync+Fn(&mut TResult,ROQueryItem<'_,'_,TQueryData>),
    TResult:Send+Sync+Default,
{
    fn par_map_fold(&self,parallel:&mut Parallel<TResult>,fold_fn:TFoldFn);
}

impl<'w,'s,TQueryData,TQueryFilter,TResult,TFoldFn> QueryParFold<TQueryData,TResult,TFoldFn> 
for Query<'w,'s, TQueryData, TQueryFilter> 
    where TQueryData:QueryData,
    TQueryFilter:QueryFilter,
    TFoldFn:Sync+Fn(&mut TResult,ROQueryItem<'_,'_,TQueryData>),
    TResult:Send+Sync+Default,
{
    fn par_map_fold(&self,parallel:&mut Parallel<TResult>,fold_fn:TFoldFn) {
        self.par_iter().for_each_init(
            || parallel.borrow_local_mut(),
            |queue, query_data| {
                fold_fn(queue,query_data)
            },
        );
    }
}