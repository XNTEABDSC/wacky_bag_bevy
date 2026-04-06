pub mod position_to_transform;

use std::marker::PhantomData;

use bevy::{app::{PluginGroup, PluginGroupBuilder}, asset::{AssetPath, AssetServer}, ecs::{bundle::Bundle, system::Res}, math::Vec3, sprite::Sprite, transform::components::Transform};
use nalgebra::RealField;


pub struct SpawnEntityVisualData<'a>{
	pub img_path:AssetPath<'a>,
	pub scale:Vec3
}

impl<'a> SpawnEntityVisualData<'a> {
	pub fn to_bundle(&'a self,asset_server: Res<AssetServer>)->impl Bundle{
		let img_path=self.img_path.clone();
		let scale=self.scale;
		let img=asset_server.load(img_path);
		(
			Sprite::from_image(img),
			Transform::from_scale(scale)
		)
	}
}

pub struct VisualPluginGroup<Num>{
	pub p:PhantomData<Num>
}

impl<Num:RealField+Copy> PluginGroup for VisualPluginGroup<Num> {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
        //.add(VisualPluginMark)
        .add(position_to_transform::plugin::<Num>)
    }
}