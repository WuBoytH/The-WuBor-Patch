use super::*;

pub mod movement;
mod guard;
pub mod escape;
mod damage;
pub mod attack;
mod catch;

mod fall;

mod passive;
pub mod cliff;
mod appeal;
mod rebirth;
mod sub;
mod sub_transitions;
mod sub_fighter;

use {
    escape::escape_air_slide::*,
    cliff::cliff_jump1::*,
    guard::guard_cancel::{
        attack::*,
        escape::*,
        appeal::*
    },
};

pub fn install() {
    movement::install();
    guard::install();
    escape::install();
    damage::install();
    attack::install();
    catch::install();

    fall::install();

    passive::install();
    cliff::install();
    appeal::install();
    rebirth::install();
    sub::install();
    sub_transitions::install();
    sub_fighter::install();

    let agent = &mut Agent::new("fighter");

    agent.status(Pre, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, escape_air_slide_pre);
    agent.status(Init, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, escape_air_slide_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, escape_air_slide_main);
    agent.status(End, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, escape_air_slide_end);
    agent.status(CalcParam, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, escape_air_slide_calc_param);

    agent.status(Exec, *FIGHTER_STATUS_KIND_CLIFF_JUMP1, sub_cliff_jump1_uniq_process_exec);

    agent.status(Pre, vars::fighter::status::GUARD_CANCEL_ATTACK, guard_cancel_attack_pre);
    agent.status(Main, vars::fighter::status::GUARD_CANCEL_ATTACK, guard_cancel_attack_main);

    agent.status(Pre, vars::fighter::status::GUARD_CANCEL_ESCAPE_F, guard_cancel_escape_f_pre);

    agent.status(Pre, vars::fighter::status::GUARD_CANCEL_ESCAPE_B, guard_cancel_escape_b_pre);

    agent.status(Pre, vars::fighter::status::GUARD_CANCEL_PASS, guard_cancel_pass_pre);
    agent.status(Init, vars::fighter::status::GUARD_CANCEL_PASS, guard_cancel_pass_init);
    agent.status(Main, vars::fighter::status::GUARD_CANCEL_PASS, guard_cancel_pass_main);
    agent.status(Exec, vars::fighter::status::GUARD_CANCEL_PASS, guard_cancel_pass_exec);
    agent.status(End, vars::fighter::status::GUARD_CANCEL_PASS, guard_cancel_pass_end);

    agent.status(Pre, vars::fighter::status::GUARD_CANCEL_APPEAL, guard_cancel_appeal_pre);

    agent.install();
}