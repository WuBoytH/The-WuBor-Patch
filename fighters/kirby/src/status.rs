use super::*;

mod kirby;

mod captain;
mod purin;
mod koopa;
mod ganon;
mod lucario;
mod ike;
mod wolf;
mod ryu;
mod belmont;
mod jack;
mod dolly;

pub fn install(agent: &mut Agent) {
    kirby::install(agent);

    captain::install(agent);
    purin::install(agent);
    koopa::install(agent);
    ganon::install(agent);
    lucario::install(agent);
    ike::install(agent);
    wolf::install(agent);
    ryu::install(agent);
    belmont::install(agent);
    jack::install(agent);
    dolly::install(agent);
}