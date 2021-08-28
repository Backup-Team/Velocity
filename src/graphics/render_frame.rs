use wgpu::{
    Color as Colour,
    CommandEncoder,
    IndexFormat,
    LoadOp,
    Operations,
    RenderPassColorAttachment as RenderPassColourAttachment,
    RenderPassDescriptor,
    SwapChainFrame,
};

use crate::graphics::{Mesh, Pipeline};

pub struct RenderFrame {
    pub(in crate::graphics) encoder: CommandEncoder,
    pub(in crate::graphics) frame:   SwapChainFrame,
}

impl RenderFrame {
    pub fn render_mesh(&mut self, pipeline: &Pipeline, mesh: &Mesh) {
        let colour_attachments = &[RenderPassColourAttachment {
            view:           &self.frame.output.view,
            resolve_target: None,
            ops:            Operations {
                load:  LoadOp::Clear(Colour::BLACK),
                store: true,
            },
        }];

        let mut render_pass = self.encoder.begin_render_pass(&RenderPassDescriptor {
            label:                    None,
            color_attachments:        colour_attachments,
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&pipeline.render_pipeline);
        render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        render_pass.set_index_buffer(mesh.index_buffer.slice(..), IndexFormat::Uint16);
        render_pass.draw_indexed(0..mesh.index_count, 0, 0..1);
    }
}

// impl<'renderer> Drop for RenderFrame<'renderer> {
//     fn drop(&mut self) {
//         self.renderer
//             .queue
//             .submit(std::iter::once(self.encoder.finish()));
//     }
// }
