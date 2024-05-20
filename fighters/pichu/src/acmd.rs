use super::*;

mod normals;
mod catch;
mod appeal;
mod escape;
mod cliff;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    catch::install(agent);
    appeal::install(agent);
    escape::install(agent);
    cliff::install(agent);
}
