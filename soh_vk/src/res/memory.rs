//-----------------------------------------------------------------------------
use anyhow::{anyhow, Result};
use ash::vk;
//-----------------------------------------------------------------------------

pub struct Memory {
    device: crate::DeviceRef,

    memory: vk::DeviceMemory,
    data_ptr: *mut std::ffi::c_void,

    properties: crate::MemoryPropertyFlags,
    size: u64,
}

//-----------------------------------------------------------------------------
// Getters
impl Memory {
    pub fn properties(&self) -> crate::MemoryPropertyFlags {
        return self.properties;
    }
    pub fn size(&self) -> u64 {
        return self.size;
    }
    pub fn is_mapped(&self) -> bool {
        return !self.data_ptr.is_null();
    }
    pub fn can_be_mapped(&self) -> bool {
        return self.properties.contains(
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        );
    }
}

//-----------------------------------------------------------------------------
// Specific implementation
impl Memory {
    /// Map the buffer and write data to it
    pub fn map_and_write<T>(&mut self, data: &[T]) -> Result<()>
    where
        T: Copy,
    {
        anyhow::ensure!(
            self.can_be_mapped(),
            "Buffer cannot be mapped to write memory"
        );

        /*
         * Map the memory
         */
        self.map()?;

        /*
         * Write the data to the mapped memory
         */
        let res = self.write(data);

        /*
         * Unmap
         */
        self.unmap();

        return res;
    }

    pub fn map(&mut self) -> Result<()> {
        anyhow::ensure!(
            !self.is_mapped(),
            "Trying to map an already mapped GPU memory"
        );

        self.data_ptr = unsafe {
            self.device
                .map_memory(self.memory, 0, self.size, vk::MemoryMapFlags::empty())?
        };

        return Ok(());
    }

    pub fn unmap(&mut self) {
        assert!(self.is_mapped());

        unsafe {
            self.device.unmap_memory(self.memory);
        }

        self.data_ptr = std::ptr::null_mut();
    }

    /// Write data to mapped memory
    pub fn write<T>(&mut self, data: &[T]) -> Result<()>
    where
        T: Copy,
    {
        let buffer_size = size_of_val(data) as u64;

        anyhow::ensure!(
            self.size >= buffer_size,
            "Buffer memory is smaller than the data that is being written to it"
        );

        anyhow::ensure!(self.is_mapped(), "Trying to write to unmapped GPU memory");

        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr().cast(), self.data_ptr, size_of_val(data));
        }

        return Ok(());
    }

    pub(crate) fn alloc(
        device: &crate::DeviceRef,
        memory_requirements: vk::MemoryRequirements,
        properties: vk::MemoryPropertyFlags,
    ) -> Result<Self> {
        /*
         * Find which GPU memory type to use for allocation
         */
        let Some(memory_type_index) = device
            .physical()
            .find_memory_type(memory_requirements.memory_type_bits, properties)
        else {
            return Err(anyhow!("Failed to find GPU memory type"));
        };

        /*
         * Allocate memory
         */
        let alloc_info = vk::MemoryAllocateInfo::default()
            .allocation_size(memory_requirements.size)
            .memory_type_index(memory_type_index);

        let memory = unsafe { device.allocate_memory(&alloc_info, None)? };

        return Ok(Memory {
            device: device.clone(),
            memory,
            properties,
            size: memory_requirements.size,
            data_ptr: std::ptr::null_mut(),
        });
    }
}

//-----------------------------------------------------------------------------
// Drop
impl Drop for Memory {
    fn drop(&mut self) {
        unsafe {
            self.device.free_memory(**self, None);
        }
    }
}

//-----------------------------------------------------------------------------
// Deref
impl std::ops::Deref for Memory {
    type Target = vk::DeviceMemory;

    fn deref(&self) -> &Self::Target {
        return &self.memory;
    }
}

//-----------------------------------------------------------------------------
