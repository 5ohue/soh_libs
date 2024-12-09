//-----------------------------------------------------------------------------
use winit::event::WindowEvent;
//-----------------------------------------------------------------------------
/// Camera transformer which moves the camera along camera's axis
pub struct Flier {
    moving_forward: bool,
    moving_back: bool,
    moving_left: bool,
    moving_right: bool,
    moving_up: bool,
    moving_down: bool,

    moving_speed: f32,
}

//-----------------------------------------------------------------------------

impl Flier {
    pub fn moving_speed(&self) -> f32 {
        return self.moving_speed;
    }

    pub fn set_moving_speed(&mut self, speed: f32) {
        self.moving_speed = speed.abs();
    }
}

//-----------------------------------------------------------------------------

impl Flier {
    pub fn new() -> Self {
        return Flier {
            moving_forward: false,
            moving_back: false,
            moving_left: false,
            moving_right: false,
            moving_up: false,
            moving_down: false,

            moving_speed: 0.2,
        };
    }

    pub fn on_window_event(&mut self, event: &WindowEvent) {
        match *event {
            /*
             * Move camera on WASD
             */
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key: winit::keyboard::PhysicalKey::Code(key_code),
                        state,
                        ..
                    },
                ..
            } => {
                use winit::keyboard::KeyCode;

                match key_code {
                    KeyCode::KeyW => {
                        self.moving_forward = state.is_pressed();
                    }
                    KeyCode::KeyS => {
                        self.moving_back = state.is_pressed();
                    }
                    KeyCode::KeyA => {
                        self.moving_left = state.is_pressed();
                    }
                    KeyCode::KeyD => {
                        self.moving_right = state.is_pressed();
                    }
                    KeyCode::KeyR => {
                        self.moving_up = state.is_pressed();
                    }
                    KeyCode::KeyF => {
                        self.moving_down = state.is_pressed();
                    }
                    _ => {}
                }
            }
            /*
             * Update speed on mouse wheel
             */
            WindowEvent::MouseWheel {
                device_id: _,
                delta: winit::event::MouseScrollDelta::LineDelta(_dx, dy),
                phase: _,
            } => {
                self.moving_speed = f32::exp(f32::ln(self.moving_speed) + dy * 0.1);
            }
            _ => {}
        }
    }

    pub fn move_camera(&mut self, camera: &mut super::Camera, delta: f32) {
        let velocity = self.moving_speed * delta;

        let mut pos = camera.pos();

        let right = -camera.axis().col(0);
        let up = camera.axis().col(1);
        let forward = camera.axis().col(2);

        if self.moving_forward {
            pos += forward * velocity;
        }
        if self.moving_back {
            pos -= forward * velocity;
        }

        if self.moving_right {
            pos += right * velocity;
        }
        if self.moving_left {
            pos -= right * velocity;
        }

        if self.moving_up {
            pos += up * velocity;
        }
        if self.moving_down {
            pos -= up * velocity;
        }

        camera.set_pos(pos);
    }
}

//-----------------------------------------------------------------------------

impl Default for Flier {
    fn default() -> Self {
        return Self::new();
    }
}

//-----------------------------------------------------------------------------
