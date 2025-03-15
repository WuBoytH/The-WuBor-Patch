use super::*;

mod dash;

mod guard;

mod normals;
mod smashes;
mod aerials;
mod specials;
mod throws;

mod catch;

mod escape;
mod cliff;

mod appeal;

pub fn install(agent: &mut Agent) {
    dash::install(agent);

    guard::install(agent);

    normals::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    specials::install(agent);
    throws::install(agent);

    catch::install(agent);

    escape::install(agent);
    cliff::install(agent);

    appeal::install(agent);
}