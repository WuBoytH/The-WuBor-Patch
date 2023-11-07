mod normals;
mod smashes;
mod specials;
mod stance;
mod escape;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    smashes::install(agent);
    specials::install(agent);
    stance::install(agent);
    escape::install(agent);
}