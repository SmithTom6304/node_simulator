use node_simulator::graphics::scene_event;

#[derive(clap::Args, Debug)]
pub struct FpsArgs {
    pub target: u32,
}

impl From<&FpsArgs> for scene_event::SetTargetFpsEvent {
    fn from(value: &FpsArgs) -> Self {
        Self {
            target_fps: value.target,
        }
    }
}
