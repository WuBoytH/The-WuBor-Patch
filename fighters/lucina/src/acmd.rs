use super::*;

mod dash;
mod run;

mod guard;

mod normals;
mod smashes;
mod throws;
mod aerials;
mod specials;

mod catch;

mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    dash::install(agent);
    run::install(agent);

    guard::install(agent);

    normals::install(agent);
    smashes::install(agent);
    throws::install(agent);
    aerials::install(agent);
    specials::install(agent);

    catch::install(agent);

    escape::install(agent);
    cliff::install(agent);
}