use std::mem;

use bytemuck::{Pod, Zeroable};
use wgpu::{BufferAddress, InputStepMode, VertexAttribute, VertexBufferLayout, VertexFormat};

use crate::maths::Vec3;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub position: Vec3,
    pub colour:   Vec3,
}

impl Vertex {
    pub fn buffer_descriptor<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as BufferAddress,
            step_mode:    InputStepMode::Vertex,
            attributes:   &[
                VertexAttribute {
                    offset:          0,
                    shader_location: 0,
                    format:          VertexFormat::Float32,
                },
                VertexAttribute {
                    offset:          mem::size_of::<Vec3>() as BufferAddress,
                    shader_location: 0,
                    format:          VertexFormat::Float32,
                },
            ],
        }
    }
}
