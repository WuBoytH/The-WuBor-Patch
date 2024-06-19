use super::*;

mod dash;

mod normals;
mod aerials;
mod specials;

mod escape;
mod cliff;

mod appeal;

pub fn install(agent: &mut Agent) {
    dash::install(agent);

    normals::install(agent);
    aerials::install(agent);
    specials::install(agent);

    escape::install(agent);
    cliff::install(agent);

    appeal::install(agent);   
}