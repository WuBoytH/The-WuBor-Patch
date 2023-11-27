mod normals;
mod aerials;
mod specials;
mod catch;
mod escape;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    aerials::install(agent);
    specials::install(agent);
    catch::install(agent);
    escape::install(agent);
}