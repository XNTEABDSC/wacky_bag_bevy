use std::fmt::Debug;

use bevy::ecs::{query::{QueryData, QueryFilter}, system::Query};



pub fn query_debug<D,F>(q:Query<D,F>)
where F:QueryFilter,
	D:QueryData,
	for<'w,'s> <<D as QueryData>::ReadOnly as QueryData>::Item<'w, 's>:Debug
{
	q.iter().for_each(|a|{
		println!("{:?}",a);
	});
}