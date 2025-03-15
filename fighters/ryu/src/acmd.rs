use super::*;

mod dash;

mod guard;

mod normals;
mod smashes;
mod aerials;
mod specials;

mod catch;

mod r#final;

mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    dash::install(agent);

    guard::install(agent);

    normals::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    specials::install(agent);

    catch::install(agent);

    r#final::install(agent);

    escape::install(agent);
    cliff::install(agent);
}