use bevy::ecs::{query::{QueryData, QueryFilter, QueryItem, ROQueryItem}, system::{Query, SystemParam, SystemParamItem}};


/// Some helper method works with a query, this helps you to merge
/// 
/// This is only used for type so `'static` is used
pub struct SystemParamWithQueryT<D,F,P>
where D:QueryData+'static,
	F:QueryFilter+'static,
	P:SystemParam+'static
{
	pub query:Query<'static,'static,D,F>,
	pub others:P
}

pub trait SystemParamWithQuery {
	type D:QueryData+'static;
	type F:QueryFilter+'static;
	type P:SystemParam+'static;
}

pub type SystemParamWithQueryQuery<'w,'s,T>=Query<'w,'s,<T as SystemParamWithQuery>::D,<T as SystemParamWithQuery>::F>;
pub type SystemParamWithQueryROItem<'w,'s,T>=(
	ROQueryItem<'w,'s,
		<T as SystemParamWithQuery>::D
	>,
	SystemParamItem<'w,'s,
		<T as SystemParamWithQuery>::P
	>
);
pub type SystemParamWithQueryItem<'w,'s,T>=(
	QueryItem<'w,'s,
		<T as SystemParamWithQuery>::D
	>,
	SystemParamItem<'w,'s,
		<T as SystemParamWithQuery>::P
	>
);pub type SystemParamWithQueryParam<'w,'s,T>=(
	SystemParamWithQueryQuery<'w,'s,
		T
	>,
	<T as SystemParamWithQuery>::P
);
impl<D,F,P> SystemParamWithQuery for SystemParamWithQueryT<D,F,P>
where D:QueryData+'static,
	F:QueryFilter+'static,
	P:SystemParam+'static
{
	type D=D;

	type F=F;

	type P=P;
}

pub trait SystemParamWithQueryMerge<B> {
	type Merge;
}

impl<D1,F1,P1,D2,F2,P2,T1,T2> SystemParamWithQueryMerge<T2> for T1
where 
	T1:SystemParamWithQuery<D=D1,F=F1,P=P1>,
	T2:SystemParamWithQuery<D=D2,F=F2,P=P2>,
	D1:QueryData+'static,
	F1:QueryFilter+'static,
	P1:SystemParam+'static,
	D2:QueryData+'static,
	F2:QueryFilter+'static,
	P2:SystemParam+'static,
{
	type Merge = SystemParamWithQueryT<(D1,D2),(F1,F2),(P1,P2)>;
}
pub type SystemParamWithQueryMergeT<A,B>=<A as SystemParamWithQueryMerge<B>>::Merge;