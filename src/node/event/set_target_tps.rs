use super::Event;

pub struct SetTargetTpsEvent {
    pub target_tps: Option<u32>,
}

impl SetTargetTpsEvent {
    pub fn new(target_tps: Option<u32>) -> Event {
        Event {
            set_target_tps_event: Some(SetTargetTpsEvent { target_tps }),
            ..Default::default()
        }
    }
}

impl From<SetTargetTpsEvent> for Event {
    fn from(value: SetTargetTpsEvent) -> Self {
        Event {
            set_target_tps_event: Some(value),
            ..Default::default()
        }
    }
}
