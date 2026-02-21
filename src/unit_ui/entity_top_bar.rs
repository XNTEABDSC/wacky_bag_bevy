//! [EntityTopBarData]
//! 
//! [EntityTopBarNode]
//! 
//! [spawn_entity_top_bar]
//! 
//! [plugin_for_t]


use std::marker::PhantomData;

use bevy::{app::{App, FixedPostUpdate, Plugin}, color::Color, ecs::{component::Component, entity::Entity, message::{Message, MessageReader}, query::Without, schedule::{IntoScheduleConfigs, IntoSystemSet}, system::{Commands, ParallelCommands, Query}}, log::warn, text::{TextColor, TextFont}, ui::{BackgroundColor, Node, UiTransform, Val2, percent, px, widget::Text}, utils::default};

use crate::{query_get_or_err_return, spawn::{msg_spawn_entity::MsgForEntity, spawn_schedule}, system_set_for_t, unit_ui::entity_top_bars_panel_ui::HasTopBarsPanelUI, utils::par_simple_map::ParSimpleMap};


/// A Component that contains all information needed for a entity top bar
pub trait EntityTopBarData {
    fn bar_color(&self)->Color;
    fn bar_background_color(&self)->Color;
    fn info_text(&self)->String;
    fn info_text_color(&self)->Color;
    fn bar_text(&self)->String;
    fn bar_text_color(&self)->Color;
    fn bar_percent(&self)->f32;
}

pub const FONT_SIZE:f32=16.0;
/// Text shown at the left of the bar
#[derive(Component)]
struct EntityTopBarInfoText<T>(Entity,PhantomData<T>);

impl<T> EntityTopBarInfoText<T> {
    fn new(entity: Entity) -> Self {
        Self(entity, PhantomData)
    }
}

// Background of bar
#[derive(Component)]
struct EntityTopBarBarBackground<T>(Entity,PhantomData<T>);

impl<T> EntityTopBarBarBackground<T> {
    fn new(entity: Entity) -> Self {
        Self(entity, PhantomData)
    }
}

/// Bar
#[derive(Component)]
struct EntityTopBarBar<T>(Entity,PhantomData<T>);

impl<T> EntityTopBarBar<T> {
    fn new(entity: Entity) -> Self {
        Self(entity, PhantomData)
    }
}

/// Text on bar
#[derive(Component)]
struct EntityTopBarBarText<T>(Entity,PhantomData<T>);

impl<T> EntityTopBarBarText<T> {
    fn new(entity: Entity) -> Self {
        Self(entity, PhantomData)
    }
}

/// A [EntityTopBarNode] contains 4 part:
/// 
/// [EntityTopBarInfoText]
/// 
/// [EntityTopBarBarBackground]
/// 
/// [EntityTopBarBar]
/// 
/// [EntityTopBarBarText]
/// 
#[derive(Component)]
pub struct EntityTopBarNode<T>(pub Entity,PhantomData<T>);

impl<T> EntityTopBarNode<T> {
    fn new(entity: Entity) -> Self {
        Self(entity, PhantomData)
    }
}

pub const UNIT_TOP_BAR_HEIGHT:i32=16;

