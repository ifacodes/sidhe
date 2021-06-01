pub struct Pipeline {
    pub pipeline: wgpu::RenderPipeline,
}

impl<'a> Pipeline {
    pub fn layout_descriptor() -> &'a wgpu::PipelineLayoutDescriptor<'a> {
        &wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        }
    }
    pub fn pipeline_descriptor(
        layout_desc: &'a wgpu::PipelineLayout,
        vertex: wgpu::VertexState<'a>,
        fragment: wgpu::FragmentState<'a>,
    ) -> wgpu::RenderPipelineDescriptor<'a> {
        wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(layout_desc),
            vertex,
            fragment: Some(fragment),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        }
    }
}
