use std::{mem::swap, ops::{AddAssign, ControlFlow::{Break, Continue}, DerefMut}};

use bevy::{ecs::{entity::{Entity, EntityHash}, query::{QueryItem, ROQueryItem, Without}, relationship::{Relationship, RelationshipSourceCollection, RelationshipTarget}, system::{Local, ParamSet, SystemParam, SystemParamItem}}, log::error, tasks::{ComputeTaskPool, TaskPool}, utils::Parallel};
use dashmap::DashMap;
use num_traits::Zero;
use crate::{stat_component::change::Change, utils::system_param_with_query::{SystemParamWithQuery, SystemParamWithQueryMerge, SystemParamWithQueryParam, SystemParamWithQueryQuery, SystemParamWithQueryT}};
type SPQMerge<A,B>=<A as SystemParamWithQueryMerge<B>>::Merge;

/// Methods for [`propagate_leaf_to_root`]
pub trait PropagateLeafToRoot<R>
where R:Relationship
{
	/// [`SystemParam`] for [`from_data`]
	type FromSysParam:SystemParamWithQuery
	// where PropagateLeafToRootFromSysParam<R>:SystemParamWithQueryMerge<Self::FromSysParam>;
	;
	/// Create data from child
	fn from_data(
		values: 
			ROQueryItem<
				<SPQMerge<PropagateLeafToRootFromSysParam<R>, Self::FromSysParam> as SystemParamWithQuery>::D
			>,
		others:
			&SystemParamItem<
				<SPQMerge<PropagateLeafToRootFromSysParam<R>, Self::FromSysParam> as SystemParamWithQuery>::P
			>
	)->Self;
	/// [`SystemParam`] for [`apply_to_data`]
	type ApplySysParam:SystemParamWithQuery
	// where PropagateLeafToRootApplySysParam<R>:SystemParamWithQueryMerge<Self::ApplySysParam>;
	;
	/// Apply data to parent
	fn apply_to_data(
		self,
		values:
			ROQueryItem<
				<SPQMerge<PropagateLeafToRootApplySysParam<R>, Self::ApplySysParam> as SystemParamWithQuery>::D
			>,
		//SystemParamWithQueryROItem<'w,'s,SPQMerge<PropagateLeafToRootApplySysParam<R>, Self::ApplySysParam>>,
		others:
			&SystemParamItem<
				<SPQMerge<PropagateLeafToRootApplySysParam<R>, Self::ApplySysParam> as SystemParamWithQuery>::P
			>
	);
}
pub struct PropagateChangeLeafToRoot<T>(pub T);
impl<T,R> PropagateLeafToRoot<R> for PropagateChangeLeafToRoot<T>
where T:Send+Sync+AddAssign+'static+Zero,
	R:Relationship
{
	// type FromSysParam<'w,'s>=SystemParamWithQueryT<'w,'s,&'s Change<T>,(),()>;
	type FromSysParam = SystemParamWithQueryT<&'static Change<T>,(),()>
		where PropagateLeafToRootFromSysParam<R>:SystemParamWithQueryMerge<Self::FromSysParam>;
		
	
	type ApplySysParam = SystemParamWithQueryT<&'static Change<T>,(),()>
		where PropagateLeafToRootApplySysParam<R>:SystemParamWithQueryMerge<Self::ApplySysParam>;
		
	fn from_data(
		values: 
			ROQueryItem<
				<SPQMerge<PropagateLeafToRootFromSysParam<R>, Self::FromSysParam> as SystemParamWithQuery>::D
			>,
		_others:
			&SystemParamItem<
				<SPQMerge<PropagateLeafToRootFromSysParam<R>, Self::FromSysParam> as SystemParamWithQuery>::P
			>
	)->Self {
		Self(values.1.get_and_reset_ref())
	}

	fn apply_to_data(self,
		values:
			ROQueryItem<
				<SPQMerge<PropagateLeafToRootApplySysParam<R>, Self::ApplySysParam> as SystemParamWithQuery>::D
			>,
		//SystemParamWithQueryROItem<'w,'s,SPQMerge<PropagateLeafToRootApplySysParam<R>, Self::ApplySysParam>>,
		_others:
			&SystemParamItem<
				<SPQMerge<PropagateLeafToRootApplySysParam<R>, Self::ApplySysParam> as SystemParamWithQuery>::P
			>
	) {
		let a=values.1;
		a.add_change(self.0);
	}
}

pub type PropagateLeafToRootFromSysParam<R>=SystemParamWithQueryT<
	&'static R,
	(),
	()>;
