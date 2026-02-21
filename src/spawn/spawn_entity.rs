
use std::{fmt::Debug, ops::{Deref, DerefMut}};

use bevy::{ecs::{entity::Entity, message::{Message, MessageMutator, MessageReader}, system::{EntityCommands, ParallelCommands, SystemParam}}, log::warn};
use wacky_bag::traits::has::{Has, HasMarker, HasMut};




pub fn spawn_entity_for_msg<TMsg>(mut mm:MessageMutator<TMsg>,p_cmd:ParallelCommands)
	where TMsg:for<'a> HasMut<'a,MsgSpawnEntity>+Message+Debug
{
	mm.par_read().for_each(|m|{
		let mut b=m.get_mut(MsgSpawnEntity);
		let m_e=b.deref_mut();
		if let Some(e)=m_e{
			let e_str=e.to_string();
			drop(b);
			warn!("spawn msg {:?} already has entity id {}",m,e_str);
		}else {
			p_cmd.command_scope(|mut cmd|{
				*m_e=Some(cmd.spawn_empty().id());
			})
		}
	});
}

pub const fn for_each_spawn_entity_system<TSpawnMessage,ExtraParam,F>(op_fn:F)->
    impl Fn((MessageReader<'_, '_, TSpawnMessage>, ParallelCommands<'_, '_>, ExtraParam))

    where TSpawnMessage:for<'a> Has<'a,MsgSpawnEntity>+Message+Debug,
    F:Fn(&mut EntityCommands,&TSpawnMessage,&ExtraParam),
    F:Sync,
    ExtraParam:Sync+SystemParam,
{

    let a=
    move |(mut er,p_cmd,extra)
    :(MessageReader<'_, '_, TSpawnMessage>, ParallelCommands<'_, '_>,ExtraParam)|{
        er.par_read().for_each(|event|{
            p_cmd.command_scope(|mut cmd|{
				let entity_may=event.get(MsgSpawnEntity);
				if let Some(entity)=entity_may.deref() {
					let Ok(mut entity_cmd)=cmd.get_entity(*entity)else{
						warn!("spawn msg {:?} entity {} not found",event, entity);
						return;};
					op_fn(&mut entity_cmd,event,&extra);
					
				}else {
					warn!("spawn msg {:?} dont have entity", event);
				}
            });
            
        });
    };
    return a;
}