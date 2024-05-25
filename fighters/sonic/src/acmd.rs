use super::*;

mod specials;
mod catch;
mod throws;

mod escape;
mod cliff;

mod appeal;

pub fn install(agent: &mut Agent) {
    specials::install(agent);
    catch::install(agent);
    throws::install(agent);

    escape::install(agent);
    cliff::install(agent);

    appeal::install(agent);
}