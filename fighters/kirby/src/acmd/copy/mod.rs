use super::*;

mod ganon;
mod ike;
mod jack;
mod richter;
mod ryu;
mod simon;
mod wolf;

pub fn install(agent: &mut Agent) {
    ganon::install(agent);
    ike::install(agent);
    jack::install(agent);
    richter::install(agent);
    ryu::install(agent);
    simon::install(agent);
    wolf::install(agent);
}
