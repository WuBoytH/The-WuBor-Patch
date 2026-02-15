use super::*;

mod status;

pub fn install() {
    let agent = &mut Agent::new("pickel_table");
    status::install(agent);
    agent.install();
}