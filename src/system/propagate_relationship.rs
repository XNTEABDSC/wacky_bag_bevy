use std::{mem::swap, ops::{AddAssign, ControlFlow::{Break, Continue}, DerefMut}};

use bevy::{ecs::{entity::{Entity, EntityHash}, name::Name, query::{QueryData, QueryItem, ROQueryItem, With, Without}, relationship::{Relationship, RelationshipSourceCollection, RelationshipTarget}, system::{Local, ParamSet, Query}}, log::{debug, error}, tasks::{ComputeTaskPool, TaskPool}, utils::Parallel};
use dashmap::DashMap;
use num_traits::Zero;
use crate::stat_component::change::Change;

pub trait PropagateLeafToRoot
{
	type Data:QueryData;
	fn from_data<'w,'s>(values:&ROQueryItem<'w,'s,Self::Data>)->Self;
	fn apply_to_data<'w,'s>(self,values:&ROQueryItem<'w,'s,Self::Data>);
}

pub struct PropagateChangeLeafToRoot<T>(pub T);
impl<T> PropagateLeafToRoot for PropagateChangeLeafToRoot<T>
	where T:Send+Sync+AddAssign+'static+Zero
{
	type Data=&'static Change<T>;

	fn from_data<'w,'s>(values:&ROQueryItem<'w,'s,Self::Data>)->Self {
		Self(values.get_and_reset_ref())
	}

	fn apply_to_data<'w,'s>(self,values:&ROQueryItem<'w,'s,Self::Data>) {
		values.add_change(self.0);
	}
}

type UpdateSourcesSet=DashMap<Entity,usize,EntityHash>;
pub fn propagate_leaf_to_root<T,R>(
	mut ps:ParamSet<(
		Query<(T::Data,&R),Without<R::RelationshipTarget>>,
		Query<(T::Data,&R::RelationshipTarget,Option<&R>)>,
	)>,
	update_sources_set:
	Local<UpdateSourcesSet>,
	mut update_tasks:Local<(Parallel<Vec<(Entity,T)>>,Parallel<Vec<(Entity,T)>>)>,
)
	where T:PropagateLeafToRoot+Send+Sync,
	R:Relationship,
{
	
	let (task_,task_cache_)=update_tasks.deref_mut();
	let mut task=task_;
	let mut task_cache=task_cache_;

	let task_pool = ComputeTaskPool::get_or_init(TaskPool::default);
	{
		let leafs=ps.p0();

		leafs.par_iter().for_each_init(
			||task.borrow_local_mut(), 
			|a,(c,at)|{
				a.push((at.get(),T::from_data(&c)));
			}
		);
	}

	
	let bases=ps.p1();

	while task.iter_mut().try_fold((), |_,b|if b.is_empty() {Continue(())} else {Break(())}).is_break()
	{

		task_pool.scope(|s|
		{
			fn helper<T,R>(
				srcs_p:&UpdateSourcesSet,
				// tasks_p:&Parallel<Vec<(Entity,T)>>,
				tasks:&mut Vec<(Entity,T)>,
				tasks_cache_p:&Parallel<Vec<(Entity,T)>>,
				q:&Query<(T::Data,&R::RelationshipTarget,Option<&R>)>,
			)
			where 
				T:PropagateLeafToRoot+Send+Sync,
				R:Relationship,
			{
				
				let mut tasks_cache=tasks_cache_p.borrow_local_mut();

				for e in tasks.drain(..) {

					let Ok(a)=q.get(e.0) else{
						error!("entity {} dont matches query {:?}",e.0,q);
						continue;
					};

					// let mut str=String::new();
					// str+=&format!("resolve task {}, childs: {}",e.0,a.1.collection().len());
					

					e.1.apply_to_data(&a.0);

					if let Some(at)=a.2 {

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
							let v=a.1.collection().len()-1;
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
							tasks_cache.push((at.get(),T::from_data(&a.0)));
						}

					}

					// println!("{str}");
					
				}
				
			}

			task.iter_mut().for_each(|a|{
				if !a.is_empty() {
					s.spawn(async {helper(
						&update_sources_set,
						a,
						&task_cache,
						&bases
					)})
				}
			});
			
		});

		swap::<&mut Parallel<_>>(&mut task, &mut task_cache);

	}

	update_sources_set.clear();
}

pub trait PropagateRootToLeaf:Clone
{
	type DataBegin:QueryData;
	type Data:QueryData;
	fn from_data<'w,'s>(values:&ROQueryItem<'w,'s,Self::DataBegin>)->Self;
	fn process_data<'w,'s>(&mut self,values:&ROQueryItem<'w,'s,Self::Data>);
}

pub fn propagate_root_to_leaf<T,R>(
	mut ps:ParamSet<(
		Query<(T::DataBegin,&R::RelationshipTarget),(Without<R>,)>,
		Query<(T::Data,&R,Option<&R::RelationshipTarget>)>,
	)>,
	mut update_tasks:Local<(Parallel<Vec<(Entity,T)>>,Parallel<Vec<(Entity,T)>>)>,
)
where T:PropagateRootToLeaf+Send+Sync+Clone,
	R:Relationship,
{
	let (task_,task_cache_)=update_tasks.deref_mut();
	let mut task=task_;
	let mut task_cache=task_cache_;

	let task_pool = ComputeTaskPool::get_or_init(TaskPool::default);

	ps.p0().par_iter().for_each_init(||task.borrow_local_mut(),|l,q|{
		let t=T::from_data(&q.0);
		q.1.iter().for_each(|e|{
			l.push((e,t.clone()));
		});
	});

	let nodes_q=ps.p1();
	while task.iter_mut().try_fold((), |_,b|if b.is_empty() {Continue(())} else {Break(())}).is_break() {
		
		task_pool.scope(|scope|{
			task.iter_mut().for_each(|task_list|{
				scope.spawn(async{
					let mut task_cache=task_cache.borrow_local_mut();
					for (e,mut t) in task_list.drain(..) {
						let a=match nodes_q.get(e) {
								Ok(a) => a,
								Err(e) => {error!("{e}");continue;},
							};
						t.process_data(&a.0);
						if let Some(rt)=a.2 {
							for i in rt.collection().iter() {
								task_cache.push((i,t.clone()));
							}
						}
					}
				});
			});
		});

		swap::<&mut Parallel<_>>(&mut task, &mut task_cache);
	}
}
#[cfg(test)]
mod test{
	use super::*;
	use bevy::{ecs::world::CommandQueue, prelude::*};
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
}