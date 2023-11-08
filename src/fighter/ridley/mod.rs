mod acmd;
mod status;
mod frame;

// mod breath;

pub fn install() {
    let agent = &mut smashline::Agent::new("ridley");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent.install();

    // breath::install();
}