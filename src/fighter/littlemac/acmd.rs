mod aerials;
mod specials;
mod escape;
mod cliff;

pub fn install(agent: &mut smashline::Agent) {
    aerials::install(agent);
    specials::install(agent);
    escape::install(agent);
    cliff::install(agent);
}