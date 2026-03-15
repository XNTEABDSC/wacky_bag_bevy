use bevy::ecs::schedule::{IntoScheduleConfigs, Schedulable, ScheduleConfigs, SystemSet};
use frunk::{Func, Poly, hlist::{HFoldLeftable}};

/// `|i:(ScheduleConfigs<T>,Set)|i.0.in_set(i.1)`
pub struct FoldScheduleConfigsInSet;

impl<T,Set> Func<(ScheduleConfigs<T>,Set)> for FoldScheduleConfigsInSet
	where 
		// Systems:IntoScheduleConfigs<T, M1>,
		T:Schedulable<Metadata = bevy::ecs::schedule::GraphInfo, GroupMetadata = bevy::ecs::schedule::Chain>,
		Set:SystemSet,
{
	type Output=ScheduleConfigs<T>;

	fn call(i: (ScheduleConfigs<T>,Set)) -> Self::Output {
		i.0.in_set(i.1)
	}
}

pub fn schedule_configs_in_sets<T,Sets>(schedule_configs:ScheduleConfigs<T>,sets:Sets)->ScheduleConfigs<T>
	where 
		T:Schedulable<Metadata = bevy::ecs::schedule::GraphInfo, GroupMetadata = bevy::ecs::schedule::Chain>,
		Sets:HFoldLeftable<Poly<FoldScheduleConfigsInSet>,ScheduleConfigs<T>,Output = ScheduleConfigs<T>>
{
	sets.foldl(Poly(FoldScheduleConfigsInSet), schedule_configs)
}

pub trait ScheduleConfigsInSets<T>
	where T:Schedulable<Metadata = bevy::ecs::schedule::GraphInfo, GroupMetadata = bevy::ecs::schedule::Chain>
{
	/// set [self] to be in all sets. sets is [frunk::hlist]
	fn in_sets<Sets>(self,sets:Sets)->ScheduleConfigs<T>
		where Sets:HFoldLeftable<Poly<FoldScheduleConfigsInSet>,ScheduleConfigs<T>,Output = ScheduleConfigs<T>>;
}

impl<T> ScheduleConfigsInSets<T> for ScheduleConfigs<T> 
	where T:Schedulable<Metadata = bevy::ecs::schedule::GraphInfo, GroupMetadata = bevy::ecs::schedule::Chain>
{
	fn in_sets<Sets>(self,sets:Sets)->ScheduleConfigs<T> 
		where Sets:HFoldLeftable<Poly<FoldScheduleConfigsInSet>,ScheduleConfigs<T>,Output = ScheduleConfigs<T>>
	{
		schedule_configs_in_sets(self,sets)
	}
}
/// `|i: (ScheduleConfigs<T>,Set)|i.0.after(i.1)`
pub struct FoldScheduleConfigsAfterSets;

impl<T,Set> Func<(ScheduleConfigs<T>,Set)> for FoldScheduleConfigsAfterSets
	where 
		T:Schedulable<Metadata = bevy::ecs::schedule::GraphInfo, GroupMetadata = bevy::ecs::schedule::Chain>,
		Set:SystemSet,
{
	type Output=ScheduleConfigs<T>;

	fn call(i: (ScheduleConfigs<T>,Set)) -> Self::Output {
		i.0.after(i.1)
	}
}

pub fn schedule_configs_after_sets<T,Sets>(schedule_configs:ScheduleConfigs<T>,sets:Sets)->ScheduleConfigs<T>
	where 
		T:Schedulable<Metadata = bevy::ecs::schedule::GraphInfo, GroupMetadata = bevy::ecs::schedule::Chain>,
		Sets:HFoldLeftable<Poly<FoldScheduleConfigsAfterSets>,ScheduleConfigs<T>,Output = ScheduleConfigs<T>>
{
	sets.foldl(Poly(FoldScheduleConfigsAfterSets), schedule_configs)
}

