mod acmd;
mod status;

mod sword;

pub fn install() {
    let agent = &mut smashline::Agent::new("master");
    acmd::install(agent);
    status::install(agent);
    agent.install();

    sword::install();
}