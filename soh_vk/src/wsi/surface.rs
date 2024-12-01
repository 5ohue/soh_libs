use anyhow::{anyhow, Result};
use ash::vk::{self, Handle};

pub struct Surface {
    instance: crate::InstanceRef,
    surface: vk::SurfaceKHR,
}

// Constructor, destructor
impl Surface {
    pub fn new(instance: &crate::InstanceRef, window: &sdl2::video::Window) -> Result<Surface> {
        let surface = window
            .vulkan_create_surface(instance.handle().as_raw() as usize)
            .map_err(|err_msg| {
                return anyhow!("Failed to create VK surface: {err_msg}");
            })?;

        return Ok(Surface {
            instance: instance.clone(),
            surface: vk::SurfaceKHR::from_raw(surface),
        });
    }

    pub fn destroy(&self) {
        unsafe {
            self.instance
                .instance_surface()
                .destroy_surface(self.surface, None);
        }
    }
}

// Deref
impl std::ops::Deref for Surface {
    type Target = vk::SurfaceKHR;

    fn deref(&self) -> &Self::Target {
        return &self.surface;
    }
}
