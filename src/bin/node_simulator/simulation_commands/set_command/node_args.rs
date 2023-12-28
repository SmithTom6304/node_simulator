#[derive(clap::Args, Debug)]
pub struct NodeArgs {
    #[arg(long)]
    id: u32,
    #[arg(long)]
    position: Option<String>,
    #[arg(long)]
    velocity: Option<String>,
    #[arg(long)]
    mass: Option<f32>,
    #[arg(long)]
    gravitational_constant_override: Option<f32>,
    #[arg(long)]
    dampen_rate: Option<f32>,
    #[arg(long)]
    freeze: Option<bool>,
}
