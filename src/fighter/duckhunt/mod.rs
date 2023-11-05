mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("duckhunt");
    acmd::install(agent);
}