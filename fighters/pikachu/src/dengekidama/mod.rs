use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("pikachu_dengekidama");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}