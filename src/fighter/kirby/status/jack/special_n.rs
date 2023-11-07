use crate::imports::status_imports::*;

pub unsafe extern "C" fn kirby_jack_special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let keep_first = if prev_status != *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE {
        false
    }
    else {
        VarModule::is_flag(fighter.module_accessor, jack::status::flag::SPECIAL_N_FIRST)
    };
    let original = smashline::original_status(smashline::Main, fighter, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N);
    let ret = original(fighter);
    VarModule::set_flag(fighter.module_accessor, jack::status::flag::SPECIAL_N_FIRST, keep_first);
    ret
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N, kirby_jack_special_n_pre);
}