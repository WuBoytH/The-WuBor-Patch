use super::*;

mod wait;

mod movement;

mod attacks;

mod specials;

pub fn install(agent: &mut Agent) {
    wait::install(agent);

    movement::install(agent);

    attacks::install(agent);

    specials::install(agent);
}