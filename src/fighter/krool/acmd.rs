mod smashes;
// mod aerials;
mod escape;

pub fn install(agent: &mut smashline::Agent) {
    smashes::install(agent);
    // aerials::install(agent);
    escape::install(agent);
}