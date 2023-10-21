use crate::graphics::texture;
use wgpu;

pub struct Material {
    pub name: String,
    pub diffuse_texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
}

impl Material {
    pub async fn load(
        material_name: String,
        file_name: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Material {
        let texture = texture::load_texture(file_name, device, queue)
            .await
            .unwrap();
        let bind_group_layout = texture::Texture::create_texture_bind_group_layout(device);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: None,
        });

        Material {
            name: material_name,
            diffuse_texture: texture,
            bind_group,
        }
    }
}
