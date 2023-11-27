mod catch;
mod escape;
mod cliff;

pub fn install(agent: &mut smashline::Agent) {
    catch::install(agent);
    escape::install(agent);
    cliff::install(agent);
}
