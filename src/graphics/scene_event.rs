pub enum Event {
    Close(CloseEvent),
    ToggleScene(ToggleSceneEvent),
    SetTargetFps(SetTargetFpsEvent),
    GetFps,
}

pub struct CloseEvent {}

pub struct ToggleSceneEvent {}

pub struct SetTargetFpsEvent {
    pub target_fps: u32,
}
