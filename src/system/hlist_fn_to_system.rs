
use bevy::{ecs::{query::{QueryData, QueryFilter, QueryItem, ReadOnlyQueryData}, schedule::{IntoScheduleConfigs, ScheduleConfigs}, system::{Query, ScheduleSystem}}, prelude::SystemParamFunction};
use frunk::{HNil, Poly, hlist::{HFoldLeftable, HMappable, HZippable}};
use wacky_bag_hlist::{chain_fn::ChainFunc, h_list_helpers::{HMapP, HTypeFnToMapper, MapFromRef, MapMut, MapRef}, reverse_func::ReverseFunc, variadics_tuple::{ToVariadicsTuple, VariadicsTupleToHlist}};
use crate::{system::{multi_sets::{FoldScheduleConfigsAfterSets, FoldScheduleConfigsBeforeSets, }, processing_system::{MapToProcessingSystemSet, ScheduleConfigsProcessing}}, utils::{h_list_query::{HToQuery, HToQueryType}, stat_for_hlist::{HChangeAdd, HChangeAddG, HStatSet, HTakeChagne, HTakeChagneG, MapFromStatRef, MapToChange, MapToDetermining, MapToStat, MapToQFWith}}};



/// use [to_calculate_system] for system
#[derive(Debug,Default,Clone, Copy)]
pub struct CalculateChangeSystem<F>(pub F);

/// convert a `Fn(HList!(&A,&B,&C))->HList!(D,E,F)` into a system with `Query<(&Stat<A>,&Stat<B>,&Stat<C>,&Chagne<D>,&Change<E>,&Change<F>)>`
pub fn to_calculate_change_system<F,FIR,FO>(f:F)->
impl SystemParamFunction<(FIR,FO),In = (),Out = ()>
// CalculateSystem<F>
	where F:Fn(FIR)->FO+Send+Sync+'static,
	CalculateChangeSystem<F>:SystemParamFunction<(FIR,FO),In = (),Out = ()>
{
	CalculateChangeSystem(f)
}


impl<
	F,
	FIR,FO,
	FIRS, FOC, FORC,
	FIRSQ, FORCQ,
	FIRSQR, FORCQR,
	// FIR2,
	FO2,
	// M
> SystemParamFunction<
	// (Self,FIR,FO)
	// (FIR,FO)
	// HList!(FIR,FO)
	(FIR,FO)
	
	// (<<FIRSQR as QueryData>::Item<'static,'static> as HMappable<Poly<MapFromStatRef>>>::Output, FO2)
