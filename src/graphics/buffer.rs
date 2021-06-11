use wgpu::Buffer as WgpuBuffer;

pub struct Buffer {
    pub(in crate::graphics) buffer: WgpuBuffer,
    pub(in crate::graphics) size:   u32,
}
