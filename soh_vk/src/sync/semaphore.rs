use anyhow::Result;
use ash::vk;

#[repr(transparent)]
pub struct Semaphore {
    semaphore: vk::Semaphore,
}

// Constructor, destructor
impl Semaphore {
    pub fn new(device: &crate::Device) -> Result<Self> {
        let create_info = vk::SemaphoreCreateInfo::default();

        let semaphore = unsafe { device.create_semaphore(&create_info, None)? };
        return Ok(Semaphore { semaphore });
    }

    pub fn destroy(&self, device: &crate::Device) {
        device.assert_not_destroyed();
        unsafe {
            device.destroy_semaphore(**self, None);
        }
    }
}

// Deref
impl std::ops::Deref for Semaphore {
    type Target = vk::Semaphore;

    fn deref(&self) -> &Self::Target {
        return &self.semaphore;
    }
}
