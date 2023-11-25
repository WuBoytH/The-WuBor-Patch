mod specials;
mod throws;
mod escape;
mod cliff;

pub fn install(agent: &mut smashline::Agent) {
    specials::install(agent);
    throws::install(agent);
    escape::install(agent);
    cliff::install(agent);
}