//-----------------------------------------------------------------------------
use anyhow::Result;
use ash::vk;
//-----------------------------------------------------------------------------

pub struct SetLayout {
    device: crate::DeviceRef,

    layout: vk::DescriptorSetLayout,
    bindings: Vec<super::SetLayoutBinding>,
}

//-----------------------------------------------------------------------------
// Getters
impl SetLayout {
    pub fn bindings(&self) -> &[super::SetLayoutBinding] {
        return &self.bindings;
    }
}

//-----------------------------------------------------------------------------
// Constructor
impl SetLayout {
    pub fn new(device: &crate::DeviceRef, bindings: &[super::SetLayoutBinding]) -> Result<Self> {
        let vk_bindings = bindings
            .iter()
            .enumerate()
            .map(|(idx, binding)| {
                return vk::DescriptorSetLayoutBinding::default()
                    .binding(idx as u32)
                    .descriptor_type(binding.descriptor_type)
                    .descriptor_count(binding.count)
                    .stage_flags(binding.state_flags);
            })
            .collect::<Vec<_>>();

        let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&vk_bindings);

        let layout = unsafe { device.create_descriptor_set_layout(&create_info, None)? };

        return Ok(SetLayout {
            device: device.clone(),
            layout,
            bindings: bindings.to_vec(),
        });
    }
}

//-----------------------------------------------------------------------------
// Drop
impl Drop for SetLayout {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_descriptor_set_layout(self.layout, None);
        }
    }
}

//-----------------------------------------------------------------------------
// Deref
impl std::ops::Deref for SetLayout {
    type Target = vk::DescriptorSetLayout;

    fn deref(&self) -> &Self::Target {
        return &self.layout;
    }
}

//-----------------------------------------------------------------------------
