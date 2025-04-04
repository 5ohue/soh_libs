//-----------------------------------------------------------------------------
//! Convenient Vulkan wrappers
//-----------------------------------------------------------------------------
// Private modules
mod device;
mod framebuffer;
mod instance;
mod pipeline;
mod render_pass;
//-----------------------------------------------------------------------------
// Public imports
pub use device::*;
pub use framebuffer::*;
pub use instance::*;
pub use pipeline::*;
pub use render_pass::*;
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// Typedefs
pub mod types;
pub use types::*;

// Context (reduce boilerplate)
pub mod context;
pub use context::*;

// Shader related structures
pub mod shader;
pub use shader::Shader;

// Debug messenger
pub mod debug;

// Window system integration
pub mod wsi;
pub use wsi::{Surface, Swapchain};

// Allocated resources (buffers, images)
pub mod res;
pub use res::*;

// Command pool and buffer
pub mod cmd;

// Synchronization promitives (fences, semaphores)
pub mod sync;

// Vertex trait and vertex buffer
pub mod vertex;
pub use soh_vk_derive::Vertex;
pub use vertex::Vertex;

// Index buffer
pub mod index;

// Descripor stuff
pub mod descriptor;
pub use descriptor::uniform;

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

fn get_handles_vec<T, H>(wrappers: &[&T]) -> Vec<H>
where
    T: std::ops::Deref<Target = H>,
    H: ash::vk::Handle + Copy,
{
    return wrappers.iter().map(|w| ***w).collect();
}

//-----------------------------------------------------------------------------
