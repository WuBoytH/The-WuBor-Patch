mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("pikachu_kaminari");
    acmd::install(agent);
    agent.install();
}