pub type PropagateLeafToRootApplySysParam<R>=SystemParamWithQueryT< 
	(&'static <R as Relationship>::RelationshipTarget,Option<&'static R>,Entity) , 
	(),
	()>;
pub type PropagateLeafToRootFromSysParamBegin<R>=SystemParamWithQueryT<
	&'static R,
	Without<<R as Relationship>::RelationshipTarget>,
	()>;

type UpdateSourcesSet=DashMap<Entity,usize,EntityHash>;
/// Propagate data from leaf to root through [`Relationship`] with method [`PropagateLeafToRoot`]
pub fn propagate_leaf_to_root<T,R>(
	mut ps:ParamSet<(
		SystemParamWithQueryParam<SPQMerge<PropagateLeafToRootFromSysParam<R>, T::FromSysParam>>,
		SystemParamWithQueryParam<SPQMerge<PropagateLeafToRootApplySysParam<R>, T::ApplySysParam>> ,
		SystemParamWithQueryParam<SPQMerge<PropagateLeafToRootFromSysParamBegin<R>, T::FromSysParam>>,
	)>,
	update_sources_set:
	Local<UpdateSourcesSet>,
	mut update_tasks:Local<(Parallel<Vec<(Entity,T)>>,Parallel<Vec<Entity>>)>,
)
	where T:PropagateLeafToRoot<R>+Send+Sync,
	R:Relationship,
	for <'w,'s> <<<T as PropagateLeafToRoot<R>>::FromSysParam as SystemParamWithQuery>::P as SystemParam>::Item<'w, 's>: Sync+Send,
	for <'w,'s> <<<T as PropagateLeafToRoot<R>>::ApplySysParam as SystemParamWithQuery>::P as SystemParam>::Item<'w, 's>: Sync+Send
{
	
	let (task_,task_from_)=update_tasks.deref_mut();
	let task=task_;
	let task_from=task_from_;

	let task_pool = ComputeTaskPool::get_or_init(TaskPool::default);
	{
		let (leafs,others)=ps.p2();

		leafs.par_iter().for_each_init(
			||task.borrow_local_mut(), 
			|a,(at,c)|{
				a.push( (at.get(),T::from_data((at,c),&others)) );
			}
		);
	}

	
	

	while task.iter_mut().try_fold((), |_,b| if b.is_empty() {Continue(())} else {Break(())} ).is_break()
	{
		{
			let (bases,others)=ps.p1();
			task_pool.scope(|s|
			{
				fn helper<T,R>(
					srcs_p:&UpdateSourcesSet,
					// tasks_p:&Parallel<Vec<(Entity,T)>>,
					tasks:&mut Vec<(Entity,T)>,
					task_from_p:&Parallel<Vec<Entity>>,
					q: & SystemParamWithQueryQuery<SPQMerge<PropagateLeafToRootApplySysParam<R>, T::ApplySysParam>>,
					o: & SystemParamItem<<SPQMerge<PropagateLeafToRootApplySysParam<R>, T::ApplySysParam> as SystemParamWithQuery>::P >
				)
				where 
					T:PropagateLeafToRoot<R>+Send+Sync,
					R:Relationship,
				{
					
					let mut task_from=task_from_p.borrow_local_mut();

					for e in tasks.drain(..) {

						let Ok(a)=q.get(e.0) else{
							error!("entity {} dont matches query {:?}",e.0,q);
							continue;
						};

						// let mut str=String::new();
						// str+=&format!("resolve task {}, childs: {}",e.0,a.1.collection().len());
						
						if let Some(_at)=a.0.1 {

							let spread=
							if let Some(mut v)=srcs_p.get_mut(&e.0) {
								*v-=1;
								// str+=&format!("node spread v left {}",*v);
								if *v==0 {
									drop(v);
									srcs_p.remove(&e.0);
									true
								}else {
									false
								}
							}else{
								let v=a.0.0.collection().len()-1;
								// str+=&format!("new node spread v left {}",v);
								if v==0 {
									true
								}else {
									srcs_p.insert(e.0, v);
									false
								}
							};
							
							if spread {
								// str+=&format!("generate task {} ",at.get());
								// tasks_cache.push((at.get(),T::from_data(&a.0)));
								task_from.push(a.0.2);
							}

						}

						e.1.apply_to_data(a,o);

						// println!("{str}");
						
					}
					
				}

				task.iter_mut().for_each(|a|{
					if !a.is_empty() {
						s.spawn( async {helper(
							&update_sources_set,
							a,
							&task_from,
							&bases,
							&others
						)})
					}
				});
				
			});
		}
		if task_from.iter_mut().try_fold((), |_,b| if b.is_empty() {Continue(())} else {Break(())} ).is_continue() {
			break;
		}
		{
			let (leafs,others)=ps.p0();
			task_pool.scope(|s|{
				task_from.iter_mut().for_each(|task_from|{
					if !task_from.is_empty() {
						s.spawn( async {
							let mut task=task.borrow_local_mut();
							for e in task_from.drain(..){
								let Ok((at,c))=leafs.get(e) else {
									continue;
								};
								task.push((at.get(),T::from_data((at,c),&others)));
							}
						} );
					}
				});
			});
		}

		

		// leafs.par_iter().for_each_init(
		// 	||task.borrow_local_mut(), 
		// 	|a,(at,c)|{
		// 		a.push( (at.get(),T::from_data((at,c),&others)) );
		// 	}
		// );
		// swap::<&mut Parallel<_>>(&mut task, &mut task_from);
		
	}

	update_sources_set.clear();
}
/// Methods for [`propagate_leaf_to_root_mut`]
pub trait PropagateLeafToRootMut<R>
where R:Relationship
{
	/// [`SystemParam`] for [`from_data`]
	type FromSysParam:SystemParamWithQuery
	// where PropagateLeafToRootFromSysParam<R>:SystemParamWithQueryMerge<Self::FromSysParam>;
	;
	/// Create data from child
	fn from_data(
		values: 
			QueryItem<
				<SPQMerge<PropagateLeafToRootMutFromSysParam<R>, Self::FromSysParam> as SystemParamWithQuery>::D
			>,
		others: 
			&SystemParamItem<
				<SPQMerge<PropagateLeafToRootMutFromSysParam<R>, Self::FromSysParam> as SystemParamWithQuery>::P
			>
	)->Self;

	type ApplySysParam:SystemParamWithQuery
	// where PropagateLeafToRootApplySysParam<R>:SystemParamWithQueryMerge<Self::ApplySysParam>;
	;
	/// Apply data to parent
	fn apply_to_data(
		self,
		values:
			ROQueryItem<
				<SPQMerge<PropagateLeafToRootMutApplySysParam<R>, Self::ApplySysParam> as SystemParamWithQuery>::D
			>,
		//SystemParamWithQueryROItem<'w,'s,SPQMerge<PropagateLeafToRootApplySysParam<R>, Self::ApplySysParam>>,
		others:
			&SystemParamItem<
				<SPQMerge<PropagateLeafToRootMutApplySysParam<R>, Self::ApplySysParam> as SystemParamWithQuery>::P
			>
	);
}
pub struct PropagateChangeLeafToRootMut<T>(pub T);
impl<T,R> PropagateLeafToRootMut<R> for PropagateChangeLeafToRootMut<T>
where T:Send+Sync+AddAssign+'static+Zero,
	R:Relationship
{
	// type FromSysParam<'w,'s>=SystemParamWithQueryT<'w,'s,&'s Change<T>,(),()>;
	type FromSysParam = SystemParamWithQueryT<&'static mut Change<T>,(),()>
		where PropagateLeafToRootMutFromSysParam<R>:SystemParamWithQueryMerge<Self::FromSysParam>;
		
	
	type ApplySysParam = SystemParamWithQueryT<&'static Change<T>,(),()>
		where PropagateLeafToRootMutApplySysParam<R>:SystemParamWithQueryMerge<Self::ApplySysParam>;
		
	fn from_data(
		mut values: 
			QueryItem<
				<SPQMerge<PropagateLeafToRootMutFromSysParam<R>, Self::FromSysParam> as SystemParamWithQuery>::D
			>,
		_others:
			&SystemParamItem<
				<SPQMerge<PropagateLeafToRootMutFromSysParam<R>, Self::FromSysParam> as SystemParamWithQuery>::P
			>
	)->Self {
		Self(values.1.get_and_reset())
	}

	fn apply_to_data(self,
		values:
			ROQueryItem<
				<SPQMerge<PropagateLeafToRootMutApplySysParam<R>, Self::ApplySysParam> as SystemParamWithQuery>::D
			>,
		//SystemParamWithQueryROItem<'w,'s,SPQMerge<PropagateLeafToRootApplySysParam<R>, Self::ApplySysParam>>,
		_others:
			&SystemParamItem<
				<SPQMerge<PropagateLeafToRootMutApplySysParam<R>, Self::ApplySysParam> as SystemParamWithQuery>::P
			>
	) {
		let a=values.1;
		a.add_change(self.0);
	}
}

