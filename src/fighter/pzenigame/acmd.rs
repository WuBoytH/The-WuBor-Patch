mod specials;
mod escape;

pub fn install(agent : &mut smashline::Agent) {
    specials::install(agent);
    escape::install(agent);
}
