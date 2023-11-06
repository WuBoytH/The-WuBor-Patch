mod throws;
mod specials;
mod escape;

pub fn install(agent : &mut smashline::Agent) {
    throws::install(agent);
    specials::install(agent);
    escape::install(agent);
}