pub type PropagateLeafToRootMutFromSysParam<R>=SystemParamWithQueryT<
	&'static R,
	(),
	()>;
pub type PropagateLeafToRootMutApplySysParam<R>=SystemParamWithQueryT< 
	(&'static <R as Relationship>::RelationshipTarget,Option<&'static R>,Entity) , 
	(),
	()>;
pub type PropagateLeafToRootMutFromSysParamBegin<R>=SystemParamWithQueryT<
	&'static R,
	Without<<R as Relationship>::RelationshipTarget>,
	()>;
	
/// Propagate data from leaf to root through [`Relationship`] with method [`PropagateLeafToRoot`]
/// 
/// # Safety
/// 
/// [`Relationship`] ensures tree shape, that one [`Relationship`] will not have multiple [`RelationshipTarget`], otherwise Rust's aliasing guarantees can be violated by this function. 
pub fn propagate_leaf_to_root_mut<T,R>(
	mut ps:ParamSet<(
		SystemParamWithQueryParam<SPQMerge<PropagateLeafToRootMutFromSysParam<R>, T::FromSysParam>>,
		SystemParamWithQueryParam<SPQMerge<PropagateLeafToRootMutApplySysParam<R>, T::ApplySysParam>> ,
		SystemParamWithQueryParam<SPQMerge<PropagateLeafToRootMutFromSysParamBegin<R>, T::FromSysParam>>,
	)>,
	update_sources_set:
	Local<UpdateSourcesSet>,
	mut update_tasks:Local<(Parallel<Vec<(Entity,T)>>,Parallel<Vec<Entity>>)>,
)
	where T:PropagateLeafToRootMut<R>+Send+Sync,
	R:Relationship,
	for <'w,'s> <<<T as PropagateLeafToRootMut<R>>::FromSysParam as SystemParamWithQuery>::P as SystemParam>::Item<'w, 's>: Sync+Send,
	for <'w,'s> <<<T as PropagateLeafToRootMut<R>>::ApplySysParam as SystemParamWithQuery>::P as SystemParam>::Item<'w, 's>: Sync+Send
{
	
	let (task_,task_from_)=update_tasks.deref_mut();
	let task=task_;
	let task_from_p=task_from_;

	let task_pool = ComputeTaskPool::get_or_init(TaskPool::default);
	{
		let (mut leafs,others)=ps.p2();

		leafs.par_iter_mut().for_each_init(
			||task.borrow_local_mut(), 
			|a,(at,c)|{
				a.push( (at.get(),T::from_data((at,c),&others)) );
			}
		);
	}

	while task.iter_mut().try_fold((), |_,b| if b.is_empty() {Continue(())} else {Break(())} ).is_break()
	{
		{
			let (query,others)=ps.p1();
			task_pool.scope(|s|
			{
				task.iter_mut().for_each(|tasks|{
					if !tasks.is_empty() {
						s.spawn( async {
							let mut task_from=task_from_p.borrow_local_mut();

							for e in tasks.drain(..) {

								let Ok(a)=query.get(e.0) else{
									error!("entity {} dont matches query {:?}",e.0,query);
									continue;
								};

								// let mut str=String::new();
								// str+=&format!("resolve task {}, childs: {}",e.0,a.1.collection().len());
								
								if let Some(_at)=a.0.1 {

									let spread=
									if let Some(mut v)=update_sources_set.get_mut(&e.0) {
										*v-=1;
										// str+=&format!("node spread v left {}",*v);
										if *v==0 {
											drop(v);
											update_sources_set.remove(&e.0);
											true
										}else {
											false
										}
									}else{
										let v=a.0.0.collection().len()-1;
										// str+=&format!("new node spread v left {}",v);
										if v==0 {
											true
										}else {
											update_sources_set.insert(e.0, v);
											false
										}
									};
									
									if spread {
										// str+=&format!("generate task {} ",at.get());
										// tasks_cache.push((at.get(),T::from_data(&a.0)));
										task_from.push(a.0.2);
									}

								}

								e.1.apply_to_data(a,&others);

								// println!("{str}");
								
							}
						})
					}
				});
				
			});
		}
		if task_from_p.iter_mut().try_fold((), |_,b| if b.is_empty() {Continue(())} else {Break(())} ).is_continue() {
			break;
		}
		{
			let (leafs,others)=ps.p0();
			task_pool.scope(|s|{
				task_from_p.iter_mut().for_each(|task_from|{
					if !task_from.is_empty() {
						s.spawn( async {
							let mut task=task.borrow_local_mut();
							for e in task_from.drain(..){
								let mut reb=unsafe{leafs.reborrow_unsafe()};
								let Ok((at,c))=reb.get_mut(e) else {
									continue;
								};
								task.push((at.get(),T::from_data((at,c),&others)));
							}
						} );
					}
				});
			});
		}
	}
	update_sources_set.clear();
}

