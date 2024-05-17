mod acmd;
mod status;
mod frame;

mod clay;

pub fn install() {
    let agent = &mut smashline::Agent::new("duckhunt");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent.install();

    clay::install();
}