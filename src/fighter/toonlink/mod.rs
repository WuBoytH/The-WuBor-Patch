mod acmd;
mod frame;
mod status;
pub mod vl;

mod bowarrow;
mod boomerang;

pub fn install() {
    let agent = &mut smashline::Agent::new("toonlink");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    agent.install();

    bowarrow::install();
    boomerang::install();
}