pub trait PropagateRootToLeaf<R>
where R:Relationship
{
	/// [`SystemParam`] for [`from_data`]
	type BeginSysParam:SystemParamWithQuery
	// where PropagateLeafToRootFromSysParam<R>:SystemParamWithQueryMerge<Self::FromSysParam>;
	;
	/// Create data from child
	fn process_data_begin(
		values: 
			ROQueryItem<
				<SPQMerge<PropagateRootToLeafBeginSysParam<R>, Self::BeginSysParam> as SystemParamWithQuery>::D
			>,
		others: 
			&SystemParamItem<
				<SPQMerge<PropagateRootToLeafBeginSysParam<R>, Self::BeginSysParam> as SystemParamWithQuery>::P
			>
	)->
	// Vec<(Entity,Self)>
	impl FnMut(Entity)->Self
	;

	type ProcessSysParam:SystemParamWithQuery;
	fn process_data<'w,'s>(
		self,
		values: 
			ROQueryItem<
				<SPQMerge<PropagateRootToLeafProcessSysParam<R>, Self::ProcessSysParam> as SystemParamWithQuery>::D
			>,
		others: 
			&SystemParamItem<
				<SPQMerge<PropagateRootToLeafProcessSysParam<R>, Self::ProcessSysParam> as SystemParamWithQuery>::P
			>)
	->
	// Vec<(Entity,Self)>
	impl FnMut(Entity)->Self
	;
}

