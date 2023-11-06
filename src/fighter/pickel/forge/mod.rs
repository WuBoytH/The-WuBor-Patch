mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("pickel_forge");
    acmd::install(agent);
    agent.install();
}