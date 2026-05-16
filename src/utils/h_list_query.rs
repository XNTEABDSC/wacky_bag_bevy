use bevy::ecs::query::{Access, QueryData, ReadOnlyQueryData, ReleaseStateQueryData, WorldQuery};
use frunk::{HCons, HNil};


/// allows to use hlist as QueryData
// pub struct HQueryData<HL>(pub HL);
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct HQueryCons<H,T>(pub HCons<H,T>);
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct HQueryNil(pub HNil);

pub trait HToQuery {
	type Output;
	fn to_query(self)->Self::Output;
}

pub type HToQueryType<T>=<T as HToQuery>::Output;

impl HToQuery for HNil {
	type Output=HQueryNil;

	fn to_query(self)->Self::Output {
		HQueryNil(HNil)
	}
}

impl<H,T> HToQuery for HCons<H,T>
	where T:HToQuery
{
	type Output=HQueryCons<H,T::Output>;

	fn to_query(self)->Self::Output {
		HQueryCons(HCons { head: self.head, tail: self.tail.to_query() })
	}
}

/// impl following impl WorldQuery for Entity
unsafe impl WorldQuery for HQueryNil {
	type Fetch<'w>=();

	type State=();

	fn shrink_fetch<'wlong: 'wshort, 'wshort>(_fetch: Self::Fetch<'wlong>) -> Self::Fetch<'wshort> {
		
	}

	unsafe fn init_fetch<'w, 's>(
		_world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'w>,
		_state: &'s Self::State,
		_last_run: bevy::ecs::change_detection::Tick,
		_this_run: bevy::ecs::change_detection::Tick,
	) -> Self::Fetch<'w> {
		
	}

	const IS_DENSE: bool = true;

	unsafe fn set_archetype<'w, 's>(
		_fetch: &mut Self::Fetch<'w>,
		_state: &'s Self::State,
		_archetype: &'w bevy::ecs::archetype::Archetype,
		_table: &'w bevy::ecs::storage::Table,
	) {
		
	}

	unsafe fn set_table<'w, 's>(
		_fetch: &mut Self::Fetch<'w>,
		_state: &'s Self::State,
		_table: &'w bevy::ecs::storage::Table,
	) {
		
	}

	fn update_component_access(_state: &Self::State, _access: &mut bevy::ecs::query::FilteredAccess) {
		
	}

	fn init_state(_world: &mut bevy::ecs::world::World) -> Self::State {
		
	}

	fn get_state(components: &bevy::ecs::component::Components) -> Option<Self::State> {
		Some(())
	}

	fn matches_component_set(
		_state: &Self::State,
		_set_contains_id: &impl Fn(bevy::ecs::component::ComponentId) -> bool,
	) -> bool {
		true
	}
}

