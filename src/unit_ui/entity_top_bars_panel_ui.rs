//! [spawn_entity_top_bar_ui]
//! 
//! [TopBarsPanelUI]

use std::{any::type_name_of_val, ops::Deref};

use bevy::{app::{App, Plugin}, ecs::{component::Component, entity::Entity, message::{Message, MessageReader}, schedule::{IntoScheduleConfigs, IntoSystemSet, SystemSet}, system::{Commands, ParallelCommands, Query}}, log::warn, reflect::Reflect, tasks::{ComputeTaskPool, ParallelSlice}, ui::{Node, PositionType}};
use derive_more::Deref;


use bevy::prelude::default;

use crate::{spawn::{msg_spawn_entity::MsgForEntity, spawn_schedule}, unit_ui::world_ui_node::{self, HasEntityWorldUiNode}};

/// Top Bars UI for a entity is ui node which at the top (in screen) of a entity in world position.
#[derive(Component,Reflect)]
#[relationship( relationship_target = HasTopBarsPanelUI)]
pub struct TopBarsPanelUI{
    pub base_entity:Entity,
}

/// On entity to its top bars ui.
/// 
/// Notes that each ui per camera.
#[derive(Component,Reflect,Deref)]
#[relationship_target( relationship = TopBarsPanelUI)]
pub struct HasTopBarsPanelUI{
    node_entity:Vec<Entity>,
}

/// Spawn [TopBarsPanelUI] for a base entity, at its [EntityWorldUiNodeTar](super::entity_transform_mark_ui_node::EntityWorldUiNodeTar)
pub trait SpawnEntityTopBarsPanelUIMsg{
    fn bottom(&self)->f32;
    fn width(&self)->f32;
	//fn get_entity_world_ui_node(&self)->Entity;
    fn on(&self, _top_bars_ui:&Entity, _cmd:&mut Commands){}
}

/// A Simple [SpawnEntityTopBarsPanelUIMsg]
/// 
#[derive(Message)]
pub struct SpawnEntityTopBarsPanelUIMsgA{
    pub entity:Entity,
    //pub base_entity:Entity,
    pub bottom:f32,
    pub width:f32,
    pub on_fn:Box<dyn Fn(&Entity,&mut Commands)+Send+Sync>,
}

impl MsgForEntity for SpawnEntityTopBarsPanelUIMsgA {
    fn entity(&self)->Entity {
        self.entity
    }
}

impl SpawnEntityTopBarsPanelUIMsg for SpawnEntityTopBarsPanelUIMsgA {
    fn bottom(&self)->f32 {
        self.bottom
    }

    fn width(&self)->f32 {
        self.width
    }
	
    
    fn on(&self, top_bars_ui:&Entity, cmd:&mut Commands) {
        (self.on_fn)(top_bars_ui,cmd)
    }
	
}


/// Spawn [TopBarsPanelUI] for each base entity given by `TSpawnMsg`
pub fn spawn_entity_top_bar_ui<TSpawnMsg>(mut er:MessageReader<TSpawnMsg>, base_entities:Query<(&HasEntityWorldUiNode,)>, p_cmd:ParallelCommands) 
    where TSpawnMsg:Message+ MsgForEntity+ SpawnEntityTopBarsPanelUIMsg,
        // TSpawnMsg:for<'a>Has<'a,EntityWorldUiNode,()>
{
    er.par_read().for_each(|e|{
        let Ok(base_entity)=base_entities.get(e.entity()) else {
            warn!("entity {} not found in {}", e.entity(),type_name_of_val(&base_entities));
            return;
        };
        base_entity.0.deref().par_splat_map(ComputeTaskPool::get(), None, |_,wld_ui_node_entity_ids|{
            p_cmd.command_scope(|mut cmd|{
                wld_ui_node_entity_ids.iter().for_each(|wld_ui_node_entity_id|{
                    if cmd.get_entity(*wld_ui_node_entity_id).is_err(){
                        warn!("entity {} not exist", wld_ui_node_entity_id);
                        return;
                    }
                    //cmd.get_entity(entity)
                    let bar_def=e;
                    let top_bars_ui_entity=cmd.spawn((
                        Node{
                            width: bevy::ui::px(bar_def.width()),
                            bottom: bevy::ui::px(bar_def.bottom()),
                            left: bevy::ui::px(bar_def.width()*-0.5),
                            min_height: bevy::ui::px(8.0),

                            position_type:PositionType::Absolute,
                            display:bevy::ui::Display::Flex,
                            flex_direction: bevy::ui::FlexDirection::ColumnReverse,
                            

                            align_items: bevy::ui::AlignItems::Stretch,
                            justify_content: bevy::ui::JustifyContent::SpaceEvenly,
                            
                            // border: bevy::ui::UiRect::all( bevy::ui::px(2)),
                            
                            ..default()
                        },
                        
                        // bevy::ui::BorderColor::all(bevy::color::Color::srgba(0.5, 0.5, 0.5,0.5)),
                        // bevy::ui::BorderRadius::ZERO,
                        // UiTransform
                    ));
                    let top_bars_ui_entity_id=top_bars_ui_entity.id();
                    // entity_cmd.add_child(node_entity_id);
                    // entity_cmd.entry::<HasTopBarsPanelUI>().or_insert(HasTopBarsPanelUI { node_entity: node_entity_id });
                    cmd.entity(*wld_ui_node_entity_id).add_child(top_bars_ui_entity_id);
                    cmd.entity(e.entity()).add_one_related::<TopBarsPanelUI>(top_bars_ui_entity_id);
                    e.on(&top_bars_ui_entity_id, &mut cmd);
                });
                
            });
        });
        
    });
    
}

/// [SystemSet] of [spawn_entity_top_bar_ui]
#[derive(SystemSet,Default,Clone, Copy,Hash,PartialEq,Eq,Debug)]
pub struct SystemSetSpawnEntityTopBarsPanelUi;

pub fn plugin(app:&mut App){
	
        app.add_message::<SpawnEntityTopBarsPanelUIMsgA>();
        app.add_plugins( plugin_for_t::<SpawnEntityTopBarsPanelUIMsgA,_>(
            world_ui_node::SystemSetSpawnWorldUiNode));
}

pub fn plugin_for_t<TSpawnMsg,M>(system_set_spawn_entity_transform_mark_ui_node:impl IntoSystemSet<M>+Copy+Send+Sync+'static) ->impl Plugin
    where TSpawnMsg:Message+ MsgForEntity+ SpawnEntityTopBarsPanelUIMsg,
        // TSpawnMsg:for<'a>Has<'a,EntityWorldUiNode,()>
{
    return move |app:&mut App|{
        app.add_systems(spawn_schedule(), spawn_entity_top_bar_ui::<TSpawnMsg>.after(system_set_spawn_entity_transform_mark_ui_node).in_set(SystemSetSpawnEntityTopBarsPanelUi));

    }
}