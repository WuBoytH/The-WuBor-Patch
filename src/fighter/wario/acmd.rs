mod throws;
mod specials;
mod escape;
mod cliff;

pub fn install(agent: &mut smashline::Agent) {
    throws::install(agent);
    specials::install(agent);
    escape::install(agent);
    cliff::install(agent);
}
