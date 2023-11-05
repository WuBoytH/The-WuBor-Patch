mod acmd;

mod clay;

pub fn install() {
    let agent = &mut smashline::Agent::new("duckhunt");
    acmd::install(agent);
    agent.install();

    clay::install();
}