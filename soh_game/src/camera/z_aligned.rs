//-----------------------------------------------------------------------------
use winit::event::{DeviceEvent, WindowEvent};
//-----------------------------------------------------------------------------
/// Camera transformer which rotates the camera around camera's X axis and world's Z axis
pub struct ZAligned {
    rotating_camera: bool,
    rotation_speed: (f32, f32),
}

//-----------------------------------------------------------------------------

impl ZAligned {
    pub fn rotation_speed(&self) -> (f32, f32) {
        return self.rotation_speed;
    }

    pub fn set_rotation_speed(&mut self, speed: (f32, f32)) {
        self.rotation_speed.0 = speed.0.abs();
        self.rotation_speed.1 = speed.1.abs();
    }
}

//-----------------------------------------------------------------------------

impl ZAligned {
    pub fn new() -> Self {
        return ZAligned {
            rotating_camera: false,
            rotation_speed: (0.5, 0.5),
        };
    }

    pub fn on_window_event(&mut self, event: &WindowEvent) {
        /*
         * Start rotating camera on left mouse drag
         */
        if let WindowEvent::MouseInput {
            device_id: _,
            state,
            button: winit::event::MouseButton::Left,
        } = *event
        {
            self.rotating_camera = state.is_pressed();
        }
    }

    pub fn on_device_event(&mut self, camera: &mut super::Camera, event: &DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta: (dx, dy) } = *event {
            if self.rotating_camera {
                self.update_camera_rotation(camera, dx as f32, dy as f32);
            }
        }
    }

    fn update_camera_rotation(&mut self, camera: &mut super::Camera, dx: f32, dy: f32) {
        const ROTATION_VELOCITY_FACTOR: f32 = 0.008;

        camera.rotate_world_z(-dx * self.rotation_speed.0 * ROTATION_VELOCITY_FACTOR);
        camera.rotate_view_x(dy * self.rotation_speed.1 * ROTATION_VELOCITY_FACTOR);
    }
}

//-----------------------------------------------------------------------------

impl Default for ZAligned {
    fn default() -> Self {
        return Self::new();
    }
}

//-----------------------------------------------------------------------------
