use super::*;

mod dash;

mod normals;
mod smashes;
mod aerials;
mod specials;

mod r#final;

mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    dash::install(agent);

    normals::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    specials::install(agent);

    r#final::install(agent);

    escape::install(agent);
    cliff::install(agent);
}