> 
for CalculateChangeSystem<F>
where 
	// Self:GetCalculateSystemMarker<M,Marker = CalculateSystemMarker<(FIR,FO)>>,
	F:Send+Sync+'static,
	for<'a,'w,'s> &'a F:
		Fn(FIR)->FO+
		Fn( <<<FIRSQR as QueryData>::Item<'w,'s>as VariadicsTupleToHlist>::Output as HMappable<Poly<MapFromStatRef>>>::Output )->FO2,
	
	// F:Fn(FIR)->FO,

	FIR:HMappable<Poly<HTypeFnToMapper<ReverseFunc<MapFromStatRef>>>,Output = FIRS>,
	FIRS:HMappable<Poly<MapFromStatRef>,Output = FIR>,

	FO:HMappable<Poly<MapToChange>,Output = FOC>,
	FOC:HMappable<Poly<HTypeFnToMapper<MapRef<'static>>>,Output = FORC>,
	// for<'a> FOC:HMappable<Poly<HTypeFnToMapper<MapRef<'a>>>>,

	// FIRS:'static,FORC:'static,
	// FIRS:HToQuery<Output = FIRSQ>,
	// FORC:HToQuery<Output = FORCQ>,
	FIRS:ToVariadicsTuple<Output = FIRSQ>,
	FORC:ToVariadicsTuple<Output = FORCQ>,
	// for<'a> HMapP<FOC,HTypeFnToMapper<MapRef<'a>>>:HToQuery,

	FIRSQ:'static+QueryData<ReadOnly = FIRSQR>,
	FORCQ:'static+QueryData<ReadOnly = FORCQR>,
	// for<'a> HToQueryType<HMapP<FOC,HTypeFnToMapper<MapRef<'a>>>>:'static+QueryData,

	FIRSQR:ReadOnlyQueryData,
	FORCQR:ReadOnlyQueryData,
	// for<'a> <HToQueryType<HMapP<FOC,HTypeFnToMapper<MapRef<'a>>>> as QueryData>::ReadOnly:ReadOnlyQueryData,

	for<'w, 's> <FIRSQR as QueryData>::Item<'w,'s>: VariadicsTupleToHlist<Output : HMappable<Poly<MapFromStatRef>>/*,Output = FIR2*/>,
	// for<'w,'s> <FIRSQR as QueryData>::Item<'w,'s>: HMappable<Poly<MapFromStatRef>,Output = FIR2>,
	
	// for<'a,'w,'s> FO2:HZippable< <<HToQueryType<HMapP<FOC,HTypeFnToMapper<MapRef<'a>>>> as QueryData>::ReadOnly as QueryData>::Item<'w,'s>,Zipped : HMappable<Poly<HChangeAdd>> >
	for<'w, 's> <FORCQR as QueryData>::Item<'w,'s>: VariadicsTupleToHlist,
	for<'w, 's> FO2:HZippable< <<FORCQR as QueryData>::Item<'w,'s>as VariadicsTupleToHlist>::Output  ,Zipped : HMappable<Poly<HChangeAdd>> >
{
    type In = ();
    type Out = ();
    type Param = Query<'static,'static,(
		FIRSQ,
		FORCQ
	)>;
	
    fn run(
            &mut self,
            _input:(),
            param_value: bevy::ecs::system::SystemParamItem<Self::Param>,
        ) -> () 
	{
		param_value.par_iter().for_each(|(a,b)|{
			let firs=a.to_hlist();
			let forc=b.to_hlist();
			let fir=firs.map(Poly(MapFromStatRef));
			fn call_inner<FI,FO>(f:impl Fn( FI )->FO,i:FI)->FO{
				f(i)
			}
			let f:&F=&self.0;
			// let fo=(f)(fir);
			let fo=call_inner(f,fir);
			fo.zip(forc).map(Poly(HChangeAdd));
		});
		
    }
}

#[derive(Debug,Default,Clone, Copy)]
pub struct CollectChangeSystem<F>(pub F);

pub fn to_collect_change_system<F,FIR,FIC,FO>(f:F)->
impl SystemParamFunction<(FIR,FIC,FO),In = (),Out = ()>
where F:Fn(FIR,FIC)->FO,
	CollectChangeSystem<F>:SystemParamFunction<(FIR,FIC,FO),In = (),Out = ()>
{
	CollectChangeSystem(f)
}

/// use `Stat<FIR>` and convert `Change<FIC>` to `Change<FO>`, with filter `With<Determining<FO>>`
pub fn to_collect_change_system_with_processing<F,FIR,FIC,FO>(f:F)->
	ScheduleConfigs<ScheduleSystem>
