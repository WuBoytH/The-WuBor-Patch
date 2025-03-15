use super::*;

mod look_up;

mod dash;
mod jump;

mod guard;

mod normals;
mod aerials;

mod specials;
mod trick;
mod catch;
mod throws;

mod escape;
mod cliff;

mod appeal;

pub fn install(agent: &mut Agent) {
    look_up::install(agent);

    dash::install(agent);
    jump::install(agent);

    guard::install(agent);

    normals::install(agent);
    aerials::install(agent);

    specials::install(agent);
    trick::install(agent);
    catch::install(agent);
    throws::install(agent);

    escape::install(agent);
    cliff::install(agent);

    appeal::install(agent);
}