use crate::imports::status_imports::*;

unsafe extern "C" fn rockman_check_special_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, rockman::instance::flag::CHARGE_SHOT_RELEASE)
    && VarModule::is_flag(fighter.module_accessor, rockman::instance::flag::CHARGE_SHOT_PLAYED_FX) {
        fighter.global_table[CMD_CAT1].assign(&L2CValue::I32(*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N));
    }
    false.into()
}

unsafe extern "C" fn rockman_special_lw_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    (!WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LEAFSHIELD)).into()
}

unsafe extern "C" fn rockman_check_air_escape_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LEAFSHIELD)
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
        VarModule::on_flag(fighter.module_accessor, fighter::status::flag::FORCE_ESCAPE_AIR_SLIDE_IN_STATUS);
    }
    false.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_GROUND_SPECIAL_UNIQ].assign(&L2CValue::Ptr(rockman_check_special_uniq as *const () as _));
    fighter.global_table[CHECK_AIR_SPECIAL_UNIQ].assign(&L2CValue::Ptr(rockman_check_special_uniq as *const () as _));
    fighter.global_table[DASH_COMMON_UNIQ].assign(&false.into());
    fighter.global_table[RUN_MAIN_UNIQ].assign(&false.into());
    fighter.global_table[JUMP_SQUAT_MAIN_UNIQ].assign(&false.into());
    fighter.global_table[CHECK_AIR_ESCAPE_UNIQ].assign(&L2CValue::Ptr(rockman_check_air_escape_uniq as *const () as _));
    fighter.global_table[GUARD_CONT_UNIQ].assign(&false.into());
    fighter.global_table[TURN_UNIQ].assign(&false.into());
    fighter.global_table[FALL_BRAKE_UNIQ].assign(&false.into());
    fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(rockman_special_lw_uniq as *const () as _));
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_start(on_start);
}