// CalculateSystem<F>
where 
	F:Fn(FIR,FIC)->FO+'static,
	CollectChangeSystem<F>:SystemParamFunction<(FIR,FIC,FO),In = (),Out = ()>,
	FIR:'static,FIC:'static,FO:'static,
	// FIR:HMappable<
	// 	Poly<HTypeFnToMapper<ChainFunc<MapFromRef,MapToStat>>>,
	// 	Output : HMappable<
	// 		Poly<MapToProcessingSystemSet>,
	// 		Output :Default+HFoldLeftable<
	// 			Poly<FoldScheduleConfigsAfterSets>, 
	// 			ScheduleConfigs<ScheduleSystem>,
	// 			Output = ScheduleConfigs<ScheduleSystem>>>>,
	FIC:HMappable<
		Poly<MapToChange>,
		Output : HMappable<
			Poly<MapToProcessingSystemSet>,
			Output :Default+HFoldLeftable<
				Poly<FoldScheduleConfigsAfterSets>, 
				ScheduleConfigs<ScheduleSystem>,
				Output = ScheduleConfigs<ScheduleSystem>>>>,
	FO:HMappable<
		Poly<MapToChange>,
		Output : HMappable<
			Poly<MapToProcessingSystemSet>,
			Output :Default+HFoldLeftable<
				Poly<FoldScheduleConfigsBeforeSets>, 
				ScheduleConfigs<ScheduleSystem>,
				Output = ScheduleConfigs<ScheduleSystem>>>>
{
	let r=CollectChangeSystem(f);
	let cfg=r.into_configs()
		.config_processing::<
			// HMapP<FIR,HTypeFnToMapper<ChainFunc<MapFromRef,MapToStat>>>,
			HMapP<FIC,MapToChange>,
			HNil,
			HMapP<FO,MapToChange>
		>()
		;
	// .after_sets(processing_system_sets::<HMapP<FIC,MapToChange>>());
	cfg
}

impl<
	F,
	FIR,FIC,FO,
	FIRS, FICMC, FOC, FOMC,
	FIRSVQ, FICMCVQ, FOMSVQ,
	FO2,
	FODW,FODWQF,
> SystemParamFunction<
	(FIR,FIC,FO)
	
> 
for CollectChangeSystem<F>
where 
	F:Send+Sync+'static,
	for<'a,'w,'s> &'a F:
		Fn(FIR,FIC)->FO+
		Fn( 
			HMapP< <QueryItem<FIRSVQ> as VariadicsTupleToHlist>::Output ,MapFromStatRef>,
			HMapP< <QueryItem<FICMCVQ> as VariadicsTupleToHlist>::Output ,HTakeChagneG>
		)->FO2,

	FIR:HMappable<Poly<HTypeFnToMapper<ReverseFunc<MapFromStatRef>>>,Output = FIRS>,
	
	FIRS:HMappable<Poly<MapFromStatRef>,Output = FIR>,

	FIC:HMappable<Poly<HTypeFnToMapper< ChainFunc< MapToChange, MapMut<'static>> >>, Output = FICMC>,

	FICMC:HMappable<Poly<HTakeChagne>,Output = FIC>,

	FO:HMappable<Poly<MapToChange>,Output = FOC>,
	FOC:HMappable<Poly<HTypeFnToMapper<MapMut<'static>>>,Output = FOMC>,

	FIRS:ToVariadicsTuple<Output = FIRSVQ>,
	FICMC:ToVariadicsTuple<Output = FICMCVQ>,
	FOMC:ToVariadicsTuple<Output = FOMSVQ>,

	FIRSVQ:'static+QueryData,
	FICMCVQ:'static+QueryData,
	FOMSVQ:'static+QueryData,

	for<'w,'s> <FIRSVQ as QueryData>::Item<'w,'s>: VariadicsTupleToHlist<Output : HMappable<Poly<MapFromStatRef>>/*,Output = FIR2*/>,
	for<'w,'s> QueryItem<'w,'s,FICMCVQ>: VariadicsTupleToHlist<Output : HMappable<Poly<HTakeChagneG>> > ,
	for<'w,'s> QueryItem<'w,'s,FOMSVQ> : VariadicsTupleToHlist,
	for<'w,'s> FO2:HZippable< <QueryItem<'w,'s,FOMSVQ> as VariadicsTupleToHlist>::Output ,Zipped : HMappable<Poly<HChangeAddG>> >,

	FO:HMappable<Poly< HTypeFnToMapper<ChainFunc<MapToDetermining, MapToQFWith>> >, Output = FODW>,
	FODW:ToVariadicsTuple<Output = FODWQF>,
	FODWQF:QueryFilter+'static,
{
    type In = ();
    type Out = ();
    type Param = Query<'static,'static,(
		FIRSVQ,
		FICMCVQ,
		FOMSVQ
	),FODWQF>;
	
    fn run(
            &mut self,
            _input:(),
            mut param_value: bevy::ecs::system::SystemParamItem<Self::Param>,
        ) -> () 
	{
		param_value.par_iter_mut().for_each(|(a,b,c)|{
			let firs=a.to_hlist();
			let ficmc=b.to_hlist();
			let fir=firs.map(Poly(MapFromStatRef));
			let fic=ficmc.map(Poly(HTakeChagneG));
			fn call_inner<FI,FI2,FO>(f:impl Fn( FI,FI2 )->FO,i:FI,i2:FI2)->FO{
				f(i,i2)
			}
			let f:&F=&self.0;
			// let fo=(f)(fir);
			let fo=call_inner(f,fir,fic);
			fo.zip(c.to_hlist()).map(Poly(HChangeAddG));
		});
		
    }
}


