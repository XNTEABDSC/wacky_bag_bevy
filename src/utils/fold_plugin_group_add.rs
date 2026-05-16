use bevy::app::{Plugin, PluginGroupBuilder};
use frunk::Func;




pub struct FoldPluginGroupBuilderAdd;

impl<P> Func<(PluginGroupBuilder,P)> for FoldPluginGroupBuilderAdd 
	where P:Plugin
{
	type Output=PluginGroupBuilder;

	fn call((pgb,p):(PluginGroupBuilder,P)) -> Self::Output {
		// todo!()
		pgb.add(p)
	}
}