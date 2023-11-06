mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("miigunner");
    acmd::install(agent);
    agent.install();
}