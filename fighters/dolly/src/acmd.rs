use super::*;

mod dash;

mod guard;

mod normals;
mod smashes;
mod catch;
mod throws;
mod aerials;
mod specials;
mod superspecials;

mod escape;
mod cliff;

mod appeal;

pub fn install(agent: &mut Agent) {
    dash::install(agent);

    guard::install(agent);

    normals::install(agent);
    smashes::install(agent);
    catch::install(agent);
    throws::install(agent);
    aerials::install(agent);
    specials::install(agent);
    superspecials::install(agent);

    escape::install(agent);
    cliff::install(agent);

    appeal::install(agent);
}