pub type PropagateRootToLeafBeginSysParam<R>=SystemParamWithQueryT<
	&'static <R as Relationship>::RelationshipTarget,
	Without<R>,
	()>;
pub type PropagateRootToLeafProcessSysParam<R>=SystemParamWithQueryT< 
	(&'static R,Option<&'static <R as Relationship>::RelationshipTarget>) , 
	(),
	()>;
// pub type PropagateRootToLeafFromSysParamBegin<R>=SystemParamWithQueryT<
// 	&'static <R as Relationship>::RelationshipTarget,
// 	Without<R>,
// 	()>;

pub fn propagate_root_to_leaf<T,R>(
	mut ps:ParamSet<(
		SystemParamWithQueryParam<SPQMerge<PropagateRootToLeafBeginSysParam<R>, T::BeginSysParam>>,
		SystemParamWithQueryParam<SPQMerge<PropagateRootToLeafProcessSysParam<R>, T::ProcessSysParam>>,
	)>,
	mut update_tasks:Local<(Parallel<Vec<(Entity,T)>>,Parallel<Vec<(Entity,T)>>)>,
)
where T:PropagateRootToLeaf<R>+Send+Sync+Clone,
	R:Relationship,
	for <'w,'s> <<<T as PropagateRootToLeaf<R>>::BeginSysParam as SystemParamWithQuery>::P as SystemParam>::Item<'w, 's>: Sync+Send,
	for <'w,'s> <<<T as PropagateRootToLeaf<R>>::ProcessSysParam as SystemParamWithQuery>::P as SystemParam>::Item<'w, 's>: Sync+Send
{
	let (task_,task_cache_)=update_tasks.deref_mut();
	let mut task=task_;
	let mut task_cache=task_cache_;

	let task_pool = ComputeTaskPool::get_or_init(TaskPool::default);

	// ps.p0().par_iter().for_each_init(||task.borrow_local_mut(),|l,q|{
	// 	let t=T::process_data_begin(&q.0);
	// 	q.1.iter().for_each(|e|{
	// 		l.push((e,t.clone()));
	// 	});
	// });

	{
		let (q,o)=ps.p0();
		q.par_iter().for_each_init(||task.borrow_local_mut(),|l,q|{
			let es=q.0.collection();
			let mut t=T::process_data_begin(q,&o);
			es.iter().for_each(|e|{
				l.push((e,t(e)));
			});
		});
	}

	{
		let (q,o)=ps.p1();
		while task.iter_mut().try_fold((), |_,b|if b.is_empty() {Continue(())} else {Break(())}).is_break() {
			task_pool.scope(|scope|{
				task.iter_mut().for_each(|task_list|{
					scope.spawn(async{
						let mut task_cache=task_cache.borrow_local_mut();
						for (e,t) in task_list.drain(..) {
							let a=match q.get(e) {
								Ok(a) => a,
								Err(e) => {error!("{e}");continue;},
							};
							if let Some(rt)=a.0.1 {
								let es=rt.collection();
			
								let mut f=t.process_data(a,&o);
								es.iter().for_each(|e|{
									task_cache.push((e,f(e)));
								});
								// for i in rt.collection().iter() {
								// 	task_cache.push((i,t.clone()));
								// }
							}else {
								let _f=t.process_data(a,&o);
							}
						}
					});
				});
			});

			swap::<&mut Parallel<_>>(&mut task, &mut task_cache);
		}
	}

	// let nodes_q=ps.p1();
	// while task.iter_mut().try_fold((), |_,b|if b.is_empty() {Continue(())} else {Break(())}).is_break() {
		
	// 	task_pool.scope(|scope|{
	// 		task.iter_mut().for_each(|task_list|{
	// 			scope.spawn(async{
	// 				let mut task_cache=task_cache.borrow_local_mut();
	// 				for (e,mut t) in task_list.drain(..) {
	// 					let a=match nodes_q.get(e) {
	// 							Ok(a) => a,
	// 							Err(e) => {error!("{e}");continue;},
	// 						};
	// 					t.process_data(&a.0);
	// 					if let Some(rt)=a.2 {
	// 						for i in rt.collection().iter() {
	// 							task_cache.push((i,t.clone()));
	// 						}
	// 					}
	// 				}
	// 			});
	// 		});
	// 	});

	// 	swap::<&mut Parallel<_>>(&mut task, &mut task_cache);
	// }
}


