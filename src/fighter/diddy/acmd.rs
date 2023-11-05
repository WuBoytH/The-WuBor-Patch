mod normals;
mod smash_attacks;
mod specials;
mod escape;

pub fn install(agent : &mut smashline::Agent) {
    normals::install(agent);
    smash_attacks::install(agent);
    specials::install(agent);
    escape::install(agent);
}