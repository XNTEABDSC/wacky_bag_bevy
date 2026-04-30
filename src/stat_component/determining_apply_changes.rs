

use std::{marker::PhantomData, ops::{AddAssign, Deref, DerefMut}};

use bevy::{app::{App, FixedPostUpdate}, ecs::{query::With, schedule::{Chain, GraphInfo, IntoScheduleConfigs, Schedulable, ScheduleConfigs}, system::{Query, ScheduleSystem}}, utils::default};
use frunk::{Func, HList, HNil, Poly, hlist::{HFoldLeftable, HMappable, HZippable}};
use num_traits::Zero;
use wacky_bag::utils::h_list_helpers::{HMapP, HZip, MapToPhantom};

use crate::{stat_component::{change::Change, determining::Determining, stat::Stat}, system::processing_system::ScheduleConfigsProcessing, utils::stat_for_hlist::{MapToChange, MapToStat}};

pub fn stat_apply_change<TStat,TChange,S,C>(mut change:C,mut stat:S)
	where 
		TStat:Zero+AddAssign<TChange>,
        TChange:Zero+AddAssign<TChange>,
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

/// for each [`Stat<T>`] and [`Change<T>`] with [`Determining<T>`], apply changes and reset [`Change<T>`].
pub fn determining_apply_changes<T>(mut query:Query<(&mut Stat<T>,&mut Change<T>),With<Determining<T>>>)
    where 
        T:Zero+AddAssign + std::marker::Send + std::marker::Sync+'static
{
    (&mut query).par_iter_mut().for_each(|(stat,change)|{
        stat_apply_change(change,stat);
    });
}

pub fn determining_apply_changes_plugin<T>(app:&mut App)
    where 
        T:Zero+AddAssign + std::marker::Send + std::marker::Sync+'static
{
	app.add_systems(FixedPostUpdate, determining_apply_changes::<T>.into_configs()
		.config_processing::<HNil,HNil,HList!(Stat<T>,Change<T>)>()
	);
}

/// for each [`Stat<TStat>`] and [`Change<TChange>`] with [`Determining<TStat>`], apply changes and reset [`Change<TChange>`].
pub fn determining_apply_changes_2<TStat,TChange>(mut query:Query<(&mut Stat<TStat>,&mut Change<TChange>),With<Determining<TStat>>>)
    where 
        //T:Deref<Target : AddAssign+Sized>+DerefMut+Into<T::Target>+ Send+ Sync+'static+Default
        TStat:Zero+AddAssign<TChange> + std::marker::Send + std::marker::Sync+'static,
        TChange:Zero+AddAssign<TChange> + std::marker::Send + std::marker::Sync+'static,
{
    (&mut query).par_iter_mut().for_each(|(stat,change)|{
        // **value += delta.get_and_reset();
		stat_apply_change(change,stat);
    });
}
pub fn determining_apply_changes_2_plugin<TStat,TChange>(app:&mut App)
    where 
        TStat:Zero+AddAssign<TChange> + std::marker::Send + std::marker::Sync+'static,
        TChange:Zero+AddAssign<TChange> + std::marker::Send + std::marker::Sync+'static,
{
	app.add_systems(FixedPostUpdate, 
		determining_apply_changes_2::<TStat,TChange>.into_configs()
		.config_processing::<HNil,HNil,HList!(Stat<TStat>,Change<TChange>)>()
	);
}



pub fn determining_apply_changes_2_spawn<TStats,TChanges>()
->ScheduleConfigs<ScheduleSystem>
where 
	TChanges:
		HMappable<Poly<MapToChange>>,
	TStats: 
		HMappable<Poly<MapToStat>, Output : HZippable<TChanges::Output,
			Zipped : HMappable<Poly<MapToPhantom>,
				Output : Default+HMappable<Poly<StatChangeToApplyChangesCfg>,
					Output : HFoldLeftable<Poly<FoldCollectCfg>,Vec<ScheduleConfigs<ScheduleSystem>>,Output = Vec<ScheduleConfigs<ScheduleSystem>>>>>>>