pub trait PropagateRootToLeafMut<R>:Clone
where R:Relationship
{
	/// [`SystemParam`] for [`from_data`]
	type BeginSysParam:SystemParamWithQuery
	// where PropagateLeafToRootFromSysParam<R>:SystemParamWithQueryMerge<Self::FromSysParam>;
	;
	/// Create data from child
	fn process_data_begin(
		values: 
			QueryItem<
				<SPQMerge<PropagateRootToLeafMutBeginSysParam<R>, Self::BeginSysParam> as SystemParamWithQuery>::D
			>,
		others: 
			&SystemParamItem<
				<SPQMerge<PropagateRootToLeafMutBeginSysParam<R>, Self::BeginSysParam> as SystemParamWithQuery>::P
			>
	)->
	// Vec<(Entity,Self)>
	impl FnMut(Entity)->Self
	;

	type ProcessSysParam:SystemParamWithQuery;
	fn process_data<'w,'s>(
		self,
		values: 
			QueryItem<
				<SPQMerge<PropagateRootToLeafMutProcessSysParam<R>, Self::ProcessSysParam> as SystemParamWithQuery>::D
			>,
		others: 
			&SystemParamItem<
				<SPQMerge<PropagateRootToLeafMutProcessSysParam<R>, Self::ProcessSysParam> as SystemParamWithQuery>::P
			>)
	->
	// Vec<(Entity,Self)>
	impl FnMut(Entity)->Self
	;
}

pub type PropagateRootToLeafMutBeginSysParam<R>=SystemParamWithQueryT<
	&'static <R as Relationship>::RelationshipTarget,
	Without<R>,
	()>;
pub type PropagateRootToLeafMutProcessSysParam<R>=SystemParamWithQueryT< 
	(&'static R,Option<&'static <R as Relationship>::RelationshipTarget>) , 
	(),
	()>;
