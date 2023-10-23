extern crate utilities;
use super::*;
#[test]
fn can_create_a_new_load_model_descriptor() {
    let instance_data = utilities::common::instance_data::InstanceData::new();

    let file_name = String::from("cube.obj");
    let device = instance_data.device();
    let queue = instance_data.queue();

    let load_model_descriptor = model::LoadModelDescriptor::new(&file_name, device, queue);

    assert_eq!(file_name, load_model_descriptor.file_name);
}
