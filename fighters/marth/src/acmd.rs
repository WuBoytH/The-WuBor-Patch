use super::*;

mod dash;

mod guard;

mod normals;
mod smashes;
mod specials;
mod stance;

mod catch;

mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    dash::install(agent);

    guard::install(agent);

    normals::install(agent);
    smashes::install(agent);
    specials::install(agent);
    stance::install(agent);

    catch::install(agent);

    escape::install(agent);
    cliff::install(agent);
}