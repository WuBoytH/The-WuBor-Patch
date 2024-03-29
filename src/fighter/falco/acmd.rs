mod smashes;
mod specials;
mod escape;
mod cliff;
mod appeal;

pub fn install(agent: &mut smashline::Agent) {
    smashes::install(agent);
    specials::install(agent);
    escape::install(agent);
    cliff::install(agent);
    appeal::install(agent);
}