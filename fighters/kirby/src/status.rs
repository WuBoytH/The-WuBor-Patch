use super::*;

mod kirby;
mod purin;
mod koopa;
mod ganon;
mod lucario;
mod ike;
mod belmont;
mod jack;

pub fn install(agent: &mut smashline::Agent) {
    kirby::install(agent);
    purin::install(agent);
    koopa::install(agent);
    ganon::install(agent);
    lucario::install(agent);
    ike::install(agent);
    belmont::install(agent);
    jack::install(agent);
}