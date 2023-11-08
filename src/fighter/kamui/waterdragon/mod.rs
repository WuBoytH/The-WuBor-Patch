mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("kamui_waterdragon");
    acmd::install(agent);
    agent.install();
}