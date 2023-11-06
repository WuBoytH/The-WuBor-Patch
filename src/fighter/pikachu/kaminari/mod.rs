mod acmd;
mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("pikachu_kaminari");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}