/// see [`to_calculate_stat_system_with_processing`]
#[derive(Debug,Default,Clone, Copy)]
pub struct CalculateStatSystem<F>(pub F);

/// convert a `Fn(HList!(&A,&B,&C))->HList!(D,E,F)` into a system with `Query<(&Stat<A>,&Stat<B>,&Stat<C>,&mut Stat<D>,&mut Stat<E>,&mut Stat<F>)>`
pub fn to_calculate_stat_system<F,FIR,FO>(f:F)->
impl SystemParamFunction<(FIR,FO),In = (),Out = ()>
// CalculateSystem<F>
	where F:Fn(FIR)->FO+Send+Sync+'static,
	CalculateStatSystem<F>:SystemParamFunction<(FIR,FO),In = (),Out = ()>
{
	CalculateStatSystem(f)
}

/// convert a Fn(HList!(&A,&B,&C))->HList!(D,E,F) into a system `Query<(&Stat<A>,&Stat<B>,&Stat<C>,&mut Stat<D>,&mut Stat<E>,&mut Stat<F>)>` (like)
/// 
/// with `config_processing<HList!(Stat<A>,Stat<B>,Stat<C>),HNil,HList!(Stat<D>,Stat<E>,Stat<F>)>`
pub fn to_calculate_stat_system_with_processing<F,FIR,FO>(f:F)->
	ScheduleConfigs<ScheduleSystem>
// CalculateSystem<F>
where 
	F:Fn(FIR)->FO+Send+Sync+'static,
	FIR:'static,FO:'static,
	CalculateStatSystem<F>:SystemParamFunction<(FIR,FO),In = (),Out = ()>,
	FIR:HMappable<Poly<HTypeFnToMapper<ChainFunc<MapFromRef,MapToStat>>>,Output : HMappable<Poly<MapToProcessingSystemSet>,Output :Default+HFoldLeftable<Poly<FoldScheduleConfigsAfterSets>, ScheduleConfigs<ScheduleSystem>,Output = ScheduleConfigs<ScheduleSystem>>>>,
	FO:HMappable<Poly<MapToStat>,Output : HMappable<Poly<MapToProcessingSystemSet>,Output :Default+HFoldLeftable<Poly<FoldScheduleConfigsBeforeSets>, ScheduleConfigs<ScheduleSystem>,Output = ScheduleConfigs<ScheduleSystem>>>>
{
	let r=CalculateStatSystem(f);
	let cfg=r.into_configs();
	let cfg=cfg.config_processing::<
		HMapP<FIR,HTypeFnToMapper<ChainFunc<MapFromRef,MapToStat>>>,
		HNil,
		HMapP<FO,MapToStat>
	>();
	cfg
}

