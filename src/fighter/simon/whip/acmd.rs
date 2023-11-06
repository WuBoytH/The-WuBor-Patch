mod normals;
mod aerials;
mod smashes;

pub fn install(agent : &mut smashline::Agent) {
    normals::install(agent);
    aerials::install(agent);
    smashes::install(agent);
}