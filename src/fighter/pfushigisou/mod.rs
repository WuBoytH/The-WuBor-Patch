mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("pfushigisou");
    acmd::install(agent);
    agent.install();
}