// #[derive(Debug,Default,Clone, Copy)]
// pub struct CalculateStatSystemMarker<A>(pub A);
pub type CalculateStatSystemMarker<A>=A;

impl<
	F,
	FIR,FO,
	FIRS, FOS, FOMS,
	FIRSQ, FOMSQ,
	// FIRSQR, FOMSQR,
	// FIR2,
	FO2,
	// M
> SystemParamFunction<
	// (Self,FIR,FO)
	// (FIR,FO)
	// HList!(FIR,FO)
	(FIR,FO)
	
	// (<<FIRSQR as QueryData>::Item<'static,'static> as HMappable<Poly<MapFromStatRef>>>::Output, FO2)
> 
for CalculateStatSystem<F>
where 
	// Self:GetCalculateSystemMarker<M,Marker = CalculateSystemMarker<(FIR,FO)>>,
	F:Send+Sync+'static,
	for<'a,'w,'s> &'a F:
		Fn(FIR)->FO+
		Fn( <<FIRSQ as QueryData>::Item<'w,'s> as HMappable<Poly<MapFromStatRef>>>::Output )->FO2,
	
	// F:Fn(FIR)->FO,

	FIR:HMappable<Poly<HTypeFnToMapper<ReverseFunc<MapFromStatRef>>>,Output = FIRS>,
	FIRS:HMappable<Poly<MapFromStatRef>,Output = FIR>,

	FO:HMappable<Poly<MapToStat>,Output = FOS>,
	FOS:HMappable<Poly<HTypeFnToMapper<MapMut<'static>>>,Output = FOMS>,
	// for<'a> FOC:HMappable<Poly<HTypeFnToMapper<MapRef<'a>>>>,

	// FIRS:'static,FORC:'static,
	FIRS:HToQuery<Output = FIRSQ>,
	FOMS:HToQuery<Output = FOMSQ>,
	// for<'a> HMapP<FOC,HTypeFnToMapper<MapRef<'a>>>:HToQuery,

	FIRSQ:'static+QueryData,
	FOMSQ:'static+QueryData,
	// for<'a> HToQueryType<HMapP<FOC,HTypeFnToMapper<MapRef<'a>>>>:'static+QueryData,
	// for<'a> <HToQueryType<HMapP<FOC,HTypeFnToMapper<MapRef<'a>>>> as QueryData>::ReadOnly:ReadOnlyQueryData,

	for<'w,'s> <FIRSQ as QueryData>::Item<'w,'s>: HMappable<Poly<MapFromStatRef>/*,Output = FIR2*/>,
	// for<'w,'s> <FIRSQR as QueryData>::Item<'w,'s>: HMappable<Poly<MapFromStatRef>,Output = FIR2>,
	
	// for<'a,'w,'s> FO2:HZippable< <<HToQueryType<HMapP<FOC,HTypeFnToMapper<MapRef<'a>>>> as QueryData>::ReadOnly as QueryData>::Item<'w,'s>,Zipped : HMappable<Poly<HChangeAdd>> >
	for<'w,'s> FO2:HZippable< <FOMSQ as QueryData>::Item<'w,'s>,Zipped : HMappable<Poly<HStatSet>> >
{
    type In = ();
    type Out = ();
    type Param = Query<'static,'static,(
		HToQueryType<FIRS>,
		HToQueryType<FOMS>
	)>;
	
    fn run(
            &mut self,
            _input:(),
            mut param_value: bevy::ecs::system::SystemParamItem<Self::Param>,
        ) -> () 
	{
		param_value.par_iter_mut().for_each(|(a,b)|{
			let firs=a;
			let forc=b;
			let fir=firs.map(Poly(MapFromStatRef));
			fn call_inner<FI,FO>(f:impl Fn( FI )->FO,i:FI)->FO{
				f(i)
			}
			let f:&F=&self.0;
			// let fo=(f)(fir);
			let fo=call_inner(f,fir);
			fo.zip(forc).map(Poly(HStatSet));
		});
		
    }
}
