use anyhow::Result;
use ash::vk;

const DYNAMIC_STATES: &[vk::DynamicState] =
    &[vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];

pub struct Pipeline {
    device: crate::DeviceRef,

    pipeline: vk::Pipeline,
    pipeline_layout: vk::PipelineLayout,
}

// Constructor, destructor
impl Pipeline {
    pub fn new(
        device: &crate::DeviceRef,
        render_pass: &crate::RenderPass,
        vertex_descriptions: &[crate::vertex::VertexDescription],
        vertex_shader: &crate::Shader,
        fragment_shader: &crate::Shader,
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
            .cull_mode(vk::CullModeFlags::BACK) // Backface culling
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
        let color_blend_attachment = vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(vk::ColorComponentFlags::RGBA)
            .blend_enable(false)
            .src_color_blend_factor(vk::BlendFactor::ONE)
            .dst_color_blend_factor(vk::BlendFactor::ZERO)
            .color_blend_op(vk::BlendOp::ADD)
            .src_alpha_blend_factor(vk::BlendFactor::ONE)
            .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
            .alpha_blend_op(vk::BlendOp::ADD);
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

        let color_blending = vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .attachments(std::slice::from_ref(&color_blend_attachment))
            .blend_constants([0.0; 4]);

        /*
         * Pipeline layout
         */
        let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo::default();
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

// Deref
impl std::ops::Deref for Pipeline {
    type Target = vk::Pipeline;

    fn deref(&self) -> &Self::Target {
        return &self.pipeline;
    }
}
