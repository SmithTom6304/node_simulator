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
impl From<CloseEvent> for SceneEvent {
    fn from(value: CloseEvent) -> Self {
        Self {
            close_event: Some(value),
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
impl From<ToggleSceneEvent> for SceneEvent {
    fn from(value: ToggleSceneEvent) -> Self {
        Self {
            toggle_scene_event: Some(value),
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
impl From<SetTargetFpsEvent> for SceneEvent {
    fn from(value: SetTargetFpsEvent) -> Self {
        Self {
            set_target_fps_event: Some(value),
            ..Default::default()
        }
    }
}
