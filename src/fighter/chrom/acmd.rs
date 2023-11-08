mod movement;
mod normals;
mod smash_attacks;
mod aerials;
mod specials;
mod escape;

pub fn install(agent: &mut smashline::Agent) {
    movement::install(agent);
    normals::install(agent);
    smash_attacks::install(agent);
    aerials::install(agent);
    specials::install(agent);
    escape::install(agent);
}