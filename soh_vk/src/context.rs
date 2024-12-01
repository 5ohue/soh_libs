//-----------------------------------------------------------------------------
use anyhow::Result;
use soh_log::LogError;
//-----------------------------------------------------------------------------

pub struct ContextBootstrapInfo<'a> {
    /*
     * Instance info
     */
    pub app_name: &'a str,
    pub app_version: (u32, u32, u32),
    pub debug_messenger_callback: crate::debug::MessengerCallback,

    /*
     * Window
     */
    pub window: &'a sdl2::video::Window,

    /*
     * Frame info
     */
    pub num_of_frames_in_flight: usize,

    /*
     * Shader info
     */
    pub shader_manager_mode: crate::shader::Mode,
    pub recompile_shaders: bool,
    pub shader_directory: &'a str,
}

/// Struct which contains the vulkan context information
///
/// This includes
/// 1. Vulkan instance, device;
/// 2. WSI stuff (surface, swapchain, framebuffers, etc...)
/// 3. Command pools and buffer
/// 4. Synchronization objects
/// 5. Shader manager
pub struct VulkanContext {
    /*
     * Vulkan stuff
     */
    instance: crate::InstanceRef,
    debug_messenger: Option<crate::debug::Messenger>,
    device: crate::DeviceRef,

    /*
     * WSI
     */
    surface: crate::Surface,
    swapchain: crate::Swapchain,
    render_pass: crate::RenderPass,
    framebuffers: Vec<crate::Framebuffer>,

    /*
     * Command pools and buffer
     */
    cmd_pool_graphics: crate::cmd::Pool,
    cmd_pool_transfer: crate::cmd::Pool,
    cmd_buffers: Vec<crate::cmd::Buffer>,

    /*
     * Synchronization objects
     */
    image_available_semaphores: Vec<crate::sync::Semaphore>,
    render_finished_semaphores: Vec<crate::sync::Semaphore>,
    in_flight_fences: Vec<crate::sync::Fence>,

    /*
     * Shader manager
     */
    shader_manager: crate::shader::Manager,
}

/// Structure containing data needed to render a frame
pub struct PerFrameData<'a> {
    pub context: &'a VulkanContext,

    pub frame_idx: usize,
    pub image_idx: usize,

    pub framebuffer: &'a crate::Framebuffer,
    pub cmd_buffer: &'a crate::cmd::Buffer,
}

//-----------------------------------------------------------------------------

// Getters
impl VulkanContext {
    pub fn instance(&self) -> &crate::InstanceRef {
        &self.instance
    }
    pub fn device(&self) -> &crate::DeviceRef {
        &self.device
    }

    pub fn surface(&self) -> &crate::Surface {
        &self.surface
    }
    pub fn swapchain(&self) -> &crate::Swapchain {
        &self.swapchain
    }
    pub fn render_pass(&self) -> &crate::RenderPass {
        &self.render_pass
    }
    pub fn framebuffers(&self) -> &[crate::Framebuffer] {
        &self.framebuffers
    }

    /// # Safety
    ///
    /// Should only call this in main thread
    pub unsafe fn cmd_pool_graphics(&self) -> &crate::cmd::Pool {
        return &self.cmd_pool_graphics;
    }
    /// # Safety
    ///
    /// Should only call this in main thread
    pub unsafe fn cmd_pool_transfer(&self) -> &crate::cmd::Pool {
        return &self.cmd_pool_transfer;
    }

    pub fn num_of_frames_in_flight(&self) -> usize {
        return self.in_flight_fences.len();
    }

    pub fn shader_manager(&self) -> &crate::shader::Manager {
        &self.shader_manager
    }
}

// Constructor, destructor
impl VulkanContext {
    pub fn bootstrap(bootstrap_info: ContextBootstrapInfo) -> Result<VulkanContext> {
        let num_of_frames = bootstrap_info.num_of_frames_in_flight as u32;

        crate::debug::setup_messenger(bootstrap_info.debug_messenger_callback);

        let instance = Self::create_instance(&bootstrap_info)?;
        let debug_messenger = crate::debug::Messenger::new(&instance).ok();

        let surface = crate::Surface::new(&instance, bootstrap_info.window)?;

        let device = crate::Device::new(&instance, &surface)?;

        let swapchain =
            crate::Swapchain::new(&device, &surface, bootstrap_info.window.drawable_size())?;
        let render_pass = crate::RenderPass::new_simple(&device, swapchain.image_format())?;
        let framebuffers =
            crate::Framebuffer::new_from_swapchain(&device, &swapchain, &render_pass)?;

        let cmd_pool_graphics = crate::cmd::Pool::new_graphics(&device)?;
        let cmd_pool_transfer = crate::cmd::Pool::new_transfer(&device)?;
        let cmd_buffers =
            cmd_pool_graphics.allocate_buffers(crate::cmd::BufferLevel::Primary, num_of_frames)?;

        let image_available_semaphores = (0..num_of_frames)
            .map(|_| crate::sync::Semaphore::new(&device).unwrap_log())
            .collect();
        let render_finished_semaphores = (0..num_of_frames)
            .map(|_| crate::sync::Semaphore::new(&device).unwrap_log())
            .collect();
        let in_flight_fences = (0..num_of_frames)
            .map(|_| crate::sync::Fence::new(&device, true).unwrap_log())
            .collect();

        let shader_manager = crate::shader::Manager::new(
            bootstrap_info.shader_manager_mode,
            bootstrap_info.recompile_shaders,
            bootstrap_info.shader_directory.to_owned(),
        )?;

        return Ok(VulkanContext {
            instance,
            debug_messenger,
            device,

            surface,
            swapchain,
            render_pass,
            framebuffers,

            cmd_pool_graphics,
            cmd_pool_transfer,
            cmd_buffers,

            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences,

            shader_manager,
        });
    }

