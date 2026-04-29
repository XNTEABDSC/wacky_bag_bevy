use bevy::ecs::query::{Access, QueryData, ReadOnlyQueryData, ReleaseStateQueryData, WorldQuery};
use frunk::{HCons, HNil, hlist::h_cons};


/// allows to use hlist as QueryData
pub struct HQueryData<HL>(pub HL);


/// impl following impl WorldQuery for Entity
unsafe impl WorldQuery for HQueryData<HNil> {
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

unsafe impl<H,T> WorldQuery for HQueryData<HCons<H,T>>
where 
	H:WorldQuery,
	HQueryData<T>:WorldQuery
{
	
	type State = HCons< H::State , <HQueryData<T> as WorldQuery>::State >;
	
	type Fetch<'w> = HCons< H::Fetch<'w> , <HQueryData<T> as WorldQuery>::Fetch<'w> >;


	fn shrink_fetch<'wlong: 'wshort, 'wshort>(fetch: Self::Fetch<'wlong>) -> Self::Fetch<'wshort> {
		// todo!()
		HCons{head:H::shrink_fetch(fetch.head),tail:<HQueryData<T> as WorldQuery>::shrink_fetch(fetch.tail)}
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
			tail:unsafe { <HQueryData<T> as WorldQuery>::init_fetch(world, &state.tail, last_run, this_run) }
		}
	}

	const IS_DENSE: bool = {
		H::IS_DENSE && <HQueryData<T> as WorldQuery>::IS_DENSE
	};

	unsafe fn set_archetype<'w, 's>(
		fetch: &mut Self::Fetch<'w>,
		state: &'s Self::State,
		archetype: &'w bevy::ecs::archetype::Archetype,
		table: &'w bevy::ecs::storage::Table,
	) {
		unsafe { H::set_archetype(&mut fetch.head, &state.head, archetype, table) };
		unsafe { <HQueryData<T> as WorldQuery>::set_archetype(&mut fetch.tail, &state.tail, archetype, table) };
		// let fs=fetch.zip(state);
		
	}

	unsafe fn set_table<'w, 's>(
		fetch: &mut Self::Fetch<'w>,
		state: &'s Self::State,
		table: &'w bevy::ecs::storage::Table,
	) {
		unsafe { H::set_table(&mut fetch.head, &state.head, table) };
		unsafe { <HQueryData<T> as WorldQuery>::set_table(&mut fetch.tail, &state.tail, table) };
	}

	fn update_component_access(state: &Self::State, access: &mut bevy::ecs::query::FilteredAccess) {
		H::update_component_access(&state.head, access);
		<HQueryData<T> as WorldQuery>::update_component_access(&state.tail,access);
	}

	fn init_state(world: &mut bevy::ecs::world::World) -> Self::State {
		HCons{
			head:H::init_state(world),
			tail:<HQueryData<T> as WorldQuery>::init_state(world)
		}
		
	}

	fn get_state(components: &bevy::ecs::component::Components) -> Option<Self::State> {
		Some(
		HCons{
			head:H::get_state(components)?,
			tail:<HQueryData<T> as WorldQuery>::get_state(components)?
		}
		)
	}

	fn matches_component_set(
		state: &Self::State,
		set_contains_id: &impl Fn(bevy::ecs::component::ComponentId) -> bool,
	) -> bool {
		H::matches_component_set(&state.head, set_contains_id)
		&& <HQueryData<T> as WorldQuery>::matches_component_set(&state.tail, set_contains_id)
	}
}


unsafe impl QueryData for HQueryData<HNil> {
	const IS_READ_ONLY: bool=true;

	const IS_ARCHETYPAL: bool=true;

	type ReadOnly=Self;

	type Item<'w, 's>=Self;

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
		Some(HQueryData(HNil))
	}

	fn iter_access(state: &Self::State) -> impl Iterator<Item = bevy::ecs::query::EcsAccessType<'_>> {
		std::iter::empty()
	}
}

unsafe impl ReadOnlyQueryData for HQueryData<HNil> {}

unsafe impl<H,T,TReadOnlyInner> QueryData for HQueryData<HCons<H,T>>
where 
	H:QueryData,
	HQueryData<T>:QueryData<ReadOnly = HQueryData<TReadOnlyInner>>,
	HQueryData<TReadOnlyInner>: ReadOnlyQueryData<State = <HQueryData<T> as WorldQuery>::State>,
{
	const IS_READ_ONLY: bool = H::IS_READ_ONLY && <HQueryData<T> as QueryData>::IS_READ_ONLY;

	const IS_ARCHETYPAL: bool = H::IS_ARCHETYPAL && <HQueryData<T> as QueryData>::IS_ARCHETYPAL;

	type ReadOnly = HQueryData< HCons<H::ReadOnly, TReadOnlyInner> >;

	type Item<'w, 's> = HCons< H::Item<'w,'s>, <HQueryData<T> as QueryData>::Item<'w,'s> >;

	fn shrink<'wlong: 'wshort, 'wshort, 's>(
		item: Self::Item<'wlong, 's>,
	) -> Self::Item<'wshort, 's> {
		// h_cons(H::shrink(item.head), <HQueryData<T> as QueryData>::shrink(item.tail))
		HCons{
			head:H::shrink(item.head),
			tail:<HQueryData<T> as QueryData>::shrink(item.tail)
		}
	}

	fn provide_extra_access(
		state: &mut Self::State,
		access: &mut Access,
		available_access: &Access,
	) {
		H::provide_extra_access(&mut state.head, access, available_access);
		<HQueryData<T> as QueryData>::provide_extra_access(&mut state.tail, access, available_access);
	}

	unsafe fn fetch<'w, 's>(
		state: &'s Self::State,
		fetch: &mut Self::Fetch<'w>,
		entity: bevy::ecs::entity::Entity,
		table_row: bevy::ecs::storage::TableRow,
	) -> Option<Self::Item<'w, 's>> {
		let h=unsafe { H::fetch(&state.head, &mut fetch.head, entity, table_row) }?;
		let t=unsafe { <HQueryData<T> as QueryData>::fetch(&state.tail, &mut fetch.tail, entity, table_row) }?;
		Some(HCons { head: h, tail: t })
	}

	fn iter_access(state: &Self::State) -> impl Iterator<Item = bevy::ecs::query::EcsAccessType<'_>> {
		H::iter_access(&state.head).chain(<HQueryData<T> as QueryData>::iter_access(&state.tail))
	}
}

unsafe impl<H,T> ReadOnlyQueryData for HQueryData<HCons<H,T>>
where 
	H:ReadOnlyQueryData, 
	HQueryData<T>:ReadOnlyQueryData
{
	
}

impl ReleaseStateQueryData for HQueryData<HNil> {
	fn release_state<'w>(_item: Self::Item<'w, '_>) -> Self::Item<'w, 'static> {
		HQueryData(HNil)
	}
}

impl<H,T,TReadOnlyInner> ReleaseStateQueryData for HQueryData<HCons<H,T>>
where 
	H:QueryData+ReleaseStateQueryData,
	HQueryData<T>:QueryData<ReadOnly = HQueryData<TReadOnlyInner>>,
	HQueryData<TReadOnlyInner>: ReadOnlyQueryData<State = <HQueryData<T> as WorldQuery>::State>,
	HQueryData<T>:ReleaseStateQueryData
{
	fn release_state<'w>(item: Self::Item<'w, '_>) -> Self::Item<'w, 'static> {
		HCons{
			head:H::release_state(item.head),
			tail:<HQueryData<T> as ReleaseStateQueryData>::release_state(item.tail)
		}
	}
}