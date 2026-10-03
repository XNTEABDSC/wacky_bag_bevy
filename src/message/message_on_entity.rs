use bevy::{ecs::{component::Component, entity::Entity, system::Query}, log::error, utils::Parallel};

use crate::message::owned_message::{OwnedMessage, ParOwnedMessages};
#[derive(Default,Component)]
pub struct MessagesOnEntity<T:Send+Sync+'static>{
	msgs:Parallel<Vec<T>>
}

pub struct MessageOnEntity<T:Send+Sync+'static>{
	pub entity:Entity,
	pub msg:T
}

pub type MessagesOnEntityRes<T> = ParOwnedMessages<MessageOnEntity<T>>;

impl<T:Send+Sync+'static> OwnedMessage for MessageOnEntity<T> {
	type SystemParam=(Query<'static,'static, &'static MessagesOnEntity<T>>,);

	fn apply_self<'w,'s>(self,q:&<Self::SystemParam as bevy::ecs::system::SystemParam>::Item<'w,'s>) {
		let Ok(moe)=q.0.get(self.entity) else {
			error!("entity {} dont matches query {:?}",self.entity,q);
			return;
		};
		moe.msgs.borrow_local_mut().push(self.msg);
	}
}