

use std::ops::{AddAssign, Deref, DerefMut};

use bevy::{app::{App, FixedPostUpdate, FixedPreUpdate}, ecs::{query::With, schedule::IntoScheduleConfigs, system::Query}};
use frunk::{HList, HNil};

use crate::{stat_component::{change::Change, determining::Determining, stat::Stat}, system::processing_system::ScheduleConfigsProcessing};

pub fn stat_apply_change<TStat,TChange,S,C>(mut change:C,mut stat:S)
	where 
		TStat:Default+AddAssign<TChange>,
        TChange:Default+AddAssign<TChange>,
		S:Deref<Target = Stat<TStat>>+DerefMut,
		C:Deref<Target = Change<TChange>>+DerefMut
{
	**stat += change.get_and_reset();
}

pub fn change_apply_change<TChange,CM,CR>(mut source:CM,target:CR)
	where 
        TChange:Default+AddAssign<TChange>,
		CM:Deref<Target = Change<TChange>>+DerefMut,
		CR:Deref<Target = Change<TChange>>
{
	target.add_change(source.get_and_reset());
}

/// for each [`Stat<T>`] and [`Change<T>`] with [`Determining<T>`], apply changes and reset [`Change<T>`].
pub fn determining_apply_changes<T>(mut query:Query<(&mut Stat<T>,&mut Change<T>),With<Determining<T>>>)
    where 
        T:Default+AddAssign + std::marker::Send + std::marker::Sync+'static
{
    (&mut query).par_iter_mut().for_each(|(stat,change)|{
        stat_apply_change(change,stat);
    });
}

pub fn determining_apply_changes_plugin<T>(app:&mut App)
    where 
        T:Default+AddAssign + std::marker::Send + std::marker::Sync+'static
{
	app.add_systems(FixedPostUpdate, determining_apply_changes::<T>.into_configs()
		.config_processing::<HNil,HNil,HList!(Stat<T>,Change<T>)>()
	);
}

/// for each [`Stat<TStat>`] and [`Change<TChange>`] with [`Determining<TStat>`], apply changes and reset [`Change<TChange>`].
pub fn determining_apply_changes_2<TStat,TChange>(mut query:Query<(&mut Stat<TStat>,&mut Change<TChange>),With<Determining<TStat>>>)
    where 
        //T:Deref<Target : AddAssign+Sized>+DerefMut+Into<T::Target>+ Send+ Sync+'static+Default
        TStat:Default+AddAssign<TChange> + std::marker::Send + std::marker::Sync+'static,
        TChange:Default+AddAssign<TChange> + std::marker::Send + std::marker::Sync+'static,
{
    (&mut query).par_iter_mut().for_each(|(stat,change)|{
        // **value += delta.get_and_reset();
		stat_apply_change(change,stat);
    });
}
pub fn determining_apply_changes_2_plugin<TStat,TChange>(app:&mut App)
    where 
        TStat:Default+AddAssign<TChange> + std::marker::Send + std::marker::Sync+'static,
        TChange:Default+AddAssign<TChange> + std::marker::Send + std::marker::Sync+'static,
{
	app.add_systems(FixedPostUpdate, 
		determining_apply_changes_2::<TStat,TChange>.into_configs()
		.config_processing::<HNil,HNil,HList!(Stat<TStat>,Change<TChange>)>()
	);
}