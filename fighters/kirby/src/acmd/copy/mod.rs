use super::*;

mod ganon;
mod ike;
mod wolf;
mod ryu;
mod belmont;
mod jack;

pub fn install(agent: &mut Agent) {
    ganon::install(agent);
    ike::install(agent);
    wolf::install(agent);
    ryu::install(agent);
    belmont::install(agent);
    jack::install(agent);
}