mod smashes;
mod specials;
mod escape;

pub fn install(agent : &mut smashline::Agent) {
    smashes::install(agent);
    specials::install(agent);
    escape::install(agent);
}