use ash::vk;

pub struct Set {
    device: crate::DeviceRef,

    set: vk::DescriptorSet,
}

// Specific implementation
impl Set {
    /// Write each uniform buffer to it's binding:
    ///
    /// `uniform_buffers` is a slice, where an element has:
    /// 1. Binding number
    /// 2. Array of uniform buffers (for each descriptor in the binding)
    pub fn update_uniform_buffers(
        &mut self,
        uniform_buffers: &[(u32, &[&crate::uniform::Buffer])],
    ) {
        /*
         * Write info for each uniform buffer
         */
        let buffer_infos = uniform_buffers
            .iter()
            .map(|(_, ubs)| {
                return ubs
                    .iter()
                    .map(|&ub| {
                        return vk::DescriptorBufferInfo::default()
                            .buffer(ub.buffer().buffer())
                            .offset(0)
                            .range(ub.buffer().size());
                    })
                    .collect::<Vec<_>>();
            })
            .collect::<Vec<_>>();

        /*
         * Descriptor write instruction for each binding
         */
        let descriptor_writes = uniform_buffers
            .iter()
            .map(|&(binding, _)| {
                return vk::WriteDescriptorSet::default()
                    .dst_set(**self)
                    .dst_binding(binding)
                    .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                    .buffer_info(&buffer_infos[binding as usize]);
            })
            .collect::<Vec<_>>();

        /*
         * Update descriptor set
         */
        unsafe {
            self.device.update_descriptor_sets(&descriptor_writes, &[]);
        }
    }

    pub(super) fn from_handle(device: crate::DeviceRef, set: vk::DescriptorSet) -> Self {
        return Set { device, set };
    }
}

// Deref
impl std::ops::Deref for Set {
    type Target = vk::DescriptorSet;

    fn deref(&self) -> &Self::Target {
        return &self.set;
    }
}
