
/// Determine whether the point (x,y) is in the rect defined by (left, bottom, right, top)
pub fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}

/// Determine whether the rect defined by (x, y, [w,h]) intersects with the
/// rect defined by (x2, y2, [w2, h2]).
pub fn rect_in_rect(
    x_a: f32, y_a: f32, dim_a: [f32; 2],
    x_b: f32, y_b: f32, dim_b: [f32; 2],
) -> bool {
    // If the rects are intersecting, the gap will be less than 0.
    (x_a - x_b).abs() < (dim_a[0] + dim_b[0]) / 2.0 &&
    (y_a - y_b).abs() < (dim_a[1] + dim_b[1]) / 2.0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_in_rect() {
        let (left, bottom, right, top) = (0.0, 0.0, 5.0, 5.0);
        // In the middle of the rect
        assert!(point_in_rect(right / 2.0, top / 2.0, left, bottom, right, top));
        // On the edges
        assert!(point_in_rect(left, 1.0, left, bottom, right, top));
        assert!(point_in_rect(1.0, bottom, left, bottom, right, top));
        assert!(point_in_rect(right, 1.0, left, bottom, right, top));
        assert!(point_in_rect(1.0, top, left, bottom, right, top));
        // Outside the rect
        assert!(!point_in_rect(6.0, 10.0, left, bottom, right, top));
    }

    #[test]
    fn test_rect_in_rect() {
        // Intersecting boxes.
        let (x_a, y_a, dim_a) = (0.0, 0.0, [10.0, 10.0]);
        let (x_b, y_b, dim_b) = (5.0, 5.0, [10.0, 10.0]);

        assert!(rect_in_rect(x_a, y_a, dim_a, x_b, y_b, dim_b));

        let (x_a, y_a, dim_a) = (0.0, 0.0, [5.0, 5.0]);
        let (x_b, y_b, dim_b) = (0.0, 2.5, [5.0, 5.0]);

        assert!(rect_in_rect(x_a, y_a, dim_a, x_b, y_b, dim_b));

        // non-intersecting boxes.
        let (x_a, y_a, dim_a) = (0.0, 0.0, [5.0, 5.0]);
        let (x_b, y_b, dim_b) = (10.0, 10.0, [5.0, 5.0]);

        assert!(!rect_in_rect(x_a, y_a, dim_a, x_b, y_b, dim_b));
    }
}