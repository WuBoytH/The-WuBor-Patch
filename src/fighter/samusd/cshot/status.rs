mod shoot;

pub fn install(agent: &mut smashline::Agent) {
    shoot::install(agent);
}