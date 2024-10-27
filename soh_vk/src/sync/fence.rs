use anyhow::Result;
use ash::vk;

#[repr(transparent)]
pub struct Fence {
    fence: vk::Fence,
}

// Constructor, destructor
impl Fence {
    pub fn new(device: &crate::Device, signaled: bool) -> Result<Self> {
        let create_info = vk::FenceCreateInfo::default().flags(if signaled {
            vk::FenceCreateFlags::SIGNALED
        } else {
            vk::FenceCreateFlags::default()
        });

        let fence = unsafe { device.create_fence(&create_info, None)? };
        return Ok(Fence { fence });
    }

    pub fn destroy(&self, device: &crate::Device) {
        device.assert_not_destroyed();
        unsafe {
            device.destroy_fence(self.fence, None);
        }
    }
}

// Specific implementation
impl Fence {
    pub fn wait(&self, device: &crate::Device) {
        unsafe {
            let _ = device.wait_for_fences(&[self.fence], true, u64::MAX);
        }
    }

    pub fn reset(&self, device: &crate::Device) {
        unsafe {
            let _ = device.reset_fences(&[self.fence]);
        }
    }
}

// Deref
impl std::ops::Deref for Fence {
    type Target = vk::Fence;

    fn deref(&self) -> &Self::Target {
        return &self.fence;
    }
}
