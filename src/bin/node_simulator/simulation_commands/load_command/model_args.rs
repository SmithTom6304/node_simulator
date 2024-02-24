use node_simulator::graphics;

#[derive(clap::Args, Debug)]
pub struct ModelArgs {
    #[arg()]
    /// Path to the model to load. Can be absolute or relative
    pub path: String,
}

impl From<&ModelArgs> for graphics::scene_event::LoadModelEvent {
    fn from(value: &ModelArgs) -> Self {
        Self {
            path: value.path.clone(),
        }
    }
}
