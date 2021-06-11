use wgpu::{
    FragmentState,
    MultisampleState,
    PipelineLayout,
    PipelineLayoutDescriptor,
    PrimitiveState,
    RenderPipeline,
    ShaderFlags,
    ShaderModuleDescriptor,
    ShaderSource,
    VertexState,
};

use crate::graphics::{Renderer, Vertex};

pub struct Pipeline {
    pub(in crate::graphics) pipeline_layout: PipelineLayout,
    pub(in crate::graphics) render_pipeline: RenderPipeline,
}

impl Pipeline {
    pub fn new(
        Renderer {
            device,
            swap_chain_format,
            ..
        }: &Renderer,
    ) -> Self {
        // TODO:
        // Pass shader path(s)
        // Pass buffer descriptor(s)

        let shader_module = device.create_shader_module(&ShaderModuleDescriptor {
            label:  None,
            flags:  ShaderFlags::all(),
            source: ShaderSource::Wgsl(include_str!("./shaders/voxel.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                None,
            bind_group_layouts:   &[],
            push_constant_ranges: &[],
        });

        let vertex_state = VertexState {
            module:      &shader_module,
            entry_point: "main",
            buffers:     &[Vertex::buffer_descriptor()],
        };

        let fragment_state = FragmentState {
            module:      &shader_module,
            entry_point: "main",
            targets:     &[(*swap_chain_format).into()],
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
