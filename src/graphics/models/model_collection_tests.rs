extern crate utilities;
use utilities::common::{instance_data, texture_bind_group_layout};

use super::*;

#[test]
fn is_empty_on_creation() {
    let model_collection = model_collection::ModelCollection::new();

    assert_eq!(0, model_collection.iter().len());
}

fn load_model(id: model::ModelId) -> super::model::Model {
    let instance_data = instance_data::InstanceData::new();

    let cube_descriptor = model::LoadModelDescriptor::new(
        "cube.obj",
        &instance_data.device(),
        &instance_data.queue(),
    );

    let model = pollster::block_on(model::load_model(cube_descriptor, id));
    let model = model.unwrap();
    return model;
}

#[test]
fn can_add_a_model() {
    let mut model_collection = model_collection::ModelCollection::new();

    model_collection.add(load_model);

    assert_eq!(1, model_collection.iter().len());
}

#[test]
fn returns_a_model_id_when_adding_a_model() {
    let mut model_collection = model_collection::ModelCollection::new();

    let id_1 = model_collection.add(load_model);
    let id_2 = model_collection.add(load_model);

    assert_ne!(id_1, id_2);
}

#[test]
fn can_remove_a_model_by_model_id() {
    let mut model_collection = model_collection::ModelCollection::new();

    let id = model_collection.add(load_model);
    model_collection.remove(id);

    assert_eq!(0, model_collection.iter().len());
}
