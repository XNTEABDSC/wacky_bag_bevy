use std::sync::Mutex;

use bevy::{ecs::{message::MessageWriter, resource::Resource, system::{Res, ResMut, SystemParam, SystemParamItem}}, tasks::ComputeTaskPool, utils::Parallel};

// use crate::utils::par_simple_map::ParSimpleMap;


/// unlike [`bevy::ecs::message::Message`] OwnedMessage is ensured to be processed by only one system, thus ownership is provided.
pub trait OwnedMessage:Sized {
	type SystemParam:SystemParam;
	fn apply_self(self,p:& SystemParamItem<Self::SystemParam>);
}

/// Resource storing all `T:`[`OwnedMessage`]
#[derive(Resource)]
pub struct ParOwnedMessages<T:OwnedMessage+Send+Sync+'static>{
	pub msgs:Parallel<Vec<T>>
}

impl<T: OwnedMessage + Send + Sync + 'static> Default for ParOwnedMessages<T> {
    fn default() -> Self {
		Self { msgs: Default::default() }
	}
}


impl<T:OwnedMessage+Send+Sync+'static> ParOwnedMessages<T> {
	pub fn write(&self,msg:T){
		self.msgs.borrow_local_mut().push(msg);
	}

	pub fn write_batch(&self,msgs:impl IntoIterator<Item = T>){
		self.msgs.borrow_local_mut().extend(msgs);
	}
}

pub fn apply_owned_message_parallel<T>(mut a:ResMut<ParOwnedMessages<T>>, p: SystemParamItem<T::SystemParam>)
where T:OwnedMessage+Send+Sync+'static,
	for<'w,'s> SystemParamItem<'w,'s,T::SystemParam>: Sync
	// <<T as OwnedMessage>::SystemParam as SystemParam>::Item<'w, 's>: Sync
{
	ComputeTaskPool::get().scope(|s|{
		a.msgs.iter_mut().for_each(|msgs|{
			s.spawn(async {
				msgs.drain(..).for_each(|msg|msg.apply_self(&p));
			});
		});
	});
}

///
#[derive(SystemParam)]
pub struct ParOwnedMessageWriter<'w,T:OwnedMessage+Send+Sync+'static>
{
	msgs:Res<'w,ParOwnedMessages<T>>
}

impl<'w,T:OwnedMessage+Send+Sync+'static> ParOwnedMessageWriter<'w,T> {
	pub fn write(&self,msg:T){
		self.msgs.msgs.borrow_local_mut().push(msg);
	}

	pub fn write_batch(&self,msgs:impl IntoIterator<Item = T>){
		self.msgs.msgs.borrow_local_mut().extend(msgs);
	}
}