mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("samus");
    acmd::install(agent);
    agent.install();
}