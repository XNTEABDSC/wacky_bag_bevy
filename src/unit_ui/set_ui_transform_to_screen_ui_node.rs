//! [set_ui_world_to_view_position]
//! 
//! [SetUiWorldToViewPosition]
//! 
//! 


use std::{any::{type_name_of_val}, fmt::Debug, marker::PhantomData};

use bevy::{app::{App, Update}, camera::Camera, ecs::{component::Component, entity::Entity, query::{QueryData, QueryEntityError, QueryItem, Without}, schedule::{IntoScheduleConfigs, SystemSet}, system::{IntoSystem, Query, SystemParam, SystemParamItem}}, log::warn, math::{Vec2, Vec3}, transform::components::GlobalTransform, ui::{UiTargetCamera, UiTransform, Val2}};

use crate::utils::world_scale_on_screen::world_to_view_scale;

use bevy::ecs::relationship::Relationship;

/// Calculate UiTransform to make Ui shows at world position.
pub trait SetUiWorldToViewPosition {
    type QueryData:QueryData;
    type SystemParams:SystemParam+Sync
        // where for<'w,'s> <Self::SystemParams as SystemParam>::Item<'w,'s> : Send+Sync
        ;
    /// To get datas to calculate UiTransform. 
    fn get_datas(
        &self,
        query_data:QueryItem<'_,'_,Self::QueryData>,
        sys_data:& SystemParamItem<'_,'_,Self::SystemParams> )
        ->Result<SetUiWorldToViewPositionData,Self::Err>;

    type Err:Debug;
}

#[derive(Component,Clone, Copy)]
pub struct SetUiWorldToViewPositionData{
    pub camera:Entity,
    pub wld_pos:Vec3
}

impl SetUiWorldToViewPosition for SetUiWorldToViewPositionData {
    type QueryData = ();

    fn get_datas(
        &self,
        _query_data:QueryItem<'_,'_,Self::QueryData>,
        _sys_data:& SystemParamItem<'_,'_,Self::SystemParams> )
        ->Result<SetUiWorldToViewPositionData,Self::Err>
    {
        Ok(*self)
    }
    
    type SystemParams =();
    type Err = ();

}


#[derive(Component)]
pub struct SetUiWorldToViewPositionFromGlobalTransformUiTargetCamera;

impl SetUiWorldToViewPosition for SetUiWorldToViewPositionFromGlobalTransformUiTargetCamera {
    type QueryData=(&'static GlobalTransform,&'static UiTargetCamera);

    fn get_datas(&self,data:<Self::QueryData as QueryData>::Item<'_,'_>,_:&Self::SystemParams)->Result<SetUiWorldToViewPositionData,Self::Err> {
        Ok(SetUiWorldToViewPositionData{
            camera:(*data.1).0,
            wld_pos:data.0.translation(),
        })
    }
    
    type SystemParams=();
    type Err = ();
}


#[derive(Component)]
pub struct SetUiWorldToViewPositionFromUiTargetCamera{
    pub wld_pos:Vec3
}

