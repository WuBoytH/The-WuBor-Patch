mod acmd;
mod status;
// mod frame;

pub fn install() {
    let agent = &mut smashline::Agent::new("captain");
    acmd::install(agent);
    status::install(agent);
    // frame::install();
    agent.install();
}