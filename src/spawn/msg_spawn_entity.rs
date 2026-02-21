use bevy::ecs::entity::Entity;
//use wacky_bag::traits::has::HasMarker;

//#[derive(Default,Debug,Clone, Copy)]
//pub struct MsgForEntity;
//
//impl HasMarker for MsgForEntity {
//	type Item=Entity;
//}


pub trait MsgForEntity{
	fn entity(&self)->Entity;
}