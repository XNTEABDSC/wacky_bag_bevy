use bevy::{app::{App, Plugin}, ecs::{resource::Resource, schedule::{IntoScheduleConfigs, ScheduleLabel}, system::ScheduleSystem}};



pub fn plugin_add_systems<Schedule:ScheduleLabel,Cfg:IntoScheduleConfigs<ScheduleSystem, M>,M>(s:Schedule,c:Cfg)->impl Plugin
	where Cfg:Send+Sync+Clone+'static,Schedule:Clone
{
	move |app:&mut App|{app.add_systems(s.clone(), c.clone());}
}

pub fn plugin_insert_resource<R>(r:R)->impl Plugin
	where R:Send+Sync+Clone+'static+Resource
{
	move |app:&mut App|{app.insert_resource(r.clone());}
}