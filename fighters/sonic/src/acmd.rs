use super::*;

mod specials;
mod catch;
mod throws;
mod appeal;
mod escape;
mod cliff;

pub fn install(agent: &mut smashline::Agent) {
    specials::install(agent);
    catch::install(agent);
    throws::install(agent);
    appeal::install(agent);
    escape::install(agent);
    cliff::install(agent);
}