mod acmd;
mod status;
mod frame;

pub fn install() {
    let agent = &mut smashline::Agent::new("krool");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent.install();
}