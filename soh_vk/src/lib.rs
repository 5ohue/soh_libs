//-----------------------------------------------------------------------------
//! Convenient Vulkan wrappers
//-----------------------------------------------------------------------------
// Private modules
mod device;
mod framebuffer;
mod instance;
mod pipeline;
mod surface;
mod swapchain;
//-----------------------------------------------------------------------------
// Public imports
pub use device::*;
pub use framebuffer::*;
pub use instance::*;
pub use pipeline::*;
pub use surface::*;
pub use swapchain::*;
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// Typedefs
pub mod typedefs;
pub use typedefs::*;

// Shader related structures
pub mod shader;
pub use shader::Shader;

// Debug messenger
pub mod debug;

// Allocated resources (buffers, images)
pub mod res;
pub use res::*;

// Command pool and buffer
pub mod command;

// Synchronization promitives (fences, semaphores)
pub mod sync;

// Vertex trait and vertex buffer
pub mod vertex;
pub use soh_vk_derive::Vertex;
pub use vertex::Vertex;

//-----------------------------------------------------------------------------
// Helps to easily get a handle from a Option<&WrapperType>
fn get_opt_handle<T, H>(opt: Option<&T>) -> H
where
    T: std::ops::Deref<Target = H>,
    H: ash::vk::Handle + Copy,
{
    return match opt {
        Some(obj) => **obj,
        None => H::from_raw(0),
    };
}

//-----------------------------------------------------------------------------
