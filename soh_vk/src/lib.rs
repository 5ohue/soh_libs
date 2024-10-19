//! Convenient Vulkan wrappers

mod instance;

pub use instance::*;

/// Trait to easily convert wrapper types to VK (ash) types.
///
/// This trait provides a way to convert types from this library to their corresponding
/// Vulkan (ash) types. Implementors of this trait should provide a `to_vk` method
/// that returns a reference to the underlying Vulkan type.
pub trait ToVK {
    /// The type of the underlying Vulkan (ash) type.
    type TypeVK;

    /// Converts the wrapper type to its corresponding Vulkan (ash) type.
    ///
    /// Returns a reference to the underlying Vulkan type.
    fn to_vk(&self) -> &Self::TypeVK;
}