unsafe impl<H,T,HState,TState> WorldQuery for HQueryCons<H,T>//HQueryData<HCons<H,T>>
where 
	H:WorldQuery<State = HState>,
	T:WorldQuery<State = TState>,
	TState:Send+Sync+Sized,
	HState:Send+Sync+Sized,
	// for<'w> <H as WorldQuery>::Fetch<'w> : Clone,
	// for<'w> HCons< H::Fetch<'w> , <HQueryData<T> as WorldQuery>::Fetch<'w> >:Clone,
	// for<'w> <HQueryData<T> as WorldQuery>::Fetch<'w> : Clone,
{
	type State = HCons< HState , TState >;

	//type State = HCons< HState , TState >;
	
	type Fetch<'w> = HCons< H::Fetch<'w> , <T as WorldQuery>::Fetch<'w> >;


	fn shrink_fetch<'wlong: 'wshort, 'wshort>(fetch: Self::Fetch<'wlong>) -> Self::Fetch<'wshort> {
		// todo!()
		HCons{head:H::shrink_fetch(fetch.head),tail:<T as WorldQuery>::shrink_fetch(fetch.tail)}
	}

	unsafe fn init_fetch<'w, 's>(
		world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'w>,
		state: &'s Self::State,
		last_run: bevy::ecs::change_detection::Tick,
		this_run: bevy::ecs::change_detection::Tick,
	) -> Self::Fetch<'w> {
		// todo!()
		HCons{
			head:unsafe { H::init_fetch(world, &state.head, last_run, this_run) },
			tail:unsafe { <T as WorldQuery>::init_fetch(world, &state.tail, last_run, this_run) }
		}
	}

	const IS_DENSE: bool = {
		H::IS_DENSE && <T as WorldQuery>::IS_DENSE
	};

	unsafe fn set_archetype<'w, 's>(
		fetch: &mut Self::Fetch<'w>,
		state: &'s Self::State,
		archetype: &'w bevy::ecs::archetype::Archetype,
		table: &'w bevy::ecs::storage::Table,
	) {
		unsafe { H::set_archetype(&mut fetch.head, &state.head, archetype, table) };
		unsafe { <T as WorldQuery>::set_archetype(&mut fetch.tail, &state.tail, archetype, table) };
		// let fs=fetch.zip(state);
		
	}

	unsafe fn set_table<'w, 's>(
		fetch: &mut Self::Fetch<'w>,
		state: &'s Self::State,
		table: &'w bevy::ecs::storage::Table,
	) {
		unsafe { H::set_table(&mut fetch.head, &state.head, table) };
		unsafe { <T as WorldQuery>::set_table(&mut fetch.tail, &state.tail, table) };
	}

	fn update_component_access(state: &Self::State, access: &mut bevy::ecs::query::FilteredAccess) {
		H::update_component_access(&state.head, access);
		<T as WorldQuery>::update_component_access(&state.tail,access);
	}

	fn init_state(world: &mut bevy::ecs::world::World) -> Self::State {
		HCons{
			head:H::init_state(world),
			tail:<T as WorldQuery>::init_state(world)
		}
		
	}

	fn get_state(components: &bevy::ecs::component::Components) -> Option<Self::State> {
		Some(
		HCons{
			head:H::get_state(components)?,
			tail:<T as WorldQuery>::get_state(components)?
		}
		)
	}

	fn matches_component_set(
		state: &Self::State,
		set_contains_id: &impl Fn(bevy::ecs::component::ComponentId) -> bool,
	) -> bool {
		H::matches_component_set(&state.head, set_contains_id)
		&& <T as WorldQuery>::matches_component_set(&state.tail, set_contains_id)
	}
}


unsafe impl QueryData for HQueryNil {
	const IS_READ_ONLY: bool=true;

	const IS_ARCHETYPAL: bool=true;

	type ReadOnly=Self;

	type Item<'w, 's>=HNil;

	fn shrink<'wlong: 'wshort, 'wshort, 's>(
		item: Self::Item<'wlong, 's>,
	) -> Self::Item<'wshort, 's> {
		item
	}

	unsafe fn fetch<'w, 's>(
		_state: &'s Self::State,
		_fetch: &mut Self::Fetch<'w>,
		_entity: bevy::ecs::entity::Entity,
		_table_row: bevy::ecs::storage::TableRow,
	) -> Option<Self::Item<'w, 's>> {
		Some(HNil)
	}

	fn iter_access(state: &Self::State) -> impl Iterator<Item = bevy::ecs::query::EcsAccessType<'_>> {
		std::iter::empty()
	}
}

unsafe impl ReadOnlyQueryData for HQueryNil {}

