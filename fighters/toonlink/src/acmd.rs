use super::*;

mod normals;
mod aerials;
mod throws;
mod specials;
mod lasso;
mod escape;
mod cliff;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    aerials::install(agent);
    throws::install(agent);
    specials::install(agent);
    lasso::install(agent);
    escape::install(agent);
    cliff::install(agent);
}
