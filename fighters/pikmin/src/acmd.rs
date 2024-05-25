use super::*;

mod normals;
mod catch;
mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    normals::install(agent);
    catch::install(agent);
    escape::install(agent);
    cliff::install(agent);
}