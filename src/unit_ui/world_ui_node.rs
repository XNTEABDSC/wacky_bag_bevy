//! [EntityWorldUiNode]
//! 
//! [SpawnWorldUiNodeMsg]
//! 
//! [spawn_entity_transform_mark_ui_node]
//! 
//! [plugin_for_t]




use bevy::{app::{App, PluginGroupBuilder, Update}, ecs::{component::Component, entity::Entity, message::{Message, MessageReader}, resource::Resource, schedule::{IntoScheduleConfigs, SystemSet}, system::{Commands, ParallelCommands}}, reflect::Reflect};
use derive_more::Deref;

use crate::{spawn::msg_spawn_entity::MsgForEntity, system_set_for_t, unit_ui::{set_ui_transform_to_screen_ui_node::SetUiWorldToViewPositionFromRelationGlobalTransformUiTargetCamera, wld_cameras::WorldCameras}};

use bevy::prelude::Res;

/// Entity world ui node.
/// 
/// Put other ui entity as child of this to be shown around entity in world.
/// 
/// Notes that 1 node per camera.
#[derive(Component,Reflect)]
#[relationship( relationship_target = HasEntityWorldUiNode)]
pub struct EntityWorldUiNode{
    pub entity:Entity
}

/// On marked entity in world, to entity world ui node
#[derive(Component,Deref,Reflect)]
#[relationship_target( relationship = EntityWorldUiNode , linked_spawn)]
pub struct HasEntityWorldUiNode{
    nodes:Vec<Entity>
}

/// Spawn [EntityWorldUiNode]
pub trait SpawnWorldUiNodeMsg___{
    /// Called for each [EntityWorldUiNode] spawned
    fn on(&self, _node_entity:&Entity, _cmd:&mut Commands){}
}



/// A simple [SpawnWorldUiNodeMsg] to spawn [EntityWorldUiNode]
/// 
/// [plugin_for_t]`::<SpawnWorldUiNodeMsgA,WorldCameras>`
#[derive(Message)]
pub struct SpawnWorldUiNodeMsgA{
    pub entity:Entity,
    pub on_fn:Box<dyn Fn(&Entity,&mut Commands)+Send+Sync>
}

//impl SpawnEntityMsg for SpawnWorldUiNodeMsgA {
//    fn entity(&self)->Entity {
//        self.entity
//    }
//}

impl MsgForEntity for SpawnWorldUiNodeMsgA {
	fn entity(&self)->Entity {
		self.entity
	}
}

//impl<'a> Has<MsgForEntity> for &'a SpawnWorldUiNodeMsgA {
//	fn get_has(self,marker:MsgForEntity)->Entity {
//		self.entity
//	}
//}

//pub struct MsgSpawnWorldUiNode<'a>(&'a Entity,&'a mut Commands<'a,'a>);
//
//impl<'a> HasMarker for MsgSpawnWorldUiNode<'a>
//{
//	type Item=();
//}

pub trait MsgSpawnWorldUiNode {
	fn on_spawn_world_ui_node(&self,entity:&Entity,cmd:&mut Commands);
}

impl MsgSpawnWorldUiNode for SpawnWorldUiNodeMsgA {
	fn on_spawn_world_ui_node(&self,entity:&Entity,cmd:&mut Commands){
		(self.on_fn)(entity,cmd);
	}
}

//impl<'a> Has<MsgSpawnWorldUiNode<'a>> for &'a SpawnWorldUiNodeMsgA {
//	fn get_has(self,marker:MsgSpawnWorldUiNode<'a>)->() {
//		(self.on_fn)(marker.0,marker.1);
//	}
//}



/// [SystemSet] for [spawn_entity_transform_mark_ui_node]
#[derive(SystemSet,Clone, Copy,PartialEq,Eq,Hash,Default,Debug)]
pub struct SystemSetSpawnWorldUiNode;

system_set_for_t!(SystemSetSpawnWorldUiNodeT);

pub fn create_world_ui_node(
    base_entity:Entity,
    camera:Entity,
    cmd:&mut Commands
)->Entity{
    use bevy::prelude::*;
    let node=cmd.spawn((
        (
            Node {
                width: px(0),
                height: px(0),
                left:px(0),
                top:px(0),
                overflow:Overflow::visible(),
                justify_content: bevy::ui::JustifyContent::Center,
                align_items: bevy::ui::AlignItems::Center,
                position_type: bevy::ui::PositionType::Absolute,
                ..default()
            },
            UiTransform::default(),
            UiTargetCamera(camera),
            SetUiWorldToViewPositionFromRelationGlobalTransformUiTargetCamera::<EntityWorldUiNode>::default(),
        ),
    )).id();
    
    cmd.entity(base_entity).add_one_related::<EntityWorldUiNode>(node);
    node
}

/// Spawn [EntityWorldUiNode] for entities given by `TMsg` and cameras given by `CamerasRes`
pub fn spawn_world_ui_node<TMsg,CamerasRes>(
    mut mr:MessageReader<TMsg>, 
    p_cmd:ParallelCommands,
    cameras:Res<CamerasRes>
)
    where TMsg:Message + MsgForEntity + MsgSpawnWorldUiNode,
		//for<'a> &'a TMsg : Has<MsgSpawnWorldUiNode<'a>>,
        // CameraFilter:QueryFilter
        CamerasRes: Resource,
        for<'a>&'a CamerasRes: IntoIterator<Item = &'a Entity>,
{
    mr.par_read().for_each(|m|{
		let base_entity=m.entity();
        (&cameras).into_iter().for_each(|camera|{
            
            p_cmd.command_scope(|mut cmd|{
                let node=create_world_ui_node(base_entity,*camera,&mut cmd);
                //m.on(&node, &mut cmd);
				//on_fn(&node, &mut cmd);
				//m.get_has(MsgSpawnWorldUiNode(&node,&mut cmd));
				m.on_spawn_world_ui_node(&node, &mut cmd);
                // cmd.write_message(m.on(&node,src_entity));
            });
        });
    });
	
    // bevy::ecs::relationship::RelationshipSourceCollection
    // UiTargetCamera
}

/// For `TMsg` and `CamerasRes`, add system [spawn_entity_transform_mark_ui_node]`::<TMsg,CamerasRes>` and in set [SpawnEntityTransformMarkUiNodeSystemSet]`::<TMsg>`
pub fn plugin_for_t<TMsg,CamerasRes>(app:&mut App) 
    where TMsg:Message + MsgForEntity + MsgSpawnWorldUiNode,
		//for<'a> &'a TMsg : Has<MsgSpawnWorldUiNode<'a>>,
        CamerasRes: Resource,
        for<'a>&'a CamerasRes: IntoIterator<Item = &'a Entity>
{
    app.add_systems(Update, (spawn_world_ui_node::<TMsg,CamerasRes>,)
        .in_set(SystemSetSpawnWorldUiNodeT::<TMsg>::default())
        .in_set(SystemSetSpawnWorldUiNode));
}

pub struct Plugins;

impl bevy::prelude::PluginGroup for Plugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(super::set_ui_transform_to_screen_ui_node::plugin_for_t::<SetUiWorldToViewPositionFromRelationGlobalTransformUiTargetCamera::<EntityWorldUiNode>>)
            .add(plugin_default)
    }
}

pub fn plugin_default(app:&mut App){
    
	app.add_message::<SpawnWorldUiNodeMsgA>();
	app.add_plugins(plugin_for_t::<SpawnWorldUiNodeMsgA,WorldCameras>);
}