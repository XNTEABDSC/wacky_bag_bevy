use bevy::{app::App, ecs::{entity::Entity, resource::Resource}, reflect::Reflect};


#[derive(Resource,Default,Debug,Reflect)]
pub struct WorldCameras(pub Vec<Entity>);

impl<'a> IntoIterator for &'a WorldCameras {
    type Item=&'a Entity;

    type IntoIter=<&'a Vec::<Entity> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

pub fn plugin(app:&mut App){
    app.init_resource::<WorldCameras>();
}