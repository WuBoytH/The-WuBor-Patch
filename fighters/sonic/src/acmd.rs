use super::*;

mod normals;
mod aerials;
mod specials;
mod trick;
mod catch;
mod throws;
mod escape;
mod cliff;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    aerials::install(agent);
    specials::install(agent);
    trick::install(agent);
    catch::install(agent);
    throws::install(agent);
    escape::install(agent);
    cliff::install(agent);
}