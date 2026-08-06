use super::*;

mod captain;
mod dolly;
mod ganon;
mod ike;
mod jack;
mod koopa;
mod lucario;
mod purin;
mod richter;
mod ryu;
mod simon;
mod wolf;

pub fn install(agent: &mut Agent) {
    captain::install(agent);
    dolly::install(agent);
    ganon::install(agent);
    ike::install(agent);
    jack::install(agent);
    koopa::install(agent);
    lucario::install(agent);
    purin::install(agent);
    richter::install(agent);
    ryu::install(agent);
    simon::install(agent);
    wolf::install(agent);
}
