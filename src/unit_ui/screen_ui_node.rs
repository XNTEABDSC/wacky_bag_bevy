use bevy::{app::{App, Startup}, ecs::{component::Component, system::Commands}, ui::{percent, AlignItems, JustifyContent, Node}, utils::default};

#[derive(Component)]
pub struct ScreenUiNode;

// #[derive(Resource)]
// pub struct ScreenUiNodeEntity(pub Option<Entity>);

pub fn setup(mut cmd:Commands){

    cmd.spawn((
        ScreenUiNode,
        Node{
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }
    ));
}

pub fn plugin(app:&mut App){
    app.add_systems(Startup, setup);
}