use super::*;

mod normals;
mod aerials;
mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    normals::install(agent);
    aerials::install(agent);
    escape::install(agent);
    cliff::install(agent);
}