/// Propagate data via [`Relationship`] and method [`PropagateRootToLeafMut`]
/// 
/// # Safety
/// 
/// [`Relationship`] ensures tree shape, that one [`Relationship`] will not have multiple [`RelationshipTarget`], otherwise Rust's aliasing guarantees can be violated by this function. 
pub fn propagate_root_to_leaf_mut<T,R>(
	mut ps:ParamSet<(
		SystemParamWithQueryParam<SPQMerge<PropagateRootToLeafMutBeginSysParam<R>, T::BeginSysParam>>,
		SystemParamWithQueryParam<SPQMerge<PropagateRootToLeafMutProcessSysParam<R>, T::ProcessSysParam>>,
	)>,
	mut update_tasks:Local<(Parallel<Vec<(Entity,T)>>,Parallel<Vec<(Entity,T)>>)>,
)
where T:PropagateRootToLeafMut<R>+Send+Sync+Clone,
	R:Relationship,
	for <'w,'s> <<<T as PropagateRootToLeafMut<R>>::BeginSysParam as SystemParamWithQuery>::P as SystemParam>::Item<'w, 's>: Sync+Send,
	for <'w,'s> <<<T as PropagateRootToLeafMut<R>>::ProcessSysParam as SystemParamWithQuery>::P as SystemParam>::Item<'w, 's>: Sync+Send
{
	let (task_,task_cache_)=update_tasks.deref_mut();
	let mut task=task_;
	let mut task_cache=task_cache_;

	let task_pool = ComputeTaskPool::get_or_init(TaskPool::default);

	{
		let (mut q,o)=ps.p0();
		q.par_iter_mut().for_each_init(||task.borrow_local_mut(),|l,q|{
			let es=q.0.collection();
			let mut t=T::process_data_begin(q,&o);
			es.iter().for_each(|e|{
				l.push((e,t(e)));
			});
		});
	}

	{
		let (q,o)=ps.p1();
		while task.iter_mut().try_fold((), |_,b|if b.is_empty() {Continue(())} else {Break(())}).is_break() {
			task_pool.scope(|scope|{
				task.iter_mut().for_each(|task_list|{
					scope.spawn(async{
						let mut task_cache=task_cache.borrow_local_mut();
						for (e,t) in task_list.drain(..) {
							// Safety: assume all e are different
							let mut q_b=unsafe {q.reborrow_unsafe()};
							let a=match q_b.get_mut(e) {
								Ok(a) => a,
								Err(e) => {error!("{e}");continue;},
							};
							if let Some(rt)=a.0.1 {
								let es=rt.collection();
			
								let mut f=t.process_data(a,&o);
								es.iter().for_each(|e|{
									task_cache.push((e,f(e)));
								});
								// for i in rt.collection().iter() {
								// 	task_cache.push((i,t.clone()));
								// }
							}else {
								let _f=t.process_data(a,&o);
							}
						}
					});
				});
			});

			swap::<&mut Parallel<_>>(&mut task, &mut task_cache);
		}
	}
}
// (
// 	mut ps:ParamSet<(
// 		Query<(T::DataBegin,&R::RelationshipTarget),(Without<R>,)>,
// 		Query<(T::Data,&R,Option<&R::RelationshipTarget>)>,
// 	)>,
// 	mut update_tasks:Local<(Parallel<Vec<(Entity,T)>>,Parallel<Vec<(Entity,T)>>)>,
// )
// where T:PropagateRootToLeafMut+Send+Sync+Clone,
// 	R:Relationship,
// {
// 	let (task_,task_cache_)=update_tasks.deref_mut();
// 	let mut task=task_;
// 	let mut task_cache=task_cache_;

// 	let task_pool = ComputeTaskPool::get_or_init(TaskPool::default);

// 	ps.p0().par_iter_mut().for_each_init(||task.borrow_local_mut(),|l,q|{
// 		let t=T::from_data(&q.0);
// 		q.1.iter().for_each(|e|{
// 			l.push((e,t.clone()));
// 		});
// 	});

// 	let nodes_q=ps.p1();
// 	while task.iter_mut().try_fold((), |_,b|if b.is_empty() {Continue(())} else {Break(())}).is_break() {
		
// 		task_pool.scope(|scope|{
// 			task.iter_mut().for_each(|task_list|{
// 				scope.spawn(async{
// 					let mut task_cache=task_cache.borrow_local_mut();
// 					for (e,mut t) in task_list.drain(..) {
// 						let mut nodes_q_b=unsafe {nodes_q.reborrow_unsafe()};
// 						let a=match nodes_q_b.get_mut(e) {
// 								Ok(a) => a,
// 								Err(e) => {error!("{e}");continue;},
// 							};
// 						t.process_data(&a.0);
// 						if let Some(rt)=a.2 {
// 							for i in rt.collection().iter() {
// 								task_cache.push((i,t.clone()));
// 							}
// 						}
// 					}
// 				});
// 			});
// 		});

// 		swap::<&mut Parallel<_>>(&mut task, &mut task_cache);
// 	}
// }


