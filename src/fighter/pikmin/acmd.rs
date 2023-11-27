mod normals;
mod catch;
mod escape;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    catch::install(agent);
    escape::install(agent);
}