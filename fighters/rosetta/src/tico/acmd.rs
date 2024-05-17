use super::*;

mod normals;
mod specials;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    specials::install(agent);
}