mod specials;
mod catch;
mod throws;
mod escape;
mod cliff;

pub fn install(agent: &mut smashline::Agent) {
    specials::install(agent);
    catch::install(agent);
    throws::install(agent);
    escape::install(agent);
    cliff::install(agent);
}