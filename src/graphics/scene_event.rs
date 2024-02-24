pub enum Event {
    Close(CloseEvent),
    ToggleScene(ToggleSceneEvent),
    SetTargetFps(SetTargetFpsEvent),
    LoadModel(LoadModelEvent),
    GetFps,
}

pub struct CloseEvent {}

pub struct ToggleSceneEvent {}

pub struct SetTargetFpsEvent {
    pub target_fps: u32,
}

pub struct LoadModelEvent {
    pub path: String,
}
