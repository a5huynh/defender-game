
/// Determine whether the point (x,y) is in the rect defined by (left, bottom, right, top)
pub fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}