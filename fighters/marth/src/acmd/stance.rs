use super::*;

mod movement;
mod normals;
mod specials;

pub fn install(agent: &mut Agent) {
    movement::install(agent);
    normals::install(agent);
    specials::install(agent);
}