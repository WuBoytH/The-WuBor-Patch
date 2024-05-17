use super::*;

mod ganon;
mod ike;
mod belmont;
mod jack;

pub fn install(agent: &mut smashline::Agent) {
    ganon::install(agent);
    ike::install(agent);
    belmont::install(agent);
    jack::install(agent);
}