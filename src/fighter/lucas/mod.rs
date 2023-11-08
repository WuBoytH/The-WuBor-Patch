mod acmd;
mod status;
mod frame;

mod pkfire;

pub fn install() {
    let agent = &mut smashline::Agent::new("lucas");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent.install();

    pkfire::install();
}