{
	let scs:
		HMapP<HZip<
			HMapP<TStats,MapToStat>,
			HMapP<TChanges,MapToChange>,
		>,MapToPhantom>=default();
	let fns=scs.map(Poly(StatChangeToApplyChangesCfg));
	let cfgs=fns.foldl(Poly(FoldCollectCfg), Vec::new());
	ScheduleConfigs::Configs { configs: cfgs, collective_conditions: default(), metadata: default() }
	// fns.into_tuple2()
}

// pub fn determining_apply_changes_2_spawn<TStats,Markers>()
// where 
// 	TStats: 
// 		HMappable<Poly<MapToPhantom>, 
// 			Output : HZippable<Markers,Zipped :
// 				HMappable<Poly<MapStatToChangeTypeZ>,
// 					Output : HMappable<Poly<MapPhantomUnwrap>,
// 						Output : HMappable<Poly<MapToChange>>
// 					>
// 				>
// 			>
// 		>+
// 		HMappable<Poly<MapToStat>, Output : HZippable<>>

// {
// 	type ToStats<TStats>=HMapP<TStats,MapToStat>;
// 	type ToChanges<TStats,Markers>=HMapP< HMapP< HMapP<
// 			HZip<
// 				HMapP<
// 					TStats
// 					,MapToPhantom> 
// 				,Markers
// 			>,MapStatToChangeTypeZ
// 		>,MapPhantomUnwrap>,MapToChange>;
// 	let scs:
// 		HMapP<HZip<
// 			ToStats<TStats>,
// 			ToChanges<TStats,Markers>
// 		>,MapToPhantom>=default();
// 	let fns=scs.map(Poly(SCToDeterminingCfg));

// }

pub struct StatChangeToApplyChangesCfg;

impl<TStat,TChange> Func< PhantomData<(TStat,TChange)> > for StatChangeToApplyChangesCfg
where 
	TStat:Zero+AddAssign<TChange> + std::marker::Send + std::marker::Sync+'static,
	TChange:Zero+AddAssign<TChange> + std::marker::Send + std::marker::Sync+'static,
{
	type Output=ScheduleConfigs<bevy::ecs::system::ScheduleSystem>;

	fn call(_i: PhantomData<(TStat,TChange)> ) -> Self::Output {
		// todo!()
		determining_apply_changes_2::<TStat,TChange>.into_configs()
		.config_processing::<HNil,HNil,HList!(Stat<TStat>,Change<TChange>)>()
	}
}

impl<TStat,TChange> Func< (PhantomData<TStat>,PhantomData<TChange>) > for StatChangeToApplyChangesCfg
where 
	TStat:Zero+AddAssign<TChange> + std::marker::Send + std::marker::Sync+'static,
	TChange:Zero+AddAssign<TChange> + std::marker::Send + std::marker::Sync+'static,
{
	type Output=ScheduleConfigs<bevy::ecs::system::ScheduleSystem>;

	fn call(_i: (PhantomData<TStat>,PhantomData<TChange>) ) -> Self::Output {
		// todo!()
		determining_apply_changes_2::<TStat,TChange>.into_configs()
		.config_processing::<HNil,HNil,HList!(Stat<TStat>,Change<TChange>)>()
	}
}

pub struct FoldCollectCfg;

impl<T> Func< (Vec<ScheduleConfigs<T>>,ScheduleConfigs<T>) > for FoldCollectCfg 
	where T:Schedulable<Metadata = GraphInfo, GroupMetadata = Chain>
{
	type Output = Vec<ScheduleConfigs<T>>;

	fn call(mut i: (Vec<ScheduleConfigs<T>>,ScheduleConfigs<T>)) -> Self::Output {
		i.0.push(i.1);
		i.0
	}
}