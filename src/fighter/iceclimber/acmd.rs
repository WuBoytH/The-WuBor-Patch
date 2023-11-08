mod escape;

pub fn install(agent: &mut smashline::Agent) {
    escape::install(agent);
}
