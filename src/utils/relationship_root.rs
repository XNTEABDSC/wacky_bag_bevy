use std::marker::PhantomData;

use bevy::{ecs::{change_detection::MaybeLocation, component::Component, entity::Entity, lifecycle::HookContext, query::{Changed, QueryItem}, relationship::{Relationship, RelationshipTarget}, system::SystemParamItem, world::DeferredWorld}, log::error, utils::prelude::DebugName};
use crate::{system::propagate_relationship::{PropagateRootToLeafMut, PropagateRootToLeafMutBeginSysParam, PropagateRootToLeafMutProcessSysParam}, utils::system_param_with_query::{SystemParamWithQuery, SystemParamWithQueryMergeT, SystemParamWithQueryT}};

#[derive(Debug,Component)]
pub struct RelationshipRoot<R>{
    pub entity:Entity,
    pub p:PhantomData<R>,
}

impl<R> RelationshipRoot<R>
where R:Relationship+Send+Sync
{
    pub fn get_root_of(world:&DeferredWorld,entity:Entity)->Result<Entity,String>{
        let tar_entity = world.entity(entity).get::<R>().ok_or_else(||
            format!(
                "The {entity:?} should have Relationship {} but does not.",
                DebugName::type_name::<R>()
            )
        )?.get();

        let tar_root = world.entity(tar_entity).get::<Self>();
        let tar_rel=world.entity(tar_entity).get::<R>().is_some();
        if tar_rel && let Some(tar_root)=tar_root {
            Ok(tar_root.entity)
        }else if !tar_rel && tar_root.is_none() {
            Ok(tar_entity)
        }else {
            Err(
                format!(
                    "The {tar_entity:?} should either have both {r_name} and {s_name} or have none of them, but actually {have_r}have {r_name} and {have_s}have {s_name}.",
                    r_name=DebugName::type_name::<R>(),
                    s_name=DebugName::type_name::<Self>(),
                    have_r=if tar_rel {""}else{"dont "},
                    have_s=if tar_root.is_some() {""}else{"dont "}
                )
            )
        }
    }

    pub fn update_root(world:&mut DeferredWorld,entity:Entity,root_entity:Entity)->bool{
        world.entity_mut(entity).get_mut::<Self>().map(|mut r|{r.entity=root_entity}).is_some()
    }

    pub fn update_root_propagate(world:&mut DeferredWorld,entity:Entity,root_entity:Entity){
        if Self::update_root(world, entity, root_entity) {
            if let Some(rts) = world.entity(entity).get::<R::RelationshipTarget>() {
                for ce in rts.iter().collect::<Vec<_>>(){
                    Self::update_root_propagate(world,ce,root_entity);
                }
            }
        }
    }
    
    pub fn on_insert(
        mut world:DeferredWorld,
        HookContext {
            entity,
            caller,
            ..
        }: HookContext){
        let res: Result<Entity, String>=Self::get_root_of(&world,entity);

        match res {
            Ok(root_entity) => {
                // world.entity_mut(entity).get_mut::<Self>().unwrap().entity=root_e;
                Self::update_root_propagate(&mut world,entity,root_entity);
            },
            Err(err_msg) => {
                error!("{} {} The invalid {} has been removed.",
                    caller.map(|location|format!("{location}: ")).unwrap_or_default(),    
                    err_msg,
                    DebugName::type_name::<Self>()
                );
                world.commands().entity(entity).remove::<Self>();
            },
        }

        // let relation = world.entity(entity).get::<R>();
        // let Some(relation) = relation else {
        //     error!(
        //         warn!(
                    // "{}The {}({target_entity:?}) relationship on entity {entity:?} relates to an entity that does not exist. The invalid {} relationship has been removed.",
                    // caller.map(|location|format!("{location}: ")).unwrap_or_default(),
                    // DebugName::type_name::<Self>(),
                    // DebugName::type_name::<Self>()
        //         );
        //         world.commands().entity(entity).remove::<Self>();
        //     );
        // };

    }
}

// // impl<R> Component for RelationshipRoot<R>
// // where R:Relationship+Send+Sync
// // {
// //     const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;

// //     type Mutability = bevy::ecs::component::Mutable;
// // }

// #[deprecated = "RelationshipRoot handles this by component hook"]
// #[derive(Debug,Clone, Copy)]
// pub struct PropagateRelationshipRoot(Entity);

// impl<R:Relationship> PropagateRootToLeafMut<R> for PropagateRelationshipRoot {
//     type BeginSysParam=SystemParamWithQueryT<
//         (Entity,),
//         (Changed<R>,),
//         ()
//     >
//         ;

//     fn process_data_begin(
//             values: 
//                 QueryItem<
//                     <SystemParamWithQueryMergeT<PropagateRootToLeafMutBeginSysParam<R>, Self::BeginSysParam> as SystemParamWithQuery>::D
//                 >,
//             _others: 
//                 &SystemParamItem<
//                     <SystemParamWithQueryMergeT<PropagateRootToLeafMutBeginSysParam<R>, Self::BeginSysParam> as SystemParamWithQuery>::P
//                 >
//         )->
//         // Vec<(Entity,Self)>
//         impl FnMut(Entity)->Self
//         {
//         return move |_|PropagateRelationshipRoot(values.1.0);
//     }

//     type ProcessSysParam=SystemParamWithQueryT<
//         (&'static mut RelationshipRoot<R>,),
//         (Changed<R>,),
//         ()
//     >;

//     fn process_data<'w,'s>(
//             self,
//             mut values: 
//                 QueryItem<
//                     <SystemParamWithQueryMergeT<PropagateRootToLeafMutProcessSysParam<R>, Self::ProcessSysParam> as SystemParamWithQuery>::D
//                 >,
//             _others: 
//                 &SystemParamItem<
//                     <SystemParamWithQueryMergeT<PropagateRootToLeafMutProcessSysParam<R>, Self::ProcessSysParam> as SystemParamWithQuery>::P
//                 >)
//         ->
//         // Vec<(Entity,Self)>
//         impl FnMut(Entity)->Self
//         {
//         values.1.0.entity=self.0;
//         return move|_|self.clone();
//     }
// }