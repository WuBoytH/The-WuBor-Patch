use super::*;

unsafe extern "C" fn wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let was_guard_cancel_attack = fighter.global_table[PREV_STATUS_KIND].get_i32() == vars::fighter::status::GUARD_CANCEL_ATTACK;
    if was_guard_cancel_attack {
        fighter.global_table[PREV_STATUS_KIND].assign(&L2CValue::I32(*FIGHTER_STATUS_KIND_ATTACK_100));
    }
    let ret = original_status(Main, fighter, *FIGHTER_STATUS_KIND_WAIT)(fighter);
    if was_guard_cancel_attack {
        fighter.global_table[PREV_STATUS_KIND].assign(&L2CValue::I32(vars::fighter::status::GUARD_CANCEL_ATTACK));
    }
    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_WAIT, wait_main);
}