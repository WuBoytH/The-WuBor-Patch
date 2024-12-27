use super::*;

mod dash;

mod guard;

mod normals;
mod specials;

mod catch;
mod throws;

mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    dash::install(agent);

    guard::install(agent);

    normals::install(agent);
    specials::install(agent);

    catch::install(agent);
    throws::install(agent);

    escape::install(agent);
    cliff::install(agent);
}
