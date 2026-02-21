

use std::ops::AddAssign;

use bevy::ecs::{query::With, system::Query};

use crate::stat_component::{change::Change, determining::Determining, stat::Stat};

/// for each [`Stat<T>`] and [`Change<T>`] with [`Determining<T>`], apply changes and reset [`Change<T>`].
pub fn determining_apply_changes<T>(mut query:Query<(&mut Stat<T>,&mut Change<T>),With<Determining<T>>>)
    where 
        T:Default+AddAssign + std::marker::Send + std::marker::Sync+'static
{
    (&mut query).par_iter_mut().for_each(|(mut value,mut delta)|{
        **value += delta.get_and_reset();
    });
}

/// for each [`Stat<TStat>`] and [`Change<TChange>`] with [`Determining<TStat>`], apply changes and reset [`Change<TChange>`].
pub fn determining_apply_changes_2<TStat,TChange>(mut query:Query<(&mut Stat<TStat>,&mut Change<TChange>),With<Determining<TStat>>>)
    where 
        //T:Deref<Target : AddAssign+Sized>+DerefMut+Into<T::Target>+ Send+ Sync+'static+Default
        TStat:Default+AddAssign<TChange> + std::marker::Send + std::marker::Sync+'static,
        TChange:Default+AddAssign<TChange> + std::marker::Send + std::marker::Sync+'static,
{
    (&mut query).par_iter_mut().for_each(|(mut value,mut delta)|{
        **value += delta.get_and_reset();
    });
}
