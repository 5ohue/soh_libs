//-----------------------------------------------------------------------------
use anyhow::Result;
use ash::vk;
//-----------------------------------------------------------------------------
const DYNAMIC_STATES: &[vk::DynamicState] =
    &[vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
//-----------------------------------------------------------------------------

pub struct Pipeline {
    device: crate::DeviceRef,

    pipeline: vk::Pipeline,
    pipeline_layout: vk::PipelineLayout,
    blend_mode: BlendMode,
}

//-----------------------------------------------------------------------------
/// Common blending modes
#[derive(Clone, Copy, Debug, Default)]
pub enum BlendMode {
    /// No blending
    #[default]
    None,
    /// Standard alpha blending
    Alpha,
    /// Additive blending
    Additive,
    /// Multiplicative blending
    Multiply,
    /// Custom blending with specific blend factors and operations
    Custom {
        src_color_factor: vk::BlendFactor,
        dst_color_factor: vk::BlendFactor,
        color_op: vk::BlendOp,
        src_alpha_factor: vk::BlendFactor,
        dst_alpha_factor: vk::BlendFactor,
        alpha_op: vk::BlendOp,
    },
}

//-----------------------------------------------------------------------------
// Getters
impl Pipeline {
    pub fn layout(&self) -> vk::PipelineLayout {
        return self.pipeline_layout;
    }
    pub fn blend_mode(&self) -> BlendMode {
        return self.blend_mode;
    }
}

// Constructor, destructor
impl Pipeline {
    pub fn new(
        device: &crate::DeviceRef,
        render_pass: &crate::RenderPass,
        descriptor_set_layouts: &[&crate::descriptor::SetLayout],
        vertex_descriptions: &[crate::vertex::VertexDescription],
        vertex_shader: &crate::Shader,
        fragment_shader: &crate::Shader,
        blend_mode: BlendMode,
    ) -> Result<Self> {
        /*
         * Describe the programmable stages
         */
        let vertex_shader_stage_info = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(**vertex_shader)
            .name(c"main");
        let fragment_shader_stage_info = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(**fragment_shader)
            .name(c"main");

        let shader_stages = [vertex_shader_stage_info, fragment_shader_stage_info];

        /*
         * Describe the dynamic state
         */
        let dynamic_state =
            vk::PipelineDynamicStateCreateInfo::default().dynamic_states(DYNAMIC_STATES);

        /*
         * Describe the layout of the input vertex data
         */
        let (binding_descriptions, attribute_descriptions) =
            crate::vertex::get_vk_vertex_description(vertex_descriptions);

        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&binding_descriptions)
            .vertex_attribute_descriptions(&attribute_descriptions);

        /*
         * Input assembly info
         */
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewport_count(1)
            .scissor_count(1);

        /*
         * Rasterizer
         */
        let rasterizer = vk::PipelineRasterizationStateCreateInfo::default()
            .depth_clamp_enable(false) // Discard fragments beyond near and far planes
            .rasterizer_discard_enable(false) // Do not disable output to frame buffer
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::FRONT) // Backface culling
            .front_face(vk::FrontFace::CLOCKWISE)
            .depth_bias_enable(false);

        /*
         * Multisampling
         */
        let multisampling = vk::PipelineMultisampleStateCreateInfo::default()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        /*
         * Color blending
         */
        let color_blend_attachment = blend_mode.to_vk_attachment();

        let color_blending = vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .attachments(std::slice::from_ref(&color_blend_attachment))
            .blend_constants([0.0; 4]);

        /*
         * Pipeline layout
         */
        let descriptor_set_layouts = crate::get_handles_vec(descriptor_set_layouts);

        let pipeline_layout_create_info =
            vk::PipelineLayoutCreateInfo::default().set_layouts(&descriptor_set_layouts);

        let pipeline_layout =
            unsafe { device.create_pipeline_layout(&pipeline_layout_create_info, None)? };

        let pipeline_create_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisampling)
            .color_blend_state(&color_blending)
            .dynamic_state(&dynamic_state)
            .layout(pipeline_layout)
            .render_pass(**render_pass)
            .subpass(0);

        let graphics_pipeline = unsafe {
            device
                .create_graphics_pipelines(
                    vk::PipelineCache::null(),
                    std::slice::from_ref(&pipeline_create_info),
                    None,
                )
                .map_err(|(_, e)| e)?
        }[0];

        return Ok(Pipeline {
            device: device.clone(),
            pipeline: graphics_pipeline,
            pipeline_layout,
            blend_mode,
        });
    }

    pub fn destroy(&self) {
        unsafe {
            self.device.destroy_pipeline(self.pipeline, None);
            self.device
                .destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
}

//-----------------------------------------------------------------------------

impl BlendMode {
    /// Convert BlendMode to Vulkan blend state
    fn to_vk_attachment(self) -> vk::PipelineColorBlendAttachmentState {
        /*** Blending pseudocode: ***/
        /*
         * if (blendEnable) {
         *     finalColor.rgb = (srcColorBlendFactor * newColor.rgb) <colorBlendOp> (dstColorBlendFactor * oldColor.rgb);
         *     finalColor.a   = (srcAlphaBlendFactor * newColor.a)   <alphaBlendOp> (dstAlphaBlendFactor * oldColor.a);
         * } else {
         *     finalColor = newColor;
         * }
         *
         * finalColor = finalColor & colorWriteMask
         */

        return match self {
            BlendMode::None => vk::PipelineColorBlendAttachmentState::default()
                .color_write_mask(vk::ColorComponentFlags::RGBA)
                .blend_enable(false),

            BlendMode::Alpha => vk::PipelineColorBlendAttachmentState::default()
                .color_write_mask(vk::ColorComponentFlags::RGBA)
                .blend_enable(true)
                .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
                .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
                .color_blend_op(vk::BlendOp::ADD)
                .src_alpha_blend_factor(vk::BlendFactor::ONE)
                .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
                .alpha_blend_op(vk::BlendOp::ADD),

            BlendMode::Additive => vk::PipelineColorBlendAttachmentState::default()
                .color_write_mask(vk::ColorComponentFlags::RGBA)
                .blend_enable(true)
                .src_color_blend_factor(vk::BlendFactor::ONE)
                .dst_color_blend_factor(vk::BlendFactor::ONE)
                .color_blend_op(vk::BlendOp::ADD)
                .src_alpha_blend_factor(vk::BlendFactor::ONE)
                .dst_alpha_blend_factor(vk::BlendFactor::ZERO) // Keep the source alpha factor
                .alpha_blend_op(vk::BlendOp::ADD),

            BlendMode::Multiply => vk::PipelineColorBlendAttachmentState::default()
                .color_write_mask(vk::ColorComponentFlags::RGBA)
                .blend_enable(true)
                .src_color_blend_factor(vk::BlendFactor::DST_COLOR)
                .dst_color_blend_factor(vk::BlendFactor::ZERO)
                .color_blend_op(vk::BlendOp::ADD)
                .src_alpha_blend_factor(vk::BlendFactor::ONE)
                .dst_alpha_blend_factor(vk::BlendFactor::ZERO) // Keep the source alpha factor
                .alpha_blend_op(vk::BlendOp::ADD),

            BlendMode::Custom {
                src_color_factor,
                dst_color_factor,
                color_op,
                src_alpha_factor,
                dst_alpha_factor,
                alpha_op,
            } => vk::PipelineColorBlendAttachmentState::default()
                .color_write_mask(vk::ColorComponentFlags::RGBA)
                .blend_enable(true)
                .src_color_blend_factor(src_color_factor)
                .dst_color_blend_factor(dst_color_factor)
                .color_blend_op(color_op)
                .src_alpha_blend_factor(src_alpha_factor)
                .dst_alpha_blend_factor(dst_alpha_factor)
                .alpha_blend_op(alpha_op),
        };
    }
}

//-----------------------------------------------------------------------------
// Deref
impl std::ops::Deref for Pipeline {
    type Target = vk::Pipeline;

    fn deref(&self) -> &Self::Target {
        return &self.pipeline;
    }
}

//-----------------------------------------------------------------------------
