use bevy::{camera::{Camera}, math::Vec3, transform::components::GlobalTransform};


/// 
pub fn world_to_view_scale(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    world_point: Vec3,
) -> Option<f32> {
    // 获取相机的视口大小
    // let viewport_size = camera.physical_viewport_size()?;
    
    // 计算世界点到屏幕的转换
    let world_to_screen = camera.world_to_viewport(camera_transform, world_point).ok()?;
    
    // 为了计算缩放，我们在世界点处创建一个微小的偏移点
    // 这里我们使用相机上方的一个小偏移
    let camera_up = camera_transform.up();
    let offset_point = world_point + camera_up * 1.0; // 1 单位长度的偏移
    
    // 计算偏移点的屏幕位置
    let offset_to_screen = camera.world_to_viewport(camera_transform, offset_point).ok()?;
    
    // 计算屏幕空间中两点之间的距离（像素）
    let screen_distance = world_to_screen.distance(offset_to_screen);
    
    // 缩放 = 屏幕距离 / 世界距离
    // 因为我们在世界空间中使用的是 1 单位长度的偏移
    Some(screen_distance)
}