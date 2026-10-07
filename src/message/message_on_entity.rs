use bevy::{ecs::{component::Component, entity::Entity, system::{Query, SystemParam}}, log::error, tasks::ComputeTaskPool, utils::Parallel};

use crate::message::owned_message::{OwnedMessage, ParOwnedMessageWriter, ParOwnedMessages};
#[derive(Component)]
pub struct MessagesOnEntity<T:Send+Sync+'static>{
	msgs:Parallel<Vec<T>>
}

impl<T:Send+Sync+'static> Default for MessagesOnEntity<T> {
	fn default() -> Self {
		Self { msgs: Default::default() }
	}
}

pub struct MessageOnEntity<T:Send+Sync+'static>{
	pub entity:Entity,
	pub msg:T
}

pub type MessagesOnEntityRes<T> = ParOwnedMessages<MessageOnEntity<T>>;

pub type MessagesOnEntityWriter<'a,T> = ParOwnedMessageWriter<'a,MessageOnEntity<T>>;

impl<T:Send+Sync+'static> MessagesOnEntity<T>  {
	pub fn par_for_each<F>(&mut self,f:&F)
	where F:Fn(T)+Sync
	{
		ComputeTaskPool::get().scope(|s|{
			self.msgs.iter_mut().for_each(|msgs|{
				s.spawn(async {
					msgs.drain(..).for_each(f);
				});
			});
		});
	}

	pub fn drain(&mut self)->impl Iterator<Item = T>{
		self.msgs.iter_mut().flat_map(|v|v.drain(..))
	}
}

impl<T:Send+Sync+'static> OwnedMessage for MessageOnEntity<T> {
	type SystemParam=(Query<'static,'static, &'static MessagesOnEntity<T>>,);
	
	fn apply_self(self,q:& bevy::ecs::system::SystemParamItem<Self::SystemParam>) {
			let Ok(moe)=q.0.get(self.entity) else {
			error!("entity {} dont matches query {:?}",self.entity,q);
			return;
		};
		moe.msgs.borrow_local_mut().push(self.msg);
	}
}