#[cfg(test)]
#[allow(unused)]
mod test{
	use crate::stat_component::stat::Stat;

use super::*;
	use bevy::{ecs::{system::SystemChangeTick, world::CommandQueue}, prelude::*};
	fn check_log(q:Query<(Entity,&Name,&Change<f64>,Option<&ChildOf>,Option<&Children>)>){
		for (e,n,c,cof,cs) in q {
			println!("e: {e}, n: {n}, c: {c:?}, cof: {cof:?}, cs: {cs:?}");
		}
	}
	#[test]
	fn test_propagate_leaf_to_root(){
		ComputeTaskPool::get_or_init(TaskPool::default);
        let mut world = World::default();

		let mut check_log_schedule = Schedule::default();
		check_log_schedule.add_systems(check_log);

		let mut propagate_leaf_to_root_schedule = Schedule::default();
		propagate_leaf_to_root_schedule.add_systems(
			propagate_leaf_to_root::<PropagateChangeLeafToRoot<f64>,ChildOf>
		);

		let mut command_queue = CommandQueue::default();
        let mut commands = Commands::new(&mut command_queue, &world);

        let root = commands.spawn((Change::new(1.1),Name::new("root"))).id();
        let parent = commands.spawn((Change::new(2.2),Name::new("parent"))).id();
        let child = commands.spawn((Change::new(3.3),Name::new("child"))).id();
        let child2 = commands.spawn((Change::new(4.4),Name::new("child2"))).id();
        let child3 = commands.spawn((Change::new(5.5),Name::new("child3"))).id();
        commands.entity(parent).insert(ChildOf(root));
        commands.entity(child).insert(ChildOf(parent));
        commands.entity(child2).insert(ChildOf(parent));
        commands.entity(child3).insert(ChildOf(child2));
        command_queue.apply(&mut world);

		check_log_schedule.run(&mut world);
		println!();
        propagate_leaf_to_root_schedule.run(&mut world);
		println!();
		check_log_schedule.run(&mut world);
		println!();

		// assert_eq!(
        //     *world.get::<Change<f64>>(root).unwrap().0.lock().unwrap(),
        //     1.1+2.2+3.3+4.4+5.5,
		// 	"root's change not updated well"
        //     // "The transform systems didn't run, ie: `GlobalTransform` wasn't updated",
        // );

		// assert_eq!(
        //     *world.get::<Change<f64>>(parent).unwrap().0.lock().unwrap(),
        //     0.0,
		// 	"parent's change not updated well"
        //     // "The transform systems didn't run, ie: `GlobalTransform` wasn't updated",
        // );
		
		// assert_eq!(
        //     *world.get::<Change<f64>>(child).unwrap().0.lock().unwrap(),
        //     0.0,
		// 	"child's change not updated well"
        //     // "The transform systems didn't run, ie: `GlobalTransform` wasn't updated",
        // );

		let mut command_queue = CommandQueue::default();
        let mut commands = Commands::new(&mut command_queue, &world);
        commands.entity(child).entry::<Change<f64>>().and_modify(|v|v.add_change(6.6));
        commands.entity(child2).entry::<Change<f64>>().and_modify(|v|v.add_change(7.7));
        command_queue.apply(&mut world);
		
		check_log_schedule.run(&mut world);
		println!();
        propagate_leaf_to_root_schedule.run(&mut world);
		println!();
		check_log_schedule.run(&mut world);
		println!();

		// assert_eq!(
        //     *world.get::<Change<f64>>(root).unwrap().0.lock().unwrap(),
        //     3.3+4.4+5.5+6.6,
        //     // "The transform systems didn't run, ie: `GlobalTransform` wasn't updated",
        // );

		// assert_eq!(
        //     *world.get::<Change<f64>>(parent).unwrap().0.lock().unwrap(),
        //     0.0,
        //     // "The transform systems didn't run, ie: `GlobalTransform` wasn't updated",
        // );
		
		// assert_eq!(
        //     *world.get::<Change<f64>>(child).unwrap().0.lock().unwrap(),
        //     0.0,
        //     // "The transform systems didn't run, ie: `GlobalTransform` wasn't updated",
        // );
	}

	fn test_tick_sys(q:Query<Ref<Stat<i32>>>,t:SystemChangeTick){
		println!("test_tick tick:{:?}",t);
		println!("last_changed:{:?}",q.single().unwrap().last_changed());
	}
	fn change_tick_sys(mut q:Query<&mut Stat<i32>>,t:SystemChangeTick){
		println!("change_tick tick:{:?}",t);
		let mut v=q.single_mut().unwrap();
		println!("last_changed:{:?}",v.last_changed());
		v.0+=1;
		println!("last_changed:{:?}",v.last_changed());
	}
	#[test]
	fn test_tick(){
		
		ComputeTaskPool::get_or_init(TaskPool::default);
        let mut world = World::default();

		
		let mut sys_schedule = Schedule::default();
		sys_schedule.add_systems(
			(change_tick_sys,test_tick_sys).chain()
		);

		let mut command_queue = CommandQueue::default();
        let mut commands = Commands::new(&mut command_queue, &world);

        let root = commands.spawn((Stat(5i32))).id();
		command_queue.apply(&mut world);

		sys_schedule.run(&mut world);
		println!();
		sys_schedule.run(&mut world);
		println!();
		sys_schedule.run(&mut world);
		println!();
	}
}