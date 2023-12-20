#[derive(Default)]
pub struct SceneEvent {
    pub close_event: Option<CloseEvent>,
    pub toggle_scene_event: Option<ToggleSceneEvent>,
    pub set_target_fps_event: Option<SetTargetFpsEvent>,
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

pub struct SetTargetFpsEvent {
    pub target_fps: Option<u32>,
}
impl SetTargetFpsEvent {
    pub fn new(target_fps: Option<u32>) -> SceneEvent {
        SceneEvent {
            set_target_fps_event: Some(SetTargetFpsEvent { target_fps }),
            ..Default::default()
        }
    }
}
