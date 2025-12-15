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
    pub event_loop: &'a winit::event_loop::ActiveEventLoop,
    pub window: &'a winit::window::Window,

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

//-----------------------------------------------------------------------------
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

//-----------------------------------------------------------------------------
// Constructor, destructor
impl VulkanContext {
    pub fn bootstrap(bootstrap_info: ContextBootstrapInfo) -> Result<VulkanContext> {
        let num_of_frames = bootstrap_info.num_of_frames_in_flight as u32;
        let win_size = bootstrap_info.window.inner_size();

        crate::debug::setup_messenger(bootstrap_info.debug_messenger_callback);

        let instance = Self::create_instance(&bootstrap_info)?;
        let debug_messenger = crate::debug::Messenger::new(&instance).ok();

        let surface = crate::Surface::new(&instance, bootstrap_info.window)?;

        let device = crate::Device::new(&instance, &surface)?;

        let swapchain =
            crate::Swapchain::new(&device, &surface, (win_size.width, win_size.height))?;
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
        let render_finished_semaphores = (0..swapchain.num_of_images())
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

        self.in_flight_fences.iter().for_each(|fence| {
            fence.destroy();
        });

        self.image_available_semaphores
            .iter()
            .for_each(|swapchain| {
                swapchain.destroy();
            });

        self.render_finished_semaphores
            .iter()
            .for_each(|swapchain| {
                swapchain.destroy();
            });

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

//-----------------------------------------------------------------------------
// Specific implementation
impl VulkanContext {
    /// Returns true if swapchain should be recreated
    pub fn on_frame<F>(&self, frame_num: usize, user_draw_func: F) -> Result<bool>
    where
        F: FnOnce(PerFrameData<'_>) -> Result<()>,
    {
        /*
         * Get current frame index
         */
        let frame_idx = frame_num % self.num_of_frames_in_flight();

        /*
         * Get object references
         */
        let cmd_buffer = &self.cmd_buffers[frame_idx];
        let image_available_semaphore = &self.image_available_semaphores[frame_idx];
        let in_flight_fence = &self.in_flight_fences[frame_idx];

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
            frame_idx,
            image_idx,

            framebuffer: &self.framebuffers[image_idx],
            cmd_buffer,
        };

        /*
         * Draw the frame
         */
        user_draw_func(per_frame_data)?;

        /*
         * Use image different semaphore per image.
         *
         * This fixes validation error spam due to attempts to signal an already signaled
         * semaphore.
         *
         * If frame X didn't finish rendering yet but we try to render frame X again that results
         * in the same semaphore being used again before it had a chance to be reset. Instead it
         * should use the image specific semaphore. That way rendering synchronization would be
         * image specific instead of frame specific.
         *
         * See https://github.com/Overv/VulkanTutorial/issues/407
         */
        let render_finished_semaphore = &self.render_finished_semaphores[image_idx];

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
        /*
         * Helper functions
         */
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

        /*
         * Create info
         */
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

        /*
         * Deduce platform
         */
        let platform = Self::deduce_platform(bootstrap_info)?;

        let instance = crate::Instance::new(&app_info, platform)?;

        return Ok(instance);
    }

    fn deduce_platform(bootstrap_info: &ContextBootstrapInfo) -> Result<crate::wsi::Platform> {
        let _ = bootstrap_info;

        if cfg!(target_os = "windows") {
            return Ok(crate::wsi::Platform::Win32);
        }

        if cfg!(target_os = "macos") {
            return Ok(crate::wsi::Platform::MacOS);
        }

        if cfg!(target_os = "linux") {
            use winit::platform::{wayland::ActiveEventLoopExtWayland, x11::ActiveEventLoopExtX11};

            let event_loop = bootstrap_info.event_loop;

            if event_loop.is_x11() {
                return Ok(crate::wsi::Platform::X11);
            }
            if event_loop.is_wayland() {
                return Ok(crate::wsi::Platform::Wayland);
            }

            anyhow::bail!("Weird platform on linux: neither X11 nor wayland");
        }

        anyhow::bail!("Unsupported WSI platform");
    }
}

//-----------------------------------------------------------------------------
