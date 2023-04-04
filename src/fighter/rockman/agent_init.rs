use crate::imports::status_imports::*;

unsafe extern "C" fn rockman_check_special_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_SPECIAL_RELEASE != 0
    && VarModule::is_flag(fighter.battle_object, rockman::instance::flag::CHARGE_SHOT_PLAYED_FX) {
        let cat1 = fighter.global_table[CMD_CAT1].get_i32();
        fighter.global_table[CMD_CAT1].assign(&L2CValue::I32(cat1 | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N));
    }
    false.into()
}

unsafe extern "C" fn rockman_special_lw_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    (!WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LEAFSHIELD)).into()
}

#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_ROCKMAN {
            return;
        }
        fighter.global_table[CHECK_GROUND_SPECIAL_UNIQ].assign(&L2CValue::Ptr(rockman_check_special_uniq as *const () as _));
        fighter.global_table[CHECK_AIR_SPECIAL_UNIQ].assign(&L2CValue::Ptr(rockman_check_special_uniq as *const () as _));
        fighter.global_table[DASH_COMMON_UNIQ].assign(&false.into());
        fighter.global_table[RUN_MAIN_UNIQ].assign(&false.into());
        fighter.global_table[JUMP_SQUAT_MAIN_UNIQ].assign(&false.into());
        fighter.global_table[GUARD_CONT_UNIQ].assign(&false.into());
        fighter.global_table[TURN_UNIQ].assign(&false.into());
        fighter.global_table[FALL_BRAKE_UNIQ].assign(&false.into());
        fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(rockman_special_lw_uniq as *const () as _));
    }
}

pub fn install() {
    install_agent_resets!(
        agent_reset
    );
}