unsafe impl<H,T,TReadOnlyInner,HState,TState> QueryData for HQueryCons<H,T>
where 
	H: QueryData<State = HState>,

	T: QueryData<ReadOnly = TReadOnlyInner, State = TState>,

	HQueryCons< H::ReadOnly, TReadOnlyInner > : ReadOnlyQueryData <State = 
		HCons< HState, TState >
	>,
	TState:Send+Sync+Sized,
	HState:Send+Sync+Sized,
{
	const IS_READ_ONLY: bool = H::IS_READ_ONLY && <T as QueryData>::IS_READ_ONLY;

	const IS_ARCHETYPAL: bool = H::IS_ARCHETYPAL && <T as QueryData>::IS_ARCHETYPAL;

	type ReadOnly = HQueryCons< H::ReadOnly, TReadOnlyInner> ;

	type Item<'w, 's> = HCons< H::Item<'w,'s>, <T as QueryData>::Item<'w,'s> >;

	fn shrink<'wlong: 'wshort, 'wshort, 's>(
		item: Self::Item<'wlong, 's>,
	) -> Self::Item<'wshort, 's> {
		// h_cons(H::shrink(item.head), <HQueryData<T> as QueryData>::shrink(item.tail))
		HCons{
			head:H::shrink(item.head),
			tail:<T as QueryData>::shrink(item.tail)
		}
	}

	fn provide_extra_access(
		state: &mut Self::State,
		access: &mut Access,
		available_access: &Access,
	) {
		H::provide_extra_access(&mut state.head, access, available_access);
		<T as QueryData>::provide_extra_access(&mut state.tail, access, available_access);
	}

	unsafe fn fetch<'w, 's>(
		state: &'s Self::State,
		fetch: &mut Self::Fetch<'w>,
		entity: bevy::ecs::entity::Entity,
		table_row: bevy::ecs::storage::TableRow,
	) -> Option<Self::Item<'w, 's>> {
		let h=unsafe { H::fetch(&state.head, &mut fetch.head, entity, table_row) }?;
		let t=unsafe { <T as QueryData>::fetch(&state.tail, &mut fetch.tail, entity, table_row) }?;
		Some(HCons { head: h, tail: t })
	}

	fn iter_access(state: &Self::State) -> impl Iterator<Item = bevy::ecs::query::EcsAccessType<'_>> {
		H::iter_access(&state.head).chain(<T as QueryData>::iter_access(&state.tail))
	}
}

unsafe impl<H,T> ReadOnlyQueryData for HQueryCons<H,T>//HQueryData<HCons<H,T>>
where 
	H: ReadOnlyQueryData,

	T:ReadOnlyQueryData,
{
	
}

impl ReleaseStateQueryData for HQueryNil {
	fn release_state<'w>(_item: Self::Item<'w, '_>) -> Self::Item<'w, 'static> {
		HNil
	}
}

impl<H,T,TReadOnlyInner> ReleaseStateQueryData for HQueryCons<H,T>//HQueryData<HCons<H,T>>
where 
	H:QueryData+ReleaseStateQueryData,
	T:QueryData<ReadOnly = TReadOnlyInner>,
	TReadOnlyInner: ReadOnlyQueryData<State = <T as WorldQuery>::State>,
	T:ReleaseStateQueryData
{
	fn release_state<'w>(item: Self::Item<'w, '_>) -> Self::Item<'w, 'static> {
		HCons{
			head:H::release_state(item.head),
			tail:<T as ReleaseStateQueryData>::release_state(item.tail)
		}
	}
}



#[cfg(test)]
mod test{
	use super::*;
    use bevy::ecs::{component::Component, system::Query};
use frunk::HList;

	#[derive(Component)]
	struct C1;
	#[derive(Component)]
	struct C2;

	fn check_w_q<Q>()
		where Q:WorldQuery
	{}

	
	fn check_q_d<Q>()
		where Q:QueryData
	{}

	fn test(){
		check_w_q::< HToQueryType<HList!(&C1,&C2)>>();

		check_q_d::< HToQueryType<HList!(&C1,&C2)> >();

		check_q_d::< HToQueryType<HList!(&C1,&C2,&C1)> >();
	}

	// fn test2(q:Query<HQueryData<HList!(&C1,&C2)>>){

	// }
}