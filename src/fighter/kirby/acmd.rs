mod normals;
mod aerials;
mod specials;
mod copy;
mod escape;
mod misc;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    aerials::install(agent);
    specials::install(agent);
    copy::install(agent);
    escape::install(agent);
    misc::install(agent);
}