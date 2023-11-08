mod guard_on;
mod guard;
mod guard_damage;
pub mod helper;

pub fn install(agent: &mut smashline::Agent) {
    guard_on::install(agent);
    guard::install(agent);
    guard_damage::install(agent);
}