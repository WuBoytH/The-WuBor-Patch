mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("rosetta_tico");
    acmd::install(agent);
    agent.install();
}