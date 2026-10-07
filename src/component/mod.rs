use std::sync::{Arc, Mutex};

use crate::stat_component::stat::Stat;

pub mod vec_component;

/// Component Newtype
/// 
/// its currently [`Stat`], and may change if needed
pub type Comp<T>=Stat<T>;


pub type CAM<T>=Comp<Arc<Mutex<T>>>;