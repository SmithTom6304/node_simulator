pub mod add_node;
pub mod remove_node;
pub mod set_target_tps;

use add_node::AddNodeEvent;
use remove_node::RemoveNodeEvent;
use set_target_tps::SetTargetTpsEvent;

pub enum Event {
    AddNode(AddNodeEvent),
    RemoveNode(RemoveNodeEvent),
    SetTargetTps(SetTargetTpsEvent),
}
