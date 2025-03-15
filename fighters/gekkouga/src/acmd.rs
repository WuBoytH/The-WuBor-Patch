use super::*;

mod dash;

mod guard;

mod normals;
mod smashes;
mod aerials;
mod throws;

mod catch;

mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    dash::install(agent);

    guard::install(agent);

    normals::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    throws::install(agent);

    catch::install(agent);

    escape::install(agent);
    cliff::install(agent);
}