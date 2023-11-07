mod start;
mod shoot;

pub fn install(agent: &mut smashline::Agent) {
    start::install(agent);
    shoot::install(agent);
}