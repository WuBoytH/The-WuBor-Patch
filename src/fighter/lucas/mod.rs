mod acmd;
mod status;

mod pkfire;

pub fn install() {
    let agent = &mut smashline::Agent::new("lucas");
    acmd::install(agent);
    status::install(agent);
    agent.install();

    pkfire::install();
}