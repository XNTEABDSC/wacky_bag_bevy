use bevy::{ecs::{query::{QueryData, QueryFilter}, system::Query}, utils::Parallel};



type AsRefQueryParam<'w,'s,T>=<<T as QueryData>::ReadOnly as QueryData>::Item<'w,'s>;

pub trait QueryParMapFold<
    TQueryData,TItem,TResult,TMapFn,TFoldFn
>
where 
    TQueryData:QueryData,
    TMapFn:Sync+Fn(AsRefQueryParam<'_,'_,TQueryData>)->TItem,
    TFoldFn:Sync+Fn(&mut TResult,TItem),
    TResult:Send+Sync+Default,
    TItem:Send+Sync,
{
    fn par_map_fold(&self,parallel:&mut Parallel<TResult>,map_fn:TMapFn,fold_fn:TFoldFn);
}

impl<'w,'s,TQueryData,TQueryFilter,TItem,TResult,TMapFn,TFoldFn> QueryParMapFold<TQueryData,TItem,TResult,TMapFn,TFoldFn> 
for Query<'w,'s, TQueryData, TQueryFilter> 
    where TQueryData:QueryData,
    TQueryFilter:QueryFilter,
    TMapFn:Sync+Fn(AsRefQueryParam<'_,'_,TQueryData>)->TItem,
    TFoldFn:Sync+Fn(&mut TResult,TItem),
    TResult:Send+Sync+Default,
    TItem:Send+Sync,
{
    fn par_map_fold(&self,parallel:&mut Parallel<TResult>,map_fn:TMapFn,fold_fn:TFoldFn) {
        self.par_iter().for_each_init(
            || parallel.borrow_local_mut(),
            |queue, query_data| {
                fold_fn(queue,map_fn(query_data))
            },
        );
    }
}