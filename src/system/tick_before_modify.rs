use std::marker::PhantomData;

use bevy::{app::App, ecs::{change_detection::Tick, resource::Resource, schedule::{IntoScheduleConfigs, ScheduleConfigs, ScheduleLabel}, system::{ResMut, ScheduleSystem, SystemChangeTick}}};
use frunk::{HList, HNil, Poly, hlist::{HFoldLeftable, HMappable}};
use wacky_bag_hlist::impl_phantom;

use crate::system::{multi_sets::{FoldScheduleConfigsAfterSets, FoldScheduleConfigsBeforeSets, FoldScheduleConfigsInSet}, processing_system::{MapToProcessingSystemSet, ScheduleConfigsProcessing}};

/// with [`get_system_change_tick_at<T>`], this get a tick at specified schedule
#[derive(Debug,Resource)]
pub struct SystemChangeTickAt<T>{
	pub t:Option<SystemChangeTick>,
	pub p:PhantomData<T>,
}

impl<T> Default for SystemChangeTickAt<T> {
		fn default() -> Self {
				Self { p: Default::default(), t: Default::default() }
		}
}

pub fn get_system_change_tick_at<T:Send+Sync+'static>(mut d:ResMut<SystemChangeTickAt<T>>,t:SystemChangeTick){
	d.t=Some(t);
}

pub fn get_system_change_tick_at_plugin<T:Send+Sync+'static, InputCompoents,ProcessingComponents,OutputComponents>(app:&mut App,schedule:impl ScheduleLabel)
where 
	InputCompoents:HMappable< Poly<MapToProcessingSystemSet> ,
		Output : 
			Default 
			+HFoldLeftable<Poly<FoldScheduleConfigsAfterSets>, ScheduleConfigs<ScheduleSystem>,Output = ScheduleConfigs<ScheduleSystem>>
		>,

	ProcessingComponents:HMappable< Poly<MapToProcessingSystemSet> ,
		Output : 
			Default 
			+HFoldLeftable<Poly<FoldScheduleConfigsInSet>, ScheduleConfigs<ScheduleSystem>,Output = ScheduleConfigs<ScheduleSystem>>
		>,

	OutputComponents:HMappable< Poly<MapToProcessingSystemSet> ,
		Output : 
			Default 
			+HFoldLeftable<Poly<FoldScheduleConfigsBeforeSets>, ScheduleConfigs<ScheduleSystem>,Output = ScheduleConfigs<ScheduleSystem>>
		>,
{
	app.init_resource::<SystemChangeTickAt<T>>();
	app.add_systems(schedule, get_system_change_tick_at::<T>.into_configs().config_processing::<
		InputCompoents,
		ProcessingComponents,
		OutputComponents
	>());
}

pub struct SystemChangeTickAtBeforeProcessingMarker<T>(pub PhantomData<T>);
impl_phantom!(SystemChangeTickAtBeforeProcessingMarker<T>);

pub type SystemChangeTickAtBeforeProcessing<T>=SystemChangeTickAt<SystemChangeTickAtBeforeProcessingMarker<T>>;

pub fn get_system_change_tick_at_before_processing_plugin<T:Send+Sync+'static>(app:&mut App,schedule:impl ScheduleLabel){
	get_system_change_tick_at_plugin::<
		SystemChangeTickAtBeforeProcessingMarker<T>,
		HNil,
		HNil,
		HList!(T)
	>(app,schedule);
}
// impl_phantom!(BeforeModify<T>);

