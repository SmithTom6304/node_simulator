use super::model::ModelVertex;
use wgpu::util::DeviceExt;

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}

impl Mesh {
    pub fn load(mesh: tobj::Mesh, name: String, device: &wgpu::Device) -> Mesh {
        let vertices = (0..mesh.positions.len() / 3)
            .map(|i| ModelVertex {
                position: [
                    mesh.positions[i * 3],
                    mesh.positions[i * 3 + 1],
                    mesh.positions[i * 3 + 2],
                ],
                tex_coords: [mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]],
                normal: [
                    mesh.normals[i * 3],
                    mesh.normals[i * 3 + 1],
                    mesh.normals[i * 3 + 2],
                ],
            })
            .collect::<Vec<_>>();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Vertex Buffer", name)),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Index Buffer", name)),
            contents: bytemuck::cast_slice(&mesh.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Mesh {
            name: name.to_string(),
            vertex_buffer,
            index_buffer,
            num_elements: mesh.indices.len() as u32,
            material: mesh.material_id.unwrap_or(0),
        }
    }
}
