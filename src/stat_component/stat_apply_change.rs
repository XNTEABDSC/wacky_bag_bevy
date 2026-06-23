

use std::{marker::{PhantomData, Send, Sync}, ops::{AddAssign, Deref, DerefMut}};

use bevy::{app::{App, FixedPostUpdate, Plugin}, ecs::{query::{QueryFilter, With}, schedule::{Chain, GraphInfo, IntoScheduleConfigs, Schedulable, ScheduleConfigs}, system::{Query, ScheduleSystem}}, utils::default};
use frunk::{Func, HList, HNil, Poly, hlist::{HFoldLeftable, HMappable, HZippable}};


use num_traits::Zero;
use physics_basic::stat_to_change_type::StatToChangeType;
use wacky_bag_hlist::{new_struct_func,{h_list_helpers::{FoldVecPush, HMapP, HRepeatFrom, HZip, MapToPhantom}, type_fn::TypeFunc}};
use wacky_bag::structures::owned::Owned;
use crate::{stat_component::{change::Change, determining::Determining, stat::Stat}, system::processing_system::ScheduleConfigsProcessing, utils::stat_for_hlist::{MapToChange, MapToStat}};

pub fn stat_apply_change<TStat,TChange,S,C>(mut change:C,mut stat:S)
	where 
		TStat:AddAssign<TChange>,
        TChange:Zero,
		S:Deref<Target = Stat<TStat>>+DerefMut,
		C:Deref<Target = Change<TChange>>+DerefMut
{
	**stat += change.get_and_reset();
}

pub fn change_apply_change<TChange,CM,CR>(mut source:CM,target:CR)
	where 
        TChange:Zero+AddAssign<TChange>,
		CM:Deref<Target = Change<TChange>>+DerefMut,
		CR:Deref<Target = Change<TChange>>
{
	target.add_change(source.get_and_reset());
}

/// [`ScheduleConfigsProcessing::config_processing`] for [`determining_apply_changes`]
/// 
/// input: `HList!(Change<TChange>)`
/// 
/// processing: `HList!(Determining<TStat>)`
/// 
/// output: `HList!(Stat<TStat>)`
pub fn set_stat_apply_change_config<TStat,TChange>(cfg:ScheduleConfigs<ScheduleSystem>)->ScheduleConfigs<ScheduleSystem>
	where TStat:'static+Send+Sync, TChange:'static+Send+Sync,
{
	cfg.config_processing::<HList!(Change<TChange>),HNil,HList!(Stat<TStat>)>()
}

/// for each [`Stat<TStat>`] and [`Change<TChange>`] with [`Determining<TStat>`], apply changes and reset [`Change<TChange>`].
pub fn stat_apply_change_system<TStat,TChange,Filter>(mut query:Query<(&mut Stat<TStat>,&mut Change<TChange>),Filter>)
    where 
        TStat:AddAssign<TChange> + Send + Sync+'static,
        TChange:Zero + Send + Sync+'static,
		Filter:QueryFilter
{
    (&mut query).par_iter_mut().for_each(|(stat,change)|{
		stat_apply_change(change,stat);
    });
}