    pub fn destroy(&self) {
        self.device.wait_idle();

        for i in 0..self.num_of_frames_in_flight() {
            self.in_flight_fences[i].destroy();
            self.render_finished_semaphores[i].destroy();
            self.image_available_semaphores[i].destroy();
        }

        self.cmd_pool_transfer.destroy();
        self.cmd_pool_graphics.destroy();

        for framebuffer in self.framebuffers.iter() {
            framebuffer.destroy();
        }
        self.render_pass.destroy();
        self.swapchain.destroy();
        self.surface.destroy();

        if let Some(ref debug_messenger) = self.debug_messenger {
            debug_messenger.destroy();
        }
    }
}

// Specific implementation
impl VulkanContext {
    /// Returns true if swapchain should be recreated
    pub fn on_frame<F>(&self, frame_idx: usize, user_draw_func: F) -> Result<bool>
    where
        F: FnOnce(PerFrameData<'_>) -> Result<()>,
    {
        /*
         * Get current frame index
         */
        let frame = frame_idx % self.num_of_frames_in_flight();

        /*
         * Get object references
         */
        let cmd_buffer = &self.cmd_buffers[frame];
        let image_available_semaphore = &self.image_available_semaphores[frame];
        let render_finished_semaphore = &self.render_finished_semaphores[frame];
        let in_flight_fence = &self.in_flight_fences[frame];

        /*
         * Wait for the frame to finish rendering
         */
        in_flight_fence.wait();

        /*
         * Acquire an image from the swapchain
         */
        let res = self
            .swapchain
            .acquire_next_image(Some(image_available_semaphore), None);

        let image_idx = match res {
            // Acquired image successfully
            Ok((image_idx, false)) => image_idx as usize,
            // Swapchain should be resized
            Ok((_, true)) | Err(ash::vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                return Ok(true);
            }
            // Error occured
            Err(e) => {
                return Err(e.into());
            }
        };

        /*
         * Reset the fence
         *
         * Only reset the fence if we are submitting work
         * ( to avoid deadlock if couldn't acquire image from swapchaain )
         */
        in_flight_fence.reset();

        /*
         * Prepare the frame data
         */
        let per_frame_data = PerFrameData {
            context: self,
            frame_idx: frame,
            image_idx,

            framebuffer: &self.framebuffers[image_idx],
            cmd_buffer,
        };

        /*
         * Draw the frame
         */
        user_draw_func(per_frame_data)?;

        /*
         * Submit the command buffer to the graphics queue
         */
        cmd_buffer.submit(
            image_available_semaphore,
            render_finished_semaphore,
            Some(in_flight_fence),
        )?;

        /*
         * Present the image to the window
         */
        let present_result = self
            .swapchain
            .present_image(render_finished_semaphore, image_idx as u32);

        return match present_result {
            // Need to recreate swapchain if Error or suboptimal
            Ok(true) | Err(_) => Ok(true),
            // Don't need to recreate swapchain
            Ok(false) => Ok(false),
        };
    }

    pub fn on_window_resize(&mut self, window_size: (u32, u32)) -> Result<()> {
        /*
         * Wait for GPU to finish work
         */
        self.device.wait_idle();

        /*
         * Recreate the swapchain
         */
        self.swapchain.recreate(&self.surface, window_size)?;

        /*
         * Recreate framebuffers
         */
        for framebuffer in self.framebuffers.iter_mut() {
            framebuffer.destroy();
        }

        self.framebuffers = crate::Framebuffer::new_from_swapchain(
            &self.device,
            &self.swapchain,
            &self.render_pass,
        )?;

        return Ok(());
    }

    fn create_instance(bootstrap_info: &ContextBootstrapInfo) -> Result<crate::InstanceRef> {
        fn make_vk_version(tuple_version: (u32, u32, u32)) -> u32 {
            return ash::vk::make_api_version(0, tuple_version.0, tuple_version.1, tuple_version.2);
        }

        fn get_this_crate_version() -> (u32, u32, u32) {
            let version_str = env!("CARGO_PKG_VERSION");
            let mut split = version_str.split('.');

            let major = split.next().unwrap_or("0").parse().unwrap();
            let minor = split.next().unwrap_or("1").parse().unwrap();
            let patch = split.next().unwrap_or("0").parse().unwrap();

            return (major, minor, patch);
        }

        let default_version = ash::vk::make_api_version(0, 1, 0, 0);
        let app_version = make_vk_version(bootstrap_info.app_version);
        let engine_version = make_vk_version(get_this_crate_version());

        let app_name = std::ffi::CString::new(bootstrap_info.app_name)
            .expect("CString::new() failed: `bootstrap_info.app_name`");
        let engine_name = c"SOH";

        let app_info = ash::vk::ApplicationInfo::default()
            .application_name(&app_name)
            .application_version(app_version)
            .engine_name(engine_name)
            .engine_version(engine_version)
            .api_version(default_version);

        let instance = crate::Instance::new(&app_info, bootstrap_info.window)?;

        return Ok(instance);
    }
}

//-----------------------------------------------------------------------------
