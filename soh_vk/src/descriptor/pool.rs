use anyhow::Result;
use ash::vk;

pub struct Pool {
    device: crate::DeviceRef,

    pool: vk::DescriptorPool,
}

// Constructor, destructor
impl Pool {
    pub fn new(
        device: &crate::DeviceRef,
        num_of_uniform_descriptors: u32,
        max_num_of_sets: u32,
    ) -> Result<Self> {
        /*
         * Configure pool sizes
         */
        let pool_sizes = [
            // Uniform buffer pool size
            vk::DescriptorPoolSize {
                ty: vk::DescriptorType::UNIFORM_BUFFER,
                descriptor_count: num_of_uniform_descriptors,
            },
        ];

        /*
         * Create pool
         */
        let create_info = vk::DescriptorPoolCreateInfo::default()
            .pool_sizes(&pool_sizes)
            .max_sets(max_num_of_sets);

        let pool = unsafe { device.create_descriptor_pool(&create_info, None)? };

        return Ok(Pool {
            device: device.clone(),
            pool,
        });
    }

    pub fn destroy(&self) {
        unsafe {
            self.device.destroy_descriptor_pool(self.pool, None);
        }
    }
}

// Specific implementation
impl Pool {
    pub fn allocate_set(&self, layout: &super::SetLayout) -> Result<super::Set> {
        let alloc_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.pool)
            .set_layouts(std::slice::from_ref(layout));

        let sets = unsafe { self.device.allocate_descriptor_sets(&alloc_info)? };

        let Some(&set) = sets.first() else {
            anyhow::bail!("No descriptor sets were allocated");
        };

        return Ok(super::Set::from_handle(self.device.clone(), set));
    }

    pub fn allocate_sets(
        &self,
        layout: &super::SetLayout,
        count: usize,
    ) -> Result<Vec<super::Set>> {
        let layouts = vec![**layout; count];

        let alloc_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.pool)
            .set_layouts(&layouts);

        let sets = unsafe { self.device.allocate_descriptor_sets(&alloc_info)? };

        anyhow::ensure!(
            sets.len() == layouts.len(),
            "Number of allocated descriptor sets doesn't match the requested count"
        );

        let res = sets
            .iter()
            .map(|set| {
                return super::Set::from_handle(self.device.clone(), *set);
            })
            .collect();

        return Ok(res);
    }
}

// Deref
impl std::ops::Deref for Pool {
    type Target = vk::DescriptorPool;

    fn deref(&self) -> &Self::Target {
        return &self.pool;
    }
}
