use super::*;

mod normals;
mod catch;

mod escape;
mod cliff;

mod appeal;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    catch::install(agent);

    escape::install(agent);
    cliff::install(agent);
    
    appeal::install(agent);
}
