//-----------------------------------------------------------------------------
use anyhow::Result;
use ash::vk;
//-----------------------------------------------------------------------------

pub struct Image {
    device: crate::DeviceRef,

    image: vk::Image,
    format: vk::Format,

    memory: Option<super::Memory>,
}

//-----------------------------------------------------------------------------
// Builder
pub struct ImageBuilder {
    format: vk::Format,
    size: (u32, u32),

    usage: vk::ImageUsageFlags,
    samples: vk::SampleCountFlags,
    tiling: vk::ImageTiling,
    initial_layout: vk::ImageLayout,

    num_of_mip_levels: u32,
    num_of_layers: u32,

    queue_families: Vec<crate::QueueType>,
}

impl ImageBuilder {
    pub fn new() -> Self {
        return ImageBuilder {
            format: vk::Format::R8G8B8A8_SRGB,
            size: (0, 0),

            usage: vk::ImageUsageFlags::empty(),
            samples: vk::SampleCountFlags::TYPE_1,
            tiling: vk::ImageTiling::LINEAR,
            initial_layout: vk::ImageLayout::UNDEFINED,

            num_of_mip_levels: 1,
            num_of_layers: 1,

            queue_families: vec![],
        };
    }

    pub fn format(mut self, format: vk::Format) -> Self {
        self.format = format;
        return self;
    }

    pub fn size(mut self, size: (u32, u32)) -> Self {
        assert!(size.0 > 0);
        assert!(size.1 > 0);
        self.size = size;
        return self;
    }

    pub fn usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.usage = usage;
        return self;
    }

    pub fn samples(mut self, samples: vk::SampleCountFlags) -> Self {
        self.samples = samples;
        return self;
    }

    pub fn tiling(mut self, tiling: vk::ImageTiling) -> Self {
        self.tiling = tiling;
        return self;
    }

    pub fn initial_layout(mut self, initial_layout: vk::ImageLayout) -> Self {
        self.initial_layout = initial_layout;
        return self;
    }

    pub fn mip_levels(mut self, num_of_mip_levels: u32) -> Self {
        assert!(num_of_mip_levels > 0);
        self.num_of_mip_levels = num_of_mip_levels;
        return self;
    }

    pub fn layers(mut self, num_of_layers: u32) -> Self {
        assert!(num_of_layers > 0);
        self.num_of_layers = num_of_layers;
        return self;
    }

    pub fn queue_families(mut self, queue_families: Vec<crate::QueueType>) -> Self {
        self.queue_families = queue_families;
        return self;
    }

    pub fn build(self, device: &crate::DeviceRef) -> Result<Image> {
        /*
         * Collect queue family indexes
         */
        let queue_families = self
            .queue_families
            .iter()
            .map(|&ty| device.physical().queue_family_idx(ty))
            .collect::<std::collections::HashSet<_>>() // Make unique
            .iter()
            .copied()
            .collect::<Vec<_>>();

        let is_concurrent = queue_families.len() > 1;

        /*
         * Image create info
         */
        let create_info = vk::ImageCreateInfo::default()
            .format(self.format)
            .extent(vk::Extent3D {
                width: self.size.0,
                height: self.size.1,
                depth: 1,
            })
            .image_type(vk::ImageType::TYPE_2D)
            .usage(self.usage)
            .samples(self.samples)
            .tiling(self.tiling)
            .initial_layout(self.initial_layout)
            .mip_levels(self.num_of_mip_levels)
            .array_layers(self.num_of_layers)
            .sharing_mode(if is_concurrent {
                vk::SharingMode::CONCURRENT
            } else {
                vk::SharingMode::EXCLUSIVE
            })
            .queue_family_indices(&queue_families);

        /*
         * Create image
         */
        let image = unsafe { device.create_image(&create_info, None)? };

        return Ok(Image {
            device: device.clone(),
            image,
            format: self.format,
            memory: None,
        });
    }
}

impl Default for ImageBuilder {
    fn default() -> Self {
        return Self::new();
    }
}

//-----------------------------------------------------------------------------
// Getters
impl Image {
    pub fn image(&self) -> vk::Image {
        return self.image;
    }
    pub fn format(&self) -> vk::Format {
        return self.format;
    }
    pub fn memory(&self) -> Option<&super::Memory> {
        return self.memory.as_ref();
    }
}

//-----------------------------------------------------------------------------
// Specific implementation
impl Image {
    pub fn allocate_memory(&mut self, properties: vk::MemoryPropertyFlags) -> Result<()> {
        /*
         * Get memory requirements
         */
        let memory_requirements = unsafe { self.device.get_image_memory_requirements(self.image) };

        /*
         * Allocate memory
         */
        let memory = super::Memory::alloc(&self.device, memory_requirements, properties)?;

        /*
         * Bind allocated memory to image
         */
        unsafe {
            self.device.bind_image_memory(self.image, *memory, 0)?;
        }

        /*
         * Set memory
         */
        self.memory = Some(memory);

        return Ok(());
    }

    pub fn free_memory(&mut self) {
        self.memory = None;
    }
}

//-----------------------------------------------------------------------------
// Drop
impl Drop for Image {
    fn drop(&mut self) {
        self.free_memory();
        unsafe {
            self.device.destroy_image(self.image, None);
        }
    }
}

//-----------------------------------------------------------------------------
// Deref
impl std::ops::Deref for Image {
    type Target = vk::Image;

    fn deref(&self) -> &Self::Target {
        return &self.image;
    }
}

//-----------------------------------------------------------------------------
