use super::*;

mod final1;
mod final1_jump;
mod final1_hit;
mod final1_fall;
mod final1_landing;
mod final1_air_end;

pub fn install(agent: &mut Agent) {
    final1::install(agent);
    final1_jump::install(agent);
    final1_hit::install(agent);
    final1_fall::install(agent);
    final1_landing::install(agent);
    final1_air_end::install(agent);
}