use super::model;

pub struct ModelCollection {
    models: Vec<model::Model>,
    new_model_id: u8,
}

impl ModelCollection {
    pub fn new() -> ModelCollection {
        let models = Vec::new();
        let new_model_id = 0;
        ModelCollection {
            models,
            new_model_id,
        }
    }

    pub fn add(&mut self, model_descriptor: model::LoadModelDescriptor) -> u8 {
        let id = self.new_model_id;
        self.new_model_id += 1;

        let model = pollster::block_on(model::load_model(model_descriptor, id));
        let model = model.unwrap();

        let id = model.id;

        self.models.push(model);

        return id;
    }

    pub fn remove(&mut self, model_id: u8) -> bool {
        let size_before = self.models.len();
        self.models.retain(|model| model.id != model_id);
        self.models.len() != size_before
    }

    pub fn iter(&self) -> std::slice::Iter<'_, model::Model> {
        self.models.iter()
    }
}
