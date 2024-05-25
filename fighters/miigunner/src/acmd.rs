use super::*;

mod normals;
mod specials;
mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    normals::install(agent);
    specials::install(agent);
    escape::install(agent);
    cliff::install(agent);
}
