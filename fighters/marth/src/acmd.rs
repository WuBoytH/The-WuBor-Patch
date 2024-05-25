use super::*;

mod normals;
mod smashes;
mod specials;
mod stance;
mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    normals::install(agent);
    smashes::install(agent);
    specials::install(agent);
    stance::install(agent);
    escape::install(agent);
    cliff::install(agent);
}