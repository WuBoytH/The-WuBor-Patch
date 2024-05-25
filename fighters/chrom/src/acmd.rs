use super::*;

mod movement;
mod normals;
mod smashes;
mod aerials;
mod specials;
mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    movement::install(agent);
    normals::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    specials::install(agent);
    escape::install(agent);
    cliff::install(agent);
}