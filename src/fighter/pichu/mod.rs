mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("pichu");
    acmd::install(agent);
    agent.install();
}