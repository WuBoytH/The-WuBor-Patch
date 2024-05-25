use super::*;

mod normals;
mod aerials;
mod specials;
mod catch;
mod throw;
mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    normals::install(agent);
    aerials::install(agent);
    specials::install(agent);
    catch::install(agent);
    throw::install(agent);
    escape::install(agent);
    cliff::install(agent);
}
