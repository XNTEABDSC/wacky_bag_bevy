use std::mem::transmute;

use bevy::tasks::{ComputeTaskPool, Scope};
use wacky_bag::traits::scope_no_ret::{
    ThreadScope, ThreadScopeCreator, ThreadScopeCreatorStd, ThreadScopeUser,
};

pub struct ComputeTaskPoolScope< 'scope, 'env: 'scope>(pub &'scope Scope<'scope, 'env, ()>);
pub struct ComputeTaskPoolScopeCreater;

impl<'scope, 'env: 'scope> ThreadScope<'scope> for ComputeTaskPoolScope<'scope, 'env> {
    fn spawn<F>(&self, f: F) -> ()
    where
        F: FnOnce() -> () + Send + 'scope,
    {
		self.0.spawn(async{f();})
		;
    }
}

impl ThreadScopeCreator for ComputeTaskPoolScopeCreater {
    fn scope<'env, F>(&self, f: F) -> ()
    where
        F: ThreadScopeUser<'env>,
    {
        // ThreadScopeCreatorStd
        ComputeTaskPool::get().scope(|s: &bevy::tasks::Scope<'_, '_, _>| {
			let a=ComputeTaskPoolScope(s);
			f.use_scope(a);
        });
    }
}