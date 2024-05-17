use super::*;

mod normals;
mod aerials;

mod escape;
mod cliff;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    aerials::install(agent);

    escape::install(agent);
    cliff::install(agent);
}