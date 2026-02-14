use super::*;

mod belmont;
mod captain;
mod dolly;
mod ganon;
mod ike;
mod jack;
mod koopa;
mod lucario;
mod purin;
mod ryu;
mod wolf;

pub fn install(agent: &mut Agent) {
    belmont::install(agent);
    captain::install(agent);
    dolly::install(agent);
    ganon::install(agent);
    ike::install(agent);
    jack::install(agent);
    koopa::install(agent);
    lucario::install(agent);
    purin::install(agent);
    ryu::install(agent);
    wolf::install(agent);
}
