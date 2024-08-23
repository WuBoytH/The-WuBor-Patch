use super::*;

mod dash;

mod normals;
mod aerials;
mod specials;

mod catch;
mod throws;

mod escape;
mod cliff;

mod appeal;

pub fn install(agent: &mut Agent) {
    dash::install(agent);

    normals::install(agent);
    aerials::install(agent);
    specials::install(agent);

    catch::install(agent);
    throws::install(agent);

    escape::install(agent);
    cliff::install(agent);

    appeal::install(agent);
}