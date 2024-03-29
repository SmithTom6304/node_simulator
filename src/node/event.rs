pub mod add_node;
pub mod get;
pub mod remove_node;
pub mod set_node;
pub mod set_target_tps;
pub mod step;

use add_node::AddNodeEvent;
use get::GetEvent;
use remove_node::RemoveNodeEvent;
use set_node::SetNodeEvent;
use set_target_tps::SetTargetTpsEvent;
use step::StepEvent;

pub enum Event {
    AddNode(AddNodeEvent),
    RemoveNode(RemoveNodeEvent),
    SetNode(SetNodeEvent),
    Get(GetEvent),
    SetTargetTps(SetTargetTpsEvent),
    Step(StepEvent),
}
