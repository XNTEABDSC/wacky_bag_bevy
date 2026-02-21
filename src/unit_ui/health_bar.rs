//! [HealthBar]
//! 
//! [SpawnEntityTopBar<HealthBar>]
//! 
//! [plugin_for_t]


use std::any::type_name_of_val;

use bevy::{app::{App, Update}, color::Color, ecs::{component::Component, entity::Entity, message::{Message, MessageReader}, schedule::IntoSystemSet, system::{ParallelCommands, Query}}, log::tracing, utils::default};
use frunk::{hlist, HList};

use crate::{basics::{health::{Health, MaxHealth}, spawn::{SpawnEntityMsg, spawn_schedule}}, stat_component::stat::Stat, unit_ui::{entity_top_bar::{self, EntityTopBarData, SpawnEntityTopBar}, entity_top_bars_panel_ui::SystemSetSpawnEntityTopBarsUi}, visual::VisualPluginMark};
use bevy::reflect::Reflect;

#[derive(Component,Reflect,Clone, Copy)]
pub struct HealthBarData{
    pub hp:f32,
    pub max_hp:f32,
    pub perc:f32,
}
impl Default for HealthBarData {
    fn default() -> Self {
        Self { hp: 0.0, max_hp: 0.0, perc: 1.0 }
    }
}

impl HealthBarData {
    pub fn new(max_hp:f32,hp:f32)->Self{
            let perc=if max_hp!=0.0 {hp/max_hp} else {1.0};
            Self { hp, max_hp, perc }

    }
    pub fn new_h(v:HList!(&MaxHealth,&Stat<Health>))->Self{
        let hp=v.tail.head.0.0;
        let max_hp=v.head.max_health.0;
        Self::new(max_hp,hp)
    }
}

impl EntityTopBarData for HealthBarData {
    fn bar_color(&self)->bevy::color::Color {
        Color::srgb(0.0, 1.0, 0.0)
    }

    fn bar_background_color(&self)->bevy::color::Color {
        Color::srgb(0.2, 0.2, 0.2)
    }

    fn info_text(&self)->String {
        String::from("Hp")
    }

    fn info_text_color(&self)->bevy::color::Color {
        Color::srgb(0.5, 0.5, 0.5)
    }

    fn bar_text(&self)->String {
        //std::fmt
        format!("{:.1}/{:.1}",self.hp,self.max_hp)
    }

    fn bar_text_color(&self)->bevy::color::Color {
        Color::srgb(0.5, 0.5, 0.5)
    }

    fn bar_percent(&self)->f32 {
        self.perc
    }
}

pub fn update_health_bar_data(mut node_query:Query<(&mut HealthBarData,&MaxHealth,&Stat<Health>)>){
    node_query.par_iter_mut().for_each(|(mut hp_bar,max_hp,hp)|{
        *hp_bar=HealthBarData::new_h(hlist![max_hp,hp]);
    });
}

pub fn spawn_health_bar_data<TMsg>(mut mr:MessageReader<TMsg>,query:Query<(Option<(&MaxHealth,&Stat<Health>)>,)>,p_cmd:ParallelCommands)
    where TMsg:Message+SpawnEntityMsg+SpawnEntityTopBar<HealthBarData>
{
    mr.par_read().for_each(|m|{
        let Ok((datas,))=query.get(m.entity()) else {
            tracing::error!("entity {} not found in {}",m.entity(),type_name_of_val(&query));
            return;
        };
        let hp_data= match datas {
            Some((max_hp,hp)) => HealthBarData::new_h(hlist![max_hp,hp]),
            None => default(),
        };
        p_cmd.command_scope(move |mut cmd|{
            cmd.entity(m.entity()).entry::<HealthBarData>().and_modify(move |mut a|{*a=hp_data;}).or_insert(hp_data);
        });

    });
}
#[derive(Message)]
pub struct SpawnHealthBarMsgA{
    entity:Entity
}

impl SpawnEntityMsg for SpawnHealthBarMsgA {
    fn entity(&self)->Entity {
        self.entity
    }
}

impl SpawnEntityTopBar<HealthBarData> for SpawnHealthBarMsgA {
    
}



pub fn plugin(app:&mut App){
    
    if !app.get_added_plugins::<VisualPluginMark>().is_empty(){
        // app.add_systems(spawn_schedule(), spawn_health_bar::<SpawnHealthBarA>.after(spawn_entity_top_bar_ui::<SpawnEntityTopBarsUIMsgA>));
        

        app.add_message::<SpawnHealthBarMsgA>();
        

        app.add_plugins(plugin_for_t::<SpawnHealthBarMsgA,_>( SystemSetSpawnEntityTopBarsUi));

        app.add_plugins(entity_top_bar::plugin_for_t::<HealthBarData>());

        app.add_systems(Update, update_health_bar_data);
    }
}

pub fn plugin_for_t<TMsg,M1>(sys_spawn_top_bars:impl IntoSystemSet<M1>+Copy+Send+Sync+'static)->impl bevy::prelude::Plugin
    where TMsg:Message+SpawnEntityMsg + SpawnEntityTopBar<HealthBarData>,
{
    return move|app:&mut App|{
        if !app.get_added_plugins::<VisualPluginMark>().is_empty(){
            // app.add_systems(spawn_schedule(), spawn_health_bar::<TMsg>.after(spawn_entity_top_bar_ui::<TSpawnUIMsg>));
            let sys_spawn_t=spawn_health_bar_data::<TMsg>;
            app.add_systems(spawn_schedule(), sys_spawn_t);
            app.add_plugins(entity_top_bar::plugin_for_t_spawn::<HealthBarData,TMsg,_,_>(sys_spawn_top_bars, sys_spawn_t ));
            
        }
    };
}