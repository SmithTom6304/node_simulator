use crate::graphics::texture;
use core::fmt;
use std::error::Error;
use wgpu;

pub struct Material {
    pub name: String,
    pub diffuse_texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
}

#[derive(Debug)]
pub struct MaterialError {
    pub message: String,
}

impl fmt::Display for MaterialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MaterialError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl Material {
    pub async fn load(
        material_name: String,
        file_name: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Material, MaterialError> {
        let texture = texture::load_texture(file_name, device, queue).await;

        // TODO Less stupid way to do this
        if texture.is_err() {
            let error = texture.err().unwrap();
            return Err(MaterialError {
                message: error.to_string(),
            });
        }

        let texture = texture.unwrap();

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

        Ok(Material {
            name: material_name,
            diffuse_texture: texture,
            bind_group,
        })
    }
}
