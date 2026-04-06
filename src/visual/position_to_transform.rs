use bevy::{app::{App, Update}, ecs::system::{Query, Res}, math::Vec3, time::{Fixed, Time}, transform::components::Transform};
use nalgebra::{RealField, SVector};
use physics_basic::stats::{Pos, Vel};


use crate::{physics::{}, stat_component::stat::Stat};


fn vec_fix_to_vec_f32<Num:RealField>(a:SVector<Num,3>)->Vec3 {
	Vec3 { x:a[0].to_subset().unwrap() , y: a[1].to_subset().unwrap(), z: a[2].to_subset().unwrap() }
}

pub fn position_to_transform<Num:RealField+Copy>(mut query:Query<(&Stat<Pos<Num,3>>,Option<&Stat<Vel<Num,3>>>,&mut Transform)>,time:Res<Time<Fixed>>) {
    query.par_iter_mut().for_each(|(p,v_may,mut t)|{
        if let Some(v)=v_may{
            let a = time.overstep_fraction()*time.delta_secs();
            //t.translation=vec_fix_to_vec_f32(p.0.0+v.0.0*a);
			t.translation=vec_fix_to_vec_f32(p.0.0) + vec_fix_to_vec_f32(v.0.0)*a;
            //if let Some(dir)=vec_fix_to_vec_f32(v.0.0).try_normalize(0){
            //    t.rotation=Quat::from_rotation_arc(Vec3::Y, dir);
            //}
        }else {
            t.translation=vec_fix_to_vec_f32(p.0.0);
        }
        //t.translation=p.0.0;
    });
}


pub fn plugin<Num:RealField+Copy>(app:&mut App) {
	app.add_systems(Update, position_to_transform::<Num>);
}
