//-----------------------------------------------------------------------------
use anyhow::Result;
use ash::vk;
//-----------------------------------------------------------------------------

pub struct Pool {
    device: crate::DeviceRef,

    pool: vk::DescriptorPool,
}

//-----------------------------------------------------------------------------
// Builder
pub struct PoolBuilder {
    max_num_of_sets: u32,
    pool_sizes: smallvec::SmallVec<[(vk::DescriptorType, u32); 11]>,
}

impl PoolBuilder {
    pub fn new() -> Self {
        return PoolBuilder {
            max_num_of_sets: 0,
            pool_sizes: smallvec::smallvec![],
        };
    }

    pub fn max_num_of_sets(mut self, count: u32) -> Self {
        assert!(count > 0);
        self.max_num_of_sets = count;
        return self;
    }

    pub fn combined_sampler_descriptor_count(mut self, count: u32) -> Self {
        assert!(count > 0);
        self.pool_sizes
            .push((vk::DescriptorType::COMBINED_IMAGE_SAMPLER, count));
        return self;
    }

    pub fn uniform_descriptor_count(mut self, count: u32) -> Self {
        assert!(count > 0);
        self.pool_sizes
            .push((vk::DescriptorType::UNIFORM_BUFFER, count));
        return self;
    }

    pub fn storage_descriptor_count(mut self, count: u32) -> Self {
        assert!(count > 0);
        self.pool_sizes
            .push((vk::DescriptorType::STORAGE_BUFFER, count));
        return self;
    }

    pub fn build(self, device: &crate::DeviceRef) -> Result<Pool> {
        /*
         * Check values for sanity
         */
        if cfg!(debug_assertions) {
            assert!(self.max_num_of_sets > 0);

            let mut set = std::collections::HashSet::new();
            for (ty, _) in self.pool_sizes.iter() {
                set.insert(ty);
            }

            assert!(set.len() == self.pool_sizes.len());
        }

        /*
         * Configure pool sizes
         */
        let pool_sizes = self
            .pool_sizes
            .iter()
            .map(|&(ty, count)| vk::DescriptorPoolSize {
                ty,
                descriptor_count: count,
            })
            .collect::<smallvec::SmallVec<[_; 11]>>();

        /*
         * Create pool
         */
        let create_info = vk::DescriptorPoolCreateInfo::default()
            .pool_sizes(&pool_sizes)
            .max_sets(self.max_num_of_sets);

        let pool = unsafe { device.create_descriptor_pool(&create_info, None)? };

        return Ok(Pool {
            device: device.clone(),
            pool,
        });
    }
}

impl Default for PoolBuilder {
    fn default() -> Self {
        return Self::new();
    }
}

//-----------------------------------------------------------------------------
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

//-----------------------------------------------------------------------------
// Drop
impl Drop for Pool {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_descriptor_pool(self.pool, None);
        }
    }
}

//-----------------------------------------------------------------------------
// Deref
impl std::ops::Deref for Pool {
    type Target = vk::DescriptorPool;

    fn deref(&self) -> &Self::Target {
        return &self.pool;
    }
}

//-----------------------------------------------------------------------------
