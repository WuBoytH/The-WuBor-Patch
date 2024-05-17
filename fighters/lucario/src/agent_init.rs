use super::*;

pub unsafe extern "C" fn lucario_status_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR
    || status == *FIGHTER_STATUS_KIND_REBIRTH {
        VarModule::off_flag(fighter.module_accessor, vars::lucario::instance::flag::USED_AURA_CHARGE_AIR);
        VarModule::off_flag(fighter.module_accessor, vars::lucario::instance::flag::EXTREME_SPEED_FORCE_NO_AURA);
        VarModule::off_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_S);
        if ![
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR
        ].contains(&status) {
            VarModule::off_flag(fighter.module_accessor, vars::lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL);
        }
    }
    if status == *FIGHTER_STATUS_KIND_REBIRTH {
        VarModule::set_int(fighter.module_accessor, vars::lucario::instance::int::AURA_LEVEL, 0);
    }
    0.into()
}

pub unsafe extern "C" fn lucario_special_s_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    (!VarModule::is_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_S)).into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(lucario_special_s_uniq as *const () as _));
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(lucario_status_end_control as *const () as _));
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_start(on_start);
}