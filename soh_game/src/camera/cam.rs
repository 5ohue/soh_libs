//-----------------------------------------------------------------------------
use soh_math::{Mat3, Mat4, Vec3};
//-----------------------------------------------------------------------------
/// Camera struct
///
/// This struct which has:
/// 1. Position and look direction axis
///    Axis are (in order): left, up, forward
/// 2. FOV, aspect ratio, near and far planes
pub struct Camera {
    /*
     * Camera info
     */
    pos: Vec3<f32>,
    axis: Mat3<f32>,

    /*
     * Projection info
     */
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,

    /*
     * Precalculated matrixes
     */
    was_view_updated: bool,
    was_proj_updated: bool,
    view: Mat4<f32>,
    proj: Mat4<f32>,
}

//-----------------------------------------------------------------------------
// Constructor
impl Camera {
    pub fn new() -> Self {
        let camera = Camera {
            pos: Vec3::zero(),
            axis: Mat3::identity(),

            fov: 70.0,
            aspect: 1.0,
            near: 0.01,
            far: 10.0,

            was_view_updated: true,
            was_proj_updated: true,
            view: Mat4::identity(),
            proj: Mat4::identity(),
        };

        return camera;
    }
}

//-----------------------------------------------------------------------------
// Getters
impl Camera {
    /*
     * View
     */
    pub fn pos(&self) -> Vec3<f32> {
        return self.pos;
    }

    pub fn axis(&self) -> Mat3<f32> {
        return self.axis;
    }

    /*
     * Projection
     */
    pub fn fov(&self) -> f32 {
        return self.fov;
    }

    pub fn aspect(&self) -> f32 {
        return self.aspect;
    }

    pub fn near(&self) -> f32 {
        return self.near;
    }

    pub fn far(&self) -> f32 {
        return self.far;
    }

    pub fn near_far(&self) -> (f32, f32) {
        return (self.near, self.far);
    }

    /*
     * State update
     */
    pub fn was_updated(&self) -> bool {
        return self.was_view_updated || self.was_proj_updated;
    }

    pub fn was_view_updated(&self) -> bool {
        return self.was_view_updated;
    }

    pub fn was_proj_updated(&self) -> bool {
        return self.was_proj_updated;
    }
}

//-----------------------------------------------------------------------------
// Updating camera
impl Camera {
    /*
     * View
     */
    pub fn pos_mut(&mut self) -> &mut Vec3<f32> {
        self.was_view_updated = true;
        return &mut self.pos;
    }

    pub fn axis_mut(&mut self) -> &mut Mat3<f32> {
        self.was_view_updated = true;
        return &mut self.axis;
    }

    pub fn set_pos(&mut self, pos: Vec3<f32>) {
        self.was_view_updated = true;
        self.pos = pos;
    }

    /// Rotate around camera's X axis
    pub fn rotate_view_x(&mut self, angle: f32) {
        self.was_view_updated = true;
        self.axis = Mat3::from_axis_angle(self.axis.col(0), angle) * self.axis;
    }

    /// Rotate around world's Z axis
    pub fn rotate_world_z(&mut self, angle: f32) {
        self.was_view_updated = true;
        self.axis = Mat3::yaw(angle) * self.axis;
    }

    /*
     * Projection
     */
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.was_proj_updated = true;
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
        self.was_proj_updated = true;
    }

    pub fn set_near_far(&mut self, near: f32, far: f32) {
        self.near = near;
        self.far = far;
        self.was_proj_updated = true;
    }
}

//-----------------------------------------------------------------------------
// Getting matrixes for rendering
impl Camera {
    pub fn get_view(&mut self) -> Mat4<f32> {
        if !self.was_view_updated {
            return self.view;
        }

        return self.update_view();
    }

    pub fn get_proj(&mut self) -> Mat4<f32> {
        if !self.was_proj_updated {
            return self.proj;
        }

        return self.update_proj();
    }
}

//-----------------------------------------------------------------------------
// 4x4 matrix implementation
impl Camera {
    // Update the view matrix based on the camera's position and axis
    fn update_view(&mut self) -> Mat4<f32> {
        let axis_inverse = self.axis.t();
        let pos_inverse = -axis_inverse * self.pos;

        self.view = Mat4::from_3x3_vec(axis_inverse, pos_inverse);
        return self.view;
    }

    // Update the projection matrix with a perspective transformation
    fn update_proj(&mut self) -> Mat4<f32> {
        let mut proj = Mat4::perspective(self.fov, self.aspect, self.near, self.far);

        // Flip X and Y axes to invert camera orientation
        *proj.at_mut(0, 0) = -proj.at(0, 0);
        *proj.at_mut(1, 1) = -proj.at(1, 1);

        self.proj = proj;
        return self.proj;
    }
}

//-----------------------------------------------------------------------------

impl Default for Camera {
    fn default() -> Self {
        return Self::new();
    }
}

//-----------------------------------------------------------------------------
