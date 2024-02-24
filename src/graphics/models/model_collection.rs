use std::collections::HashMap;

use super::model;

pub struct ModelCollection {
    models: HashMap<String, model::Model>,
    new_model_id: model::ModelId,
}

impl ModelCollection {
    pub fn new() -> ModelCollection {
        let models = HashMap::new();
        let new_model_id: model::ModelId = model::ModelId(0);
        ModelCollection {
            models,
            new_model_id,
        }
    }

    pub fn add<LM>(&mut self, load_model: LM) -> model::ModelId
    where
        LM: FnOnce(model::ModelId) -> model::Model,
    {
        let id = self.new_model_id;
        self.new_model_id.0 += 1;

        let model = load_model(id);

        let id = model.id;

        self.models.insert(model.path.clone(), model);

        id
    }

    pub fn find(&self, id: model::ModelId) -> Option<&model::Model> {
        self.iter().find(|entry| entry.1.id == id).map(|kvp| kvp.1)
    }

    pub fn remove(&mut self, model_id: model::ModelId) -> bool {
        let size_before = self.models.len();
        self.models.retain(|_, model| model.id != model_id);
        self.models.len() != size_before
    }

    pub fn iter(
        &self,
    ) -> std::collections::hash_map::Iter<
        '_,
        std::string::String,
        crate::graphics::models::model::Model,
    > {
        self.models.iter()
    }
}
