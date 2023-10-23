use std::{
    io::{BufReader, Cursor},
    ops::Range,
};

use crate::graphics::vertex;
use crate::resources;

use super::{material, mesh};

pub struct Model {
    pub id: ModelId,
    pub meshes: Vec<mesh::Mesh>,
    pub materials: Vec<material::Material>,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct ModelId(pub u32);

pub struct LoadModelDescriptor<'a> {
    pub file_name: &'a str,
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

impl vertex::Vertex for ModelVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub async fn load_model(descriptor: LoadModelDescriptor<'_>, id: ModelId) -> anyhow::Result<Model> {
    let obj_text = resources::load_string(descriptor.file_name).await?;
    let obj_cursor = Cursor::new(obj_text);
    let mut obj_reader = BufReader::new(obj_cursor);

    let (obj_models, obj_materials) = tobj::load_obj_buf_async(
        &mut obj_reader,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
        |p| async move {
            let mat_text = resources::load_string(&p).await.unwrap();
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        },
    )
    .await?;

    let mut materials = Vec::new();
    for m in obj_materials? {
        let material = material::Material::load(
            m.name,
            &m.diffuse_texture,
            descriptor.device,
            descriptor.queue,
        )
        .await;
        match material {
            Ok(mat) => materials.push(mat),
            Err(err) => println!(
                "Error loading material from file {} - {}",
                &m.diffuse_texture, err.message
            ),
        };
    }

    let meshes = obj_models
        .into_iter()
        .map(|obj_model| {
            mesh::Mesh::load(
                obj_model.mesh,
                descriptor.file_name.to_string(),
                descriptor.device,
            )
        })
        .collect::<Vec<_>>();

    Ok(Model {
        meshes,
        materials,
        id,
    })
}

pub trait DrawModel<'a> {
    fn draw_mesh(
        &mut self,
        mesh: &'a mesh::Mesh,
        material: &'a material::Material,
        camera_bind_group: &'a wgpu::BindGroup,
    );
    fn draw_mesh_instanced(
        &mut self,
        mesh: &'a mesh::Mesh,
        material: &'a material::Material,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
    );
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh(
        &mut self,
        mesh: &'b mesh::Mesh,
        material: &'b material::Material,
        camera_bind_group: &'b wgpu::BindGroup,
    ) {
        self.draw_mesh_instanced(mesh, material, 0..1, camera_bind_group);
    }

    fn draw_mesh_instanced(
        &mut self,
        mesh: &'b mesh::Mesh,
        material: &'b material::Material,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, &material.bind_group, &[]);
        self.set_bind_group(1, camera_bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }
}

impl<'a> LoadModelDescriptor<'a> {
    pub fn new(
        file_name: &'a str,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
    ) -> LoadModelDescriptor<'a> {
        LoadModelDescriptor {
            file_name,
            device,
            queue,
        }
    }
}
