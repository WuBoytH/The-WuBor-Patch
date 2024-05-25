use super::*;

mod normals;
mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    normals::install(agent);
    escape::install(agent);
    cliff::install(agent);
}
