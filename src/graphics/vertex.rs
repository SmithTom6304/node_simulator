#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],   // x, y, z in 3D space
    tex_coords: [f32; 2], // tex coords
}
