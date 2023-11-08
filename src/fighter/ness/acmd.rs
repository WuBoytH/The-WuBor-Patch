mod catch;
mod escape;

pub fn install(agent: &mut smashline::Agent) {
    catch::install(agent);
    escape::install(agent);
}
