use crate::*;

pub struct GeneralSystemsPlugin;

impl Plugin for GeneralSystemsPlugin {
    fn build(&self, _app: &mut App) {}
}

pub fn out_of_bounds(camera_x: f32, camera_y: f32, entity_x: f32, entity_y: f32) -> bool {
    ((entity_x.abs() - camera_x.abs()).abs() > SCREEN_WIDTH / 1.9)
        || ((entity_y.abs() - camera_y.abs()).abs() > SCREEN_HEIGHT / 1.9)
}

#[cfg(test)]
mod out_of_bounds_test {
    use super::*;

    #[test]
    fn test_out_of_bounds_true() {
        assert!(out_of_bounds(0.0, 0.0, 1000.0, 1000.0));
    }

    #[test]
    fn test_out_of_bounds_false() {
        assert!(!out_of_bounds(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn test_out_of_bounds_false_negative() {
        assert!(!out_of_bounds(-1000.0, -1000.0, -1000.0, -1000.0));
    }

    #[test]
    fn test_out_of_bounds_true_negative() {
        assert!(out_of_bounds(-1000.0, -1000.0, 0.0, 0.0));
    }
}
