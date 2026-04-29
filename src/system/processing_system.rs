
use std::marker::PhantomData;

use bevy::{ecs::schedule::{Schedulable, ScheduleConfigs, SystemSet}, utils::default};
use frunk::{Func, Poly, hlist::{HFoldLeftable, HMappable}};
use wacky_bag::{impl_phantom, utils::{type_fn::{OneOneMappingTypeFunc, TypeFunc}}};

use crate::system::multi_sets::{FoldScheduleConfigsAfterSets, FoldScheduleConfigsBeforeSets, FoldScheduleConfigsInSet, ScheduleConfigsAfterSets, ScheduleConfigsBeforeSets, ScheduleConfigsInSets};


/// [ProcessingSystemSet]<T> represents what a system requires(reads) and generates(writes). T is a [bevy::ecs::component::Component].
/// 
/// A system before [ProcessingSystemSet]<T> means it generates or writes T (normally initalizing). It is expected to have only one system generating T.
/// 
/// A system after [ProcessingSystemSet]<T> means it needs or reads T.
/// 
/// A system inside [ProcessingSystemSet]<T> means it modifies T. They may work parallelly.
/// 
/// see [ScheduleConfigsProcessing::config_processing] for simple usage.
#[derive(SystemSet)]
pub struct ProcessingSystemSet<T>(PhantomData<T>);
impl_phantom!(ProcessingSystemSet);

pub struct MapToProcessingSystemSet;

impl<T> TypeFunc<T> for MapToProcessingSystemSet {
	type Output=ProcessingSystemSet<T>;
}

impl<T> OneOneMappingTypeFunc<ProcessingSystemSet<T>> for MapToProcessingSystemSet {
	type Input=T;
}

impl<T> Func<T> for MapToProcessingSystemSet {
	type Output=ProcessingSystemSet<T>;

	fn call(_i: T) -> Self::Output {
		ProcessingSystemSet::default()
	}
}

pub type ProcessingSystemSets<Components>=wacky_bag::utils::h_list_helpers::HMap< Components , Poly<MapToProcessingSystemSet> >;

pub fn processing_system_sets<Components>()->ProcessingSystemSets<Components>
	where Components:HMappable<Poly<MapToProcessingSystemSet>>,
		<Components as HMappable<Poly<MapToProcessingSystemSet>>>::Output: Default
{
	default::<ProcessingSystemSets<Components>>()
}

pub fn schedule_config_processing<T,InputCompoents,ProcessingComponents,OutputComponents>(schedule_configs:ScheduleConfigs<T>)->ScheduleConfigs<T>
	where T:Schedulable<Metadata = bevy::ecs::schedule::GraphInfo, GroupMetadata = bevy::ecs::schedule::Chain>,
		
		InputCompoents:HMappable< Poly<MapToProcessingSystemSet> ,
			Output : 
				Default 
				+HFoldLeftable<Poly<FoldScheduleConfigsAfterSets>, ScheduleConfigs<T>,Output = ScheduleConfigs<T>>
			>,
		
		ProcessingComponents:HMappable< Poly<MapToProcessingSystemSet> ,
			Output : 
				Default 
				+HFoldLeftable<Poly<FoldScheduleConfigsInSet>, ScheduleConfigs<T>,Output = ScheduleConfigs<T>>
			>,
		
		OutputComponents:HMappable< Poly<MapToProcessingSystemSet> ,
			Output : 
				Default 
				+HFoldLeftable<Poly<FoldScheduleConfigsBeforeSets>, ScheduleConfigs<T>,Output = ScheduleConfigs<T>>
			>,
{
	schedule_configs
		.after_sets(processing_system_sets::<InputCompoents>())
		.in_sets(processing_system_sets::<ProcessingComponents>())
		.before_sets(processing_system_sets::<OutputComponents>())
}

pub trait ScheduleConfigsProcessing<T>
	where T:Schedulable<Metadata = bevy::ecs::schedule::GraphInfo, GroupMetadata = bevy::ecs::schedule::Chain>
{
	/// Config systems with [ProcessingSystemSet].
	/// 
	/// The systems requires `InputComponents`, modifies `ProcessingComponents`, and generates `OutputComponents`.
	/// 
	/// All type params are [frunk::hlist].
	/// 
	/// `InputComponents` `ProcessingComponents` `OutputComponents` can actually be any type marker, which is suggested to be [Component][bevy::prelude::Component] or [Resource][bevy::prelude::Resource] 
	fn config_processing<InputComponents,ProcessingComponents,OutputComponents>(self)->ScheduleConfigs<T>
		where 
			InputComponents:HMappable< Poly<MapToProcessingSystemSet> ,
				Output : 
					Default 
					+HFoldLeftable<Poly<FoldScheduleConfigsAfterSets>, ScheduleConfigs<T>,Output = ScheduleConfigs<T>>
				>,
			
			ProcessingComponents:HMappable< Poly<MapToProcessingSystemSet> ,
				Output : 
					Default 
					+HFoldLeftable<Poly<FoldScheduleConfigsInSet>, ScheduleConfigs<T>,Output = ScheduleConfigs<T>>
				>,
			
			OutputComponents:HMappable< Poly<MapToProcessingSystemSet> ,
				Output : 
					Default 
					+HFoldLeftable<Poly<FoldScheduleConfigsBeforeSets>, ScheduleConfigs<T>,Output = ScheduleConfigs<T>>
				>,
	;
}

impl<T> ScheduleConfigsProcessing<T> for ScheduleConfigs<T> 
	where T:Schedulable<Metadata = bevy::ecs::schedule::GraphInfo, GroupMetadata = bevy::ecs::schedule::Chain>
{
	fn config_processing<InputCompoents,ProcessingComponents,OutputComponents>(self)->ScheduleConfigs<T>
		where 
			InputCompoents:HMappable< Poly<MapToProcessingSystemSet> ,
				Output : 
					Default 
					+HFoldLeftable<Poly<FoldScheduleConfigsAfterSets>, ScheduleConfigs<T>,Output = ScheduleConfigs<T>>
				>,
		
			ProcessingComponents:HMappable< Poly<MapToProcessingSystemSet> ,
				Output : 
					Default 
					+HFoldLeftable<Poly<FoldScheduleConfigsInSet>, ScheduleConfigs<T>,Output = ScheduleConfigs<T>>
				>,
		
			OutputComponents:HMappable< Poly<MapToProcessingSystemSet> ,
				Output : 
					Default 
					+HFoldLeftable<Poly<FoldScheduleConfigsBeforeSets>, ScheduleConfigs<T>,Output = ScheduleConfigs<T>>
				>,
	{
		schedule_config_processing::<T,InputCompoents,ProcessingComponents,OutputComponents>(self)
	}
}