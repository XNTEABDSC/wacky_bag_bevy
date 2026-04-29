

use std::ops::{AddAssign, Deref, DerefMut};

use bevy::{app::{App, FixedPostUpdate, FixedPreUpdate}, ecs::{query::With, schedule::IntoScheduleConfigs, system::Query}};
use frunk::{HList, HNil};

use crate::{stat_component::{change_generic::{AlgebraicSystem, ChangeGeneric}, determining::Determining, stat::Stat}, system::processing_system::ScheduleConfigsProcessing};

pub fn stat_apply_change<T,OP,C,S>(mut change:C,mut stat:S)
	where OP:AlgebraicSystem<T>,
		S:Deref<Target = Stat<T>>+DerefMut,
		C:Deref<Target = ChangeGeneric<T,OP>>+DerefMut
{
	// **stat += change.get_and_reset();
	change.apply_to(&mut stat);
}

pub fn change_apply_change<T,OP,CM,CR>(mut source:CM,target:CR)
	where OP:AlgebraicSystem<T>,
		CM:Deref<Target = ChangeGeneric<T,OP>>+DerefMut,
		CR:Deref<Target = ChangeGeneric<T,OP>>
{
	target.add_change(source.get_and_reset());
}

/// for each [`Stat<T>`] and [`Change<T>`] with [`Determining<T>`], apply changes and reset [`Change<T>`].
pub fn determining_apply_changes<T,OP>(mut query:Query<(&mut Stat<T>,&mut ChangeGeneric<T,OP>),With<Determining<T>>>)
    where 
        T:Send + Sync + 'static,
		OP:AlgebraicSystem<T> + Send + Sync + 'static
{
    (&mut query).par_iter_mut().for_each(|(stat,change)|{
        stat_apply_change(change,stat);
    });
}

pub fn determining_apply_changes_plugin<T,OP>(app:&mut App)
    where 
        T:Send + Sync + 'static,
		OP:AlgebraicSystem<T> + Send + Sync + 'static
{
	app.add_systems(FixedPostUpdate, determining_apply_changes::<T,OP>.into_configs()
		.config_processing::<HNil,HNil,HList!(Stat<T>,ChangeGeneric<T,OP>)>()
	);
}

// /// for each [`Stat<TStat>`] and [`Change<TChange>`] with [`Determining<TStat>`], apply changes and reset [`Change<TChange>`].
// pub fn determining_apply_changes_2<TStat,TChange>(mut query:Query<(&mut Stat<TStat>,&mut Change<TChange>),With<Determining<TStat>>>)
//     where 
//         //T:Deref<Target : AddAssign+Sized>+DerefMut+Into<T::Target>+ Send+ Sync+'static+Default
//         TStat:Default+AddAssign<TChange> + std::marker::Send + std::marker::Sync+'static,
//         TChange:Default+AddAssign<TChange> + std::marker::Send + std::marker::Sync+'static,
// {
//     (&mut query).par_iter_mut().for_each(|(stat,change)|{
//         // **value += delta.get_and_reset();
// 		stat_apply_change(change,stat);
//     });
// }
// pub fn determining_apply_changes_2_plugin<TStat,TChange>(app:&mut App)
//     where 
//         TStat:Default+AddAssign<TChange> + std::marker::Send + std::marker::Sync+'static,
//         TChange:Default+AddAssign<TChange> + std::marker::Send + std::marker::Sync+'static,
// {
// 	app.add_systems(FixedPostUpdate, 
// 		determining_apply_changes_2::<TStat,TChange>.into_configs()
// 		.config_processing::<HNil,HNil,HList!(Stat<TStat>,Change<TChange>)>()
// 	);
// }