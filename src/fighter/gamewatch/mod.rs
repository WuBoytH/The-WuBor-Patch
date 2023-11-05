mod acmd;
mod status;

mod breath;

pub fn install() {
    let agent = &mut smashline::Agent::new("gamewatch");
    acmd::install(agent);
    status::install(agent);
    agent.install();

    breath::install();
}