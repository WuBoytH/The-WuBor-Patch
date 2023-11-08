mod aerials;
mod escape;

pub fn install(agent: &mut smashline::Agent) {
    aerials::install(agent);
    escape::install(agent);
}