pub trait ScheduleConfigsAfterSets<T>
	where T:Schedulable<Metadata = bevy::ecs::schedule::GraphInfo, GroupMetadata = bevy::ecs::schedule::Chain>
{
	/// set [self] to be after all sets. sets is [frunk::hlist]
	fn after_sets<Sets>(self,sets:Sets)->ScheduleConfigs<T>
		where Sets:HFoldLeftable<Poly<FoldScheduleConfigsAfterSets>,ScheduleConfigs<T>,Output = ScheduleConfigs<T>>;
}

impl<T> ScheduleConfigsAfterSets<T> for ScheduleConfigs<T> 
	where T:Schedulable<Metadata = bevy::ecs::schedule::GraphInfo, GroupMetadata = bevy::ecs::schedule::Chain>
{
	fn after_sets<Sets>(self,sets:Sets)->ScheduleConfigs<T> 
		where Sets:HFoldLeftable<Poly<FoldScheduleConfigsAfterSets>,ScheduleConfigs<T>,Output = ScheduleConfigs<T>>
	{
		schedule_configs_after_sets(self,sets)
	}
}


/// `|i:(ScheduleConfigs<T>,Set)|i.0.before(i.1)`
pub struct FoldScheduleConfigsBeforeSets;

impl<T,Set> Func<(ScheduleConfigs<T>,Set)> for FoldScheduleConfigsBeforeSets
	where 
		// Systems:IntoScheduleConfigs<T, M1>,
		T:Schedulable<Metadata = bevy::ecs::schedule::GraphInfo, GroupMetadata = bevy::ecs::schedule::Chain>,
		Set:SystemSet,
{
	type Output=ScheduleConfigs<T>;

	fn call(i: (ScheduleConfigs<T>,Set)) -> Self::Output {
		i.0.before(i.1)
	}
}

pub fn schedule_configs_before_sets<T,Sets>(schedule_configs:ScheduleConfigs<T>,sets:Sets)->ScheduleConfigs<T>
	where 
		T:Schedulable<Metadata = bevy::ecs::schedule::GraphInfo, GroupMetadata = bevy::ecs::schedule::Chain>,
		Sets:HFoldLeftable<Poly<FoldScheduleConfigsBeforeSets>,ScheduleConfigs<T>,Output = ScheduleConfigs<T>>
{
	sets.foldl(Poly(FoldScheduleConfigsBeforeSets), schedule_configs)
}

pub trait ScheduleConfigsBeforeSets<T>
	where T:Schedulable<Metadata = bevy::ecs::schedule::GraphInfo, GroupMetadata = bevy::ecs::schedule::Chain>
{
	///// set [self] to be before all sets. sets is [frunk::hlist]
	fn before_sets<Sets>(self,sets:Sets)->ScheduleConfigs<T>
		where Sets:HFoldLeftable<Poly<FoldScheduleConfigsBeforeSets>,ScheduleConfigs<T>,Output = ScheduleConfigs<T>>;
}

impl<T> ScheduleConfigsBeforeSets<T> for ScheduleConfigs<T> 
	where T:Schedulable<Metadata = bevy::ecs::schedule::GraphInfo, GroupMetadata = bevy::ecs::schedule::Chain>
{
	fn before_sets<Sets>(self,sets:Sets)->ScheduleConfigs<T> 
		where Sets:HFoldLeftable<Poly<FoldScheduleConfigsBeforeSets>,ScheduleConfigs<T>,Output = ScheduleConfigs<T>>
	{
		schedule_configs_before_sets(self,sets)
	}
}

#[cfg(test)]
mod test{
	use super::*;

	fn sys1() {}

	fn sys2() {}
	#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
	struct MyAudioSet;

	#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
	struct MyInputSet;
	use bevy::app::{App, Update};
use frunk::hlist;
	#[test]
	fn test() {
		let mut app=App::new();
		app.add_systems(Update, (sys1, sys2).into_configs().in_sets(hlist![MyAudioSet,MyInputSet]));
	}
}
