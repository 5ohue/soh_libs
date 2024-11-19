//-----------------------------------------------------------------------------
mod buffer;
//-----------------------------------------------------------------------------
pub use buffer::Buffer;
//-----------------------------------------------------------------------------
// Getting the format from the type
pub trait ToFormat: Copy {
    fn format() -> crate::Format;
}

macro_rules! impl_to_format {
    ( $($t:ty, $f:tt)* ) => {
        $(
            impl ToFormat for $t {
                fn format() -> crate::Format {
                    crate::Format::$f
                }
            }
        )*
    };
}

impl_to_format!(
    i8,  R8_SINT
    u8,  R8_UINT
    i16, R16_SINT
    u16, R16_UINT
    i32, R32_SINT
    u32, R32_UINT
    f32, R32_SFLOAT
    f64, R64_SFLOAT

    soh_math::Vec2<i8>,  R8G8_SINT
    soh_math::Vec2<u8>,  R8G8_UINT
    soh_math::Vec2<i16>, R16G16_SINT
    soh_math::Vec2<u16>, R16G16_UINT
    soh_math::Vec2<i32>, R32G32_SINT
    soh_math::Vec2<u32>, R32G32_UINT
    soh_math::Vec2<f32>, R32G32_SFLOAT
    soh_math::Vec2<f64>, R64G64_SFLOAT

    soh_math::Vec3<i8>,  R8G8B8_SINT
    soh_math::Vec3<u8>,  R8G8B8_UINT
    soh_math::Vec3<i16>, R16G16B16_SINT
    soh_math::Vec3<u16>, R16G16B16_UINT
    soh_math::Vec3<i32>, R32G32B32_SINT
    soh_math::Vec3<u32>, R32G32B32_UINT
    soh_math::Vec3<f32>, R32G32B32_SFLOAT
    soh_math::Vec3<f64>, R64G64B64_SFLOAT

    soh_math::Vec4<i8>,  R8G8B8A8_SINT
    soh_math::Vec4<u8>,  R8G8B8A8_UINT
    soh_math::Vec4<i16>, R16G16B16A16_SINT
    soh_math::Vec4<u16>, R16G16B16A16_UINT
    soh_math::Vec4<i32>, R32G32B32A32_SINT
    soh_math::Vec4<u32>, R32G32B32A32_UINT
    soh_math::Vec4<f32>, R32G32B32A32_SFLOAT
    soh_math::Vec4<f64>, R64G64B64A64_SFLOAT

    [i8;  2], R8G8_SINT
    [u8;  2], R8G8_UINT
    [i16; 2], R16G16_SINT
    [u16; 2], R16G16_UINT
    [i32; 2], R32G32_SINT
    [u32; 2], R32G32_UINT
    [f32; 2], R32G32_SFLOAT
    [f64; 2], R64G64_SFLOAT

    [i8;  3], R8G8B8_SINT
    [u8;  3], R8G8B8_UINT
    [i16; 3], R16G16B16_SINT
    [u16; 3], R16G16B16_UINT
    [i32; 3], R32G32B32_SINT
    [u32; 3], R32G32B32_UINT
    [f32; 3], R32G32B32_SFLOAT
    [f64; 3], R64G64B64_SFLOAT

    [i8;  4], R8G8B8A8_SINT
    [u8;  4], R8G8B8A8_UINT
    [i16; 4], R16G16B16A16_SINT
    [u16; 4], R16G16B16A16_UINT
    [i32; 4], R32G32B32A32_SINT
    [u32; 4], R32G32B32A32_UINT
    [f32; 4], R32G32B32A32_SFLOAT
    [f64; 4], R64G64B64A64_SFLOAT

    soh_math::Complex<i8>,  R8G8_SINT
    soh_math::Complex<u8>,  R8G8_UINT
    soh_math::Complex<i16>, R16G16_SINT
    soh_math::Complex<u16>, R16G16_UINT
    soh_math::Complex<i32>, R32G32_SINT
    soh_math::Complex<u32>, R32G32_UINT
    soh_math::Complex<f32>, R32G32_SFLOAT
    soh_math::Complex<f64>, R64G64_SFLOAT
);

//-----------------------------------------------------------------------------
// Getting the binding and attribute description
#[derive(Debug, Clone)]
pub struct VertexDescription {
    pub stride: u32,
    pub attribute_descriptions: Vec<AttributeDescription>,
}

#[derive(Debug, Clone)]
pub struct AttributeDescription {
    pub location: u32,
    pub format: crate::Format,
    pub offset: u32,
}

pub trait Vertex: Copy + Sized {
    fn get_vertex_description() -> VertexDescription {
        return VertexDescription {
            stride: size_of::<Self>() as u32,
            attribute_descriptions: Self::get_attribute_description(),
        };
    }
    fn get_attribute_description() -> Vec<AttributeDescription>;
}

impl<T> Vertex for T
where
    T: ToFormat,
{
    fn get_attribute_description() -> Vec<AttributeDescription> {
        return vec![AttributeDescription {
            location: 0,
            format: T::format(),
            offset: 0,
        }];
    }
}

pub(crate) fn get_vk_vertex_description(
    vertex_descriptions: &[VertexDescription],
) -> (
    Vec<ash::vk::VertexInputBindingDescription>,
    Vec<ash::vk::VertexInputAttributeDescription>,
) {
    let mut binding_descriptions = Vec::new();
    let mut attribute_descriptions = Vec::new();

    for (idx, descr) in vertex_descriptions.iter().enumerate() {
        let binding_description = ash::vk::VertexInputBindingDescription {
            binding: idx as u32,
            stride: descr.stride,
            input_rate: ash::vk::VertexInputRate::VERTEX,
        };

        for descr in descr.attribute_descriptions.iter() {
            let attribute_description = ash::vk::VertexInputAttributeDescription {
                location: descr.location,
                binding: idx as u32,
                format: descr.format,
                offset: descr.offset,
            };

            attribute_descriptions.push(attribute_description);
        }

        binding_descriptions.push(binding_description);
    }

    return (binding_descriptions, attribute_descriptions);
}

//-----------------------------------------------------------------------------
