//-----------------------------------------------------------------------------
//! Convenient Vulkan wrappers
//-----------------------------------------------------------------------------
// Private modules
mod device;
mod framebuffer;
mod instance;
mod pipeline;
mod queue;
mod surface;
mod swapchain;
//-----------------------------------------------------------------------------
// Public imports
pub use device::*;
pub use framebuffer::*;
pub use instance::*;
pub use pipeline::*;
pub use queue::*;
pub use surface::*;
pub use swapchain::*;
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
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

//-----------------------------------------------------------------------------
