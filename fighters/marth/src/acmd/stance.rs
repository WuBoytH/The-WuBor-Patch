mod movement;
mod normals;
mod specials;

pub fn install(agent: &mut smashline::Agent) {
    movement::install(agent);
    normals::install(agent);
    specials::install(agent);
}