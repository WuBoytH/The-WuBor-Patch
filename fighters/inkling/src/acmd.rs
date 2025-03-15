use super::*;

mod dash;

mod guard;

mod normals;
mod aerials;
mod specials;

mod catch;

mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    dash::install(agent);

    guard::install(agent);

    normals::install(agent);
    aerials::install(agent);

    specials::install(agent);

    catch::install(agent);

    escape::install(agent);
    cliff::install(agent);
}
