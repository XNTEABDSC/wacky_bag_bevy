use std::ops::Add;

use bevy::ecs::{query::QueryData, system::{Query, System}};
use wacky_bag::utils::output_func::HMappableFrom;

use crate::utils::h_list_query_data::HQueryData;

// pub fn h_list_inplace_fn<F,Refs,Muts,FI,ComponentToRef,ComponentToMut,RefQueryData,MutQueryData>(f:F)
// ->impl System

// where 
// 	Refs:HMappableFrom<ComponentToRef,Input = RefQueryData>,
// 	Muts:HMappableFrom<ComponentToMut,Input = MutQueryData>,
// 	HQueryData<RefQueryData>:QueryData, //<Item = HQueryData<RefQueryData>>
// 	HQueryData<MutQueryData>:QueryData, //<Item = HQueryData<MutQueryData>>
// 	Refs:Add<Muts>,
// 	F:Fn( <Refs as Add<Muts>>::Output )
// {
// 	let sys= |mut q:Query<(HQueryData< <Refs as HMappableFrom<ComponentToRef>>::Input > , HQueryData< <Muts as HMappableFrom<ComponentToMut>>::Input >)>|{
// 		q.par_iter_mut().for_each(|v|{
// 			let (ref_qd,mut_qd)=v;
// 			//ref_qd
// 		});
// 	};
// 	return sys;
// }


// pub struct HListInplaceFn<F>(pub F);

// impl<F,Refs,Muts,FI,ComponentToRef,ComponentToMut,RefQueryData,MutQueryData> bevy::prelude::SystemParamFunction<
//     HListInplaceFn<F>
// > 
// for HListInplaceFn<F>
// where 
// 	Refs:HMappableFrom<ComponentToRef,Input = RefQueryData>,
// 	Muts:HMappableFrom<ComponentToMut,Input = MutQueryData>,
// 	HQueryData<RefQueryData>:QueryData, //<Item = HQueryData<RefQueryData>>
// 	HQueryData<MutQueryData>:QueryData, //<Item = HQueryData<MutQueryData>>
// 	Refs:Add<Muts>,
// 	F:Fn( Refs ),
//     // for<'w2, 's2> <<T as SetUiWorldToViewPosition>::SystemParams as SystemParam>::Item<'w2, 's2>:Sync
// {

//     type In = ();
//     type Out = ();
//     type Param = (Query<'static,'static,(&'static mut UiTransform,&'static T, 
//         <T as SetUiWorldToViewPosition>::QueryData), (Without<Camera>,)>, //(&'w Transform,&'w UiTargetCamera)
//     Query<'static,'static,(&'static Camera,&'static GlobalTransform),(Without<T>,)>,
//     T::SystemParams);
//     fn run(
//             &mut self,
//             _input:(),
//             param_value: bevy::ecs::system::SystemParamItem<Self::Param>,
//         ) -> () {
//         let (nodes,cameras,others)=param_value;
//         set_ui_world_to_view(nodes,cameras,others);
//     }
// }