/// Create a [EntityTopBarNode] with all childrens
pub fn create_entity_top_bar_entity<T>(cmd:&mut Commands,base_entity:Entity,bar_data:&T)->Entity
    where T:Component+EntityTopBarData
{
    

    let text_entity=cmd.spawn((
        Text::new(bar_data.info_text()),
        TextColor(bar_data.info_text_color()),
        TextFont{
            font_size:FONT_SIZE,
            ..default()
        },
        EntityTopBarInfoText::<T>::new(base_entity),
        Node{
            right:percent(100.0),
            position_type:bevy::ui::PositionType::Absolute,
            ..default()
        },
        UiTransform::from_translation(Val2::px(-8.0, 0.0)),
    ));
    let text_entity_id=text_entity.id();
    let mut bar_background = cmd.spawn(
        (
            BackgroundColor(bar_data.bar_background_color()),
            Node {
                // height:percent(100),
                top:percent(0.0),
                bottom:percent(0.0),
                right:percent(0.0),
                left:percent(0.0),
                position_type:bevy::ui::PositionType::Absolute,
                align_content:bevy::ui::AlignContent::Center,
                justify_content:bevy::ui::JustifyContent::Center,
                ..default()
            },
            EntityTopBarBarBackground::<T>::new(base_entity),
        )
    );
    let bar_background_id=bar_background.id();
    bar_background.with_children(|p|{
        let mut _bar_entity=p.spawn(
            (
                Node{
                    top:percent(0.0),
                    bottom:percent(0.0),
                    left:percent(0.0),
                    width:percent(bar_data.bar_percent()*100.0),
                    position_type:bevy::ui::PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(bar_data.bar_color()),
                EntityTopBarBar::<T>::new(base_entity),
            )
        );
    });
    
    bar_background.with_children(|bar_entity|{
        let _bar_text=bar_entity.spawn((
            Text::new(bar_data.bar_text()),
            TextColor(bar_data.bar_text_color()),
            TextFont{
                font_size:FONT_SIZE,
                ..default()
            },
            
            EntityTopBarBarText::<T>::new(base_entity),

            Node{
                align_self:bevy::ui::AlignSelf::Center,
                justify_self:bevy::ui::JustifySelf::Center,
                ..default()
            }
        ));
    });
    let node_entity_id=cmd.spawn(
        (
        Node{
            display:bevy::ui::Display::Flex,
            height:px(UNIT_TOP_BAR_HEIGHT),

            justify_content:bevy::ui::JustifyContent::Start,
            align_content:bevy::ui::AlignContent::Center,
            align_items:bevy::ui::AlignItems::Center,
            flex_direction:bevy::ui::FlexDirection::Row,
            
            ..default()
        },
        //bar_data,
        EntityTopBarNode::<T>::new(base_entity),
    )).add_children(&[text_entity_id,bar_background_id]).id();
    return node_entity_id;
}

/// Message to spawn a
pub trait MsgSpawnEntityTopBar<T>
    where T:EntityTopBarData
{
    
}

pub fn spawn_entity_top_bar<TMsg,T>(mut mr:MessageReader<TMsg>,base_query:Query<(&HasTopBarsPanelUI,&T)>,p_cmd:ParallelCommands)
    where TMsg:Message+MsgForEntity+MsgSpawnEntityTopBar<T>,
        T:Component+EntityTopBarData
{
    mr.par_read().for_each(|m|{
        let base_entity_id=m.entity();
        let (top_bars,t)=query_get_or_err_return!(base_query,base_entity_id);
        (*top_bars).par_simple_map(|top_bar_entity_id|{
            p_cmd.command_scope(|mut cmd|{
                let bar_entity=create_entity_top_bar_entity(&mut cmd, base_entity_id, t);
                cmd.entity(*top_bar_entity_id).add_child(bar_entity);
            });
        });

    });
}

fn update_entity_top_bar_info_text<T>(
    mut info_text:Query<(&mut Text,&mut TextColor,&EntityTopBarInfoText<T>),(Without<T>,)>,
    node:Query<(&T,),(Without<EntityTopBarInfoText<T>>,)>
)
    where T:Component+EntityTopBarData
{
    info_text.par_iter_mut().for_each(|(mut text,mut text_color,child_of)|{
        let Ok(parent)=node.get(child_of.0)else{
            warn!("entity {} not found",child_of.0);
            return;
        };
        text.0=parent.0.info_text();
        text_color.0=parent.0.info_text_color();
    });
}

fn update_entity_top_bar_bar_background<T>(
    mut info_text:Query<(&mut BackgroundColor,&EntityTopBarBarBackground<T>),(Without<T>,)>,
    node:Query<(&T,),(Without<EntityTopBarBarBackground<T>>,)>
)
    where T:Component+EntityTopBarData
{
    info_text.par_iter_mut().for_each(|(mut color,child_of)|{
        let Ok(parent)=node.get(child_of.0)else{
            warn!("entity {} not found",child_of.0);
            return;
        };
        color.0=parent.0.bar_background_color();
    });
}

fn update_entity_top_bar_bar<T>(
    mut info_text:Query<(&mut BackgroundColor,&mut Node,&EntityTopBarBar<T>),(Without<T>,)>,
    node:Query<(&T,),(Without<EntityTopBarBar<T>>,)>
)
    where T:Component+EntityTopBarData
{
    info_text.par_iter_mut().for_each(|(mut color,mut e_node,child_of)|{
        let Ok(parent)=node.get(child_of.0)else{
            warn!("entity {} not found",child_of.0);
            return;
        };
        color.0=parent.0.bar_color();
        e_node.width=percent(parent.0.bar_percent()*100.0);

    });
}

fn update_entity_top_bar_bar_text<T>(
    mut info_text:Query<(&mut Text,&mut TextColor,&EntityTopBarBarText<T>),(Without<T>,)>,
    node:Query<(&T,),(Without<EntityTopBarBarText<T>>,)>
)
    where T:Component+EntityTopBarData
{
    info_text.par_iter_mut().for_each(|(mut text,mut text_color,child_of)|{
        let Ok(parent)=node.get(child_of.0)else{
            warn!("entity {} not found",child_of.0);
            return;
        };
        text.0=parent.0.bar_text();
        text_color.0=parent.0.bar_text_color();
    });
}

pub fn plugin_update_entity_top_bar<T>(app:&mut App)
    where T:Component+EntityTopBarData
{
    app.add_systems(FixedPostUpdate, (
        update_entity_top_bar_info_text::<T>,
        update_entity_top_bar_bar_background::<T>,
        update_entity_top_bar_bar::<T>,
        update_entity_top_bar_bar_text::<T>,
    ));
}

system_set_for_t!(SystemSetSpawnEntityTopBarT);


pub fn plugin_spawn<TSpawnMsg,T,M1,M2>(sys_spawn_top_bars:impl IntoSystemSet<M1>+Copy+Send+Sync+'static, sys_spawn_t: impl IntoSystemSet<M2>+Copy+Send+Sync+'static)->impl Plugin
    where TSpawnMsg:Message+ MsgForEntity+ MsgSpawnEntityTopBar<T>,
        T:Component+EntityTopBarData
{
    return move |app:&mut App|{
        app.add_systems(spawn_schedule(), spawn_entity_top_bar::<TSpawnMsg,T>.after(sys_spawn_top_bars).after(sys_spawn_t).in_set(SystemSetSpawnEntityTopBarT::<T>::default()) );
    };
}

pub fn plugin_for_t<T>()->impl Plugin
    where 
        T:Component+EntityTopBarData
{
    return move|app:&mut App|{
        app.add_plugins(plugin_update_entity_top_bar::<T>);
    };
}

pub fn plugin_for_t_spawn<T,TSpawnMsg,M1,M2>(sys_spawn_top_bars:impl IntoSystemSet<M1>+Copy+Send+Sync+'static, sys_spawn_t: impl IntoSystemSet<M2>+Copy+Send+Sync+'static)->impl Plugin
    where TSpawnMsg:Message + MsgForEntity + MsgSpawnEntityTopBar<T>,
        T:Component+EntityTopBarData
{
    return move|app:&mut App|{
        app.add_plugins(plugin_spawn::<TSpawnMsg,T,M1,M2>(sys_spawn_top_bars,sys_spawn_t));
    };
}