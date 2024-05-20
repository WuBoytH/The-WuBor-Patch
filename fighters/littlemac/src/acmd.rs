use super::*;

mod normals;
mod aerials;
mod appeal;
mod specials;
mod escape;
mod cliff;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    aerials::install(agent);
    appeal::install(agent);  
    specials::install(agent);
    escape::install(agent);
    cliff::install(agent);
}