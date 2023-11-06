mod acmd;
mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("ness");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}