new_struct_func!{
	pub StatChangeToApplyChanges
	impl<TStat,TChange,Filter> 
	{where
		TStat:AddAssign<TChange> + Send + Sync+'static,
		TChange:Zero + Send + Sync+'static,
		Filter:QueryFilter+'static}:
	(PhantomData<((TStat,TChange),Filter)>)->(ScheduleConfigs<ScheduleSystem>)
	|_|{
		set_stat_apply_change_config::<TStat,TChange>(
			stat_apply_change_system::<TStat,TChange,Filter>.into_configs()
		)
	}
}

// pub struct StatChangeToApplyChanges;

// impl<TStat,TChange,Filter> Func<PhantomData<((TStat,TChange),Filter)>> for StatChangeToApplyChanges
// where 
// 	TStat:AddAssign<TChange> + Send + Sync+'static,
// 	TChange:Zero + Send + Sync+'static,
// 	Filter:QueryFilter+'static
// {
// 	type Output = ScheduleConfigs<ScheduleSystem>;
	
// 	fn call(_: PhantomData<((TStat,TChange),Filter)>) -> Self::Output {
// 		set_stat_apply_change_config::<TStat,TChange>(
// 			stat_apply_change_system::<TStat,TChange,Filter>.into_configs()
// 		)
		
// 	}
// }


pub fn stat_apply_change_system_spawn<TStats,TChanges,Filters>()
->ScheduleConfigs<ScheduleSystem>
where 
	Filters:
		Clone,
	TChanges:
		HMappable<Poly<MapToChange>>,
	TStats: 
		HMappable<Poly<MapToStat>, Output : HZippable<TChanges::Output,
			Zipped : HZippable<Filters,
				Zipped : HMappable<Poly<MapToPhantom>,
					Output : Default+HMappable<Poly<StatChangeToApplyChanges>,
						Output : HFoldLeftable<Poly<FoldVecPush>,Owned<Vec<ScheduleConfigs<ScheduleSystem>>>,Output = Vec<ScheduleConfigs<ScheduleSystem>>>
						>
					>
				>
			>
		>
{
	let scs:
		HMapP<
		HZip<
			HZip<
			HMapP<TStats,MapToStat>,
			HMapP<TChanges,MapToChange>,>,
			Filters
		>
		,MapToPhantom>=default();
	let fns=scs.map(Poly(StatChangeToApplyChanges));
	let cfgs=fns.foldl(Poly(FoldVecPush), Owned(Vec::new()));
	ScheduleConfigs::Configs { configs: cfgs, collective_conditions: default(), metadata: default() }
	// fns.into_tuple2()
}

pub struct StatChangeToApplyChangesWithSameFilter<Filter>(pub PhantomData<Filter>);

impl<Filter> Default for StatChangeToApplyChangesWithSameFilter<Filter> {
    fn default() -> Self {
		Self(Default::default())
	}
}

impl<Filter> Clone for StatChangeToApplyChangesWithSameFilter<Filter> {
    fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

impl<Filter> Copy for StatChangeToApplyChangesWithSameFilter<Filter> {}

impl<TStat,TChange,Filter> Func<PhantomData<(TStat,TChange)>> for StatChangeToApplyChangesWithSameFilter<Filter>
where 
	TStat:AddAssign<TChange> + Send + Sync+'static,
	TChange:Zero + Send + Sync+'static,
	Filter:QueryFilter+'static
{
	type Output = ScheduleConfigs<ScheduleSystem>;
	
	fn call(_: PhantomData<(TStat,TChange)>) -> Self::Output {
		stat_apply_change_system::<TStat,TChange,Filter>.into_configs()
	}
}

pub fn stat_apply_change_system_spawn_with_same_filter<TStats,TChanges,Filter>()
->ScheduleConfigs<ScheduleSystem>
where 
	Filter:
		Clone,
	TChanges:
		HMappable<Poly<MapToChange>>,
	TStats: 
		HMappable<Poly<MapToStat>, Output : HZippable<TChanges::Output,
		
			Zipped : HMappable<Poly<MapToPhantom>,
				Output : Default+HMappable<Poly<StatChangeToApplyChangesWithSameFilter<Filter>>,
					Output : HFoldLeftable<Poly<FoldVecPush>,Owned<Vec<ScheduleConfigs<ScheduleSystem>>>,Output = Vec<ScheduleConfigs<ScheduleSystem>>>
					>
				>
			>
			
		>
{
	let scs:
		HMapP<HZip<
			HMapP<TStats,MapToStat>,
			HMapP<TChanges,MapToChange>,>
		,MapToPhantom>=default();
	let fns=scs.map(Poly(StatChangeToApplyChangesWithSameFilter::<Filter>::default()));
	let cfgs=fns.foldl(Poly(FoldVecPush), Owned(Vec::new()));
	ScheduleConfigs::Configs { configs: cfgs, collective_conditions: default(), metadata: default() }
	// fns.into_tuple2()
}