impl SetUiWorldToViewPosition for SetUiWorldToViewPositionFromUiTargetCamera {
    type QueryData=(&'static UiTargetCamera,);

    fn get_datas(&self,data:<Self::QueryData as QueryData>::Item<'_,'_>,_:&Self::SystemParams)->Result<SetUiWorldToViewPositionData,Self::Err> {
        Ok(SetUiWorldToViewPositionData{
            camera:(*data.0).0,
            wld_pos:self.wld_pos,
        })
    }
    
    type SystemParams=();
    type Err = ();
}

/// Set UiTransform via [GlobalTransform] of related entity and [UiTargetCamera]
#[derive(Component)]
pub struct SetUiWorldToViewPositionFromRelationGlobalTransformUiTargetCamera<TRelation>(pub PhantomData<TRelation>);

impl<TRelation> SetUiWorldToViewPosition for SetUiWorldToViewPositionFromRelationGlobalTransformUiTargetCamera<TRelation>
    where TRelation:Relationship+Component
{
    type QueryData=(&'static UiTargetCamera,&'static TRelation);
    type SystemParams = (Query<'static,'static,(&'static GlobalTransform,),(Without<SetUiWorldToViewPositionFromRelationGlobalTransformUiTargetCamera<TRelation>>,Without<Camera>)>,);
    fn get_datas(
        &self,
        query_data:QueryItem<'_,'_,Self::QueryData>,
        sys_data:& SystemParamItem<'_,'_,Self::SystemParams> )
        ->Result<SetUiWorldToViewPositionData,Self::Err>
    {
        let tar_entity=query_data.1.get();
        let tar_entity_q=sys_data.0;
        let g_trans=tar_entity_q.get(tar_entity)?;
        let wld_pos=g_trans.0.translation();
        Ok(SetUiWorldToViewPositionData{
            camera:(*query_data.0).0,
            wld_pos,
        })
    }
    type Err = QueryEntityError;
}

impl<TRelation> Default for SetUiWorldToViewPositionFromRelationGlobalTransformUiTargetCamera<TRelation> {
    fn default() -> Self { Self(Default::default())}
}

#[derive(SystemSet,Default,Hash,Debug,PartialEq,Eq,Clone, Copy)]
pub struct SetUiWorldToViewSystemSet;

pub struct SetUiWorldToViewSys<T>(pub PhantomData<T>);

impl<T> Default for SetUiWorldToViewSys<T>{
    fn default() -> Self { Self(Default::default()) }
}

pub fn set_ui_world_to_view<T:SetUiWorldToViewPosition+Component>(
    mut nodes:Query<'_,'_,(&'_ mut UiTransform,&'_ T, 
        <T as SetUiWorldToViewPosition>::QueryData), (Without<Camera>,)>, //(&'w Transform,&'w UiTargetCamera)
    cameras:Query<'_,'_,(&'_ Camera,&'_ GlobalTransform),(Without<T>,)>,
    others_sys_param: <T::SystemParams as SystemParam>::Item<'_,'_>
)
    where for<'w2, 's2> <<T as SetUiWorldToViewPosition>::SystemParams as SystemParam>::Item<'w2, 's2>: Sync
{
    nodes.par_iter_mut().for_each(|(mut ui_transform,t,
        t_data)|{
        let SetUiWorldToViewPositionData{camera: camera_entity,wld_pos:ui_wld_pos}=match t.get_datas(t_data,&others_sys_param){
            Ok(a)=>a,
            Err(err)=>{
                warn!("{:?}",err);
                return;
            }
        };
        let Ok((camera_camera,camera_transform))=cameras.get(camera_entity) else{
            warn!("entity {} not found in {}",camera_entity,type_name_of_val(&cameras));
            return;
        };


        {
            
            let res_may=camera_camera.world_to_viewport(camera_transform, ui_wld_pos);
            if let Ok(res)=res_may{
                ui_transform.translation=Val2::px(res.x, res.y);
            }
        }
        {
            
            let res_may=world_to_view_scale(camera_camera,camera_transform,ui_wld_pos);
            if let Some(res)=res_may{
                ui_transform.scale=Vec2::ONE*res;
            }
        }

    });
}

impl<T> bevy::prelude::SystemParamFunction<
        SetUiWorldToViewSys<T>
    > 
    for SetUiWorldToViewSys<T>
    where T:SetUiWorldToViewPosition+Component,
        for<'w2, 's2> <<T as SetUiWorldToViewPosition>::SystemParams as SystemParam>::Item<'w2, 's2>:Sync
{

    type In = ();
    type Out = ();
    type Param = (Query<'static,'static,(&'static mut UiTransform,&'static T, 
        <T as SetUiWorldToViewPosition>::QueryData), (Without<Camera>,)>, //(&'w Transform,&'w UiTargetCamera)
    Query<'static,'static,(&'static Camera,&'static GlobalTransform),(Without<T>,)>,
    T::SystemParams);
    fn run(
            &mut self,
            _input:(),
            param_value: bevy::ecs::system::SystemParamItem<Self::Param>,
        ) -> () {
        let (nodes,cameras,others)=param_value;
        set_ui_world_to_view(nodes,cameras,others);
    }
}



pub fn plugin_for_t<T>(app:&mut App)
    where T:SetUiWorldToViewPosition+Component,
        for<'w2, 's2> <<T as SetUiWorldToViewPosition>::SystemParams as SystemParam>::Item<'w2, 's2>: Sync
{
    let set_ui_world_to_view_system=
        IntoSystem::into_system(SetUiWorldToViewSys::<T>::default());
		
	app.add_systems(Update, (
			set_ui_world_to_view_system,
		).in_set(SetUiWorldToViewSystemSet));
}

pub fn plugin_default(app:&mut App){
    app.add_plugins((
        plugin_for_t::<SetUiWorldToViewPositionData>,
        plugin_for_t::<SetUiWorldToViewPositionFromGlobalTransformUiTargetCamera>,
    ));
}