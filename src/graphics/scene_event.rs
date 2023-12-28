pub enum Event {
    Close(CloseEvent),
    ToggleScene(ToggleSceneEvent),
    SetTargetFps(SetTargetFpsEvent),
}

pub struct CloseEvent {}

pub struct ToggleSceneEvent {}

pub struct SetTargetFpsEvent {
    pub target_fps: Option<u32>,
}
