mod aerials;
mod escape;
mod cliff;

pub fn install(agent: &mut smashline::Agent) {
    aerials::install(agent);
    escape::install(agent);
    cliff::install(agent);
}
