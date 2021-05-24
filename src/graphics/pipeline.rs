pub struct Pipeline {
    pipeline_layout: PipelineLayout,
    render_pipeline: RenderPipeline,
}

impl Pipeline {
    fn new(
        Renderer {
            device,
            swap_chain_format,
            ..
        }: &Renderer,
    ) -> Self {
        // TODO:
        // Pass shader path(s)
        // Pass buffer descriptor(s)

        let frag_spirv = include_spirv!("graphics/shaders/test.frag.spv");
        let vert_spirv = include_spirv!("graphics/shaders/test.vert.spv");

        let frag_module = device.create_shader_module(&frag_spirv);
        let vert_module = device.create_shader_module(&vert_spirv);

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                None,
            bind_group_layouts:   &[],
            push_constant_ranges: &[],
        });

        let vertex_state = VertexState {
            module:      &vert_module,
            entry_point: "main",
            buffers:     &[Vertex::buffer_descriptor()],
        };

        let fragment_state = FragmentState {
            module:      &frag_module,
            entry_point: "main",
            targets:     &[swap_chain_format.into()],
        };

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label:         None,
            layout:        Some(&pipeline_layout),
            vertex:        vertex_state,
            fragment:      Some(fragment_state),
            primitive:     PrimitiveState::default(),
            depth_stencil: None,
            multisample:   MultisampleState::default(),
        });

        Self {
            pipeline_layout,
            render_pipeline,
        }
    }
}
