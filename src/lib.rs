//! Collection of different libraries

#[cfg(feature = "rng")]
pub mod rng {
    pub use soh_rng::*;
}

#[cfg(feature = "log")]
pub mod log {
    pub use soh_log::*;
}

#[cfg(feature = "vk")]
pub mod vk {
    pub use soh_vk::*;
}
