pub mod add_node;
pub mod remove_node;
pub mod set_target_tps;

use add_node::AddNodeEvent;
use remove_node::RemoveNodeEvent;
use set_target_tps::SetTargetTpsEvent;

#[derive(Default)]
pub struct Event {
    pub add_node_event: Option<AddNodeEvent>,
    pub remove_node_event: Option<RemoveNodeEvent>,
    pub set_target_tps_event: Option<SetTargetTpsEvent>,
}
