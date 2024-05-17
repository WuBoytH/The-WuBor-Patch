use super::*;

mod aerials;
mod normals;
mod specials;
mod escape;
mod cliff;

pub fn install(agent: &mut smashline::Agent) {
    aerials::install(agent);
    normals::install(agent);
    specials::install(agent);
    escape::install(agent);
    cliff::install(agent);
}