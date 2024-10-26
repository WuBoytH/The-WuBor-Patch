use super::*;
use super::super::helper::*;

unsafe extern "C" fn dolly_attack_lw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    fighter.status_AttackLw3_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_attack_lw3_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_attack_lw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if dolly_hit_cancel(fighter).get_i32() != 0
    || dolly_attack_start_cancel(fighter).get_i32() != 0 {
        return 1.into();
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 {
        fighter.change_status(vars::dolly::status::ATTACK_LW32.into(), true.into());
        return 1.into();
    }
    fighter.status_AttackLw3_Main();
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW3, dolly_attack_lw3_main);
}