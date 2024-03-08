use super::instance;
use crate::graphics::models::model;
use std::ops::Range;

pub struct InstanceCollection {
    pub model: model::ModelId,
    instances: Vec<instance::Instance>,
}

pub struct InstanceRenderData {
    pub data: Vec<instance::InstanceRaw>,
    pub indexes: Vec<(model::ModelId, Range<u32>)>,
}

impl InstanceCollection {
    pub fn new(model: model::ModelId) -> InstanceCollection {
        let instances = Vec::new();
        InstanceCollection { model, instances }
    }

    pub fn add(&mut self, instance: instance::Instance) {
        self.instances.push(instance);
    }

    pub fn remove(&mut self, instance: instance::Instance) {
        self.instances.retain(|ins| ins != &instance);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, instance::Instance> {
        self.instances.iter()
    }

    pub fn clear(&mut self) {
        self.instances.clear()
    }

    pub fn get_instance_render_data(collections: &Vec<&InstanceCollection>) -> InstanceRenderData {
        let mut instance_indexes: Vec<(model::ModelId, Range<u32>)> = vec![];
        collections.iter().for_each(|instances| {
            let index = match instance_indexes.last() {
                None => Range {
                    start: 0 as u32,
                    end: instances.iter().len() as u32,
                },
                Some(x) => Range {
                    start: x.1.end,
                    end: x.1.end + instances.iter().len() as u32,
                },
            };
            instance_indexes.push((instances.model, index))
        });

        let instance_data = collections
            .iter()
            .flat_map(|x| x.iter())
            .map(instance::Instance::to_raw)
            .collect::<Vec<_>>();

        InstanceRenderData {
            data: instance_data,
            indexes: instance_indexes,
        }
    }
}
