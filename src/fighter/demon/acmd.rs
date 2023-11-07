mod jabs;
mod normals;
mod smashes;
mod throws;
mod aerials;
mod specials;
mod stand;
mod step;
mod squat;
mod escape;
mod appeal;

pub fn install(agent: &mut smashline::Agent) {
    jabs::install(agent);
    normals::install(agent);
    smashes::install(agent);
    throws::install(agent);
    aerials::install(agent);
    specials::install(agent);
    stand::install(agent);
    step::install(agent);
    squat::install(agent);
    escape::install(agent);
    appeal::install(agent);
}