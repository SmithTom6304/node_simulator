#[derive(Default)]
pub struct SceneEvent {
    pub close_event: Option<CloseEvent>,
    pub toggle_scene_event: Option<ToggleSceneEvent>,
}

pub struct CloseEvent {}

impl CloseEvent {
    pub fn new() -> SceneEvent {
        SceneEvent {
            close_event: Some(CloseEvent {}),
            ..Default::default()
        }
    }
}

pub struct ToggleSceneEvent {}
impl ToggleSceneEvent {
    pub fn new() -> SceneEvent {
        SceneEvent {
            toggle_scene_event: Some(ToggleSceneEvent {}),
            ..Default::default()
        }
    }
}
