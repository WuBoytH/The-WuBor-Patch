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

    if VarModule::get_int(fighter.module_accessor, vars::dolly::status::int::D_TILT_CHAIN_COUNT) > 0
    && !CancelModule::is_enable_cancel(fighter.module_accessor) {
        let stick_dir = ControlModule::get_stick_dir(fighter.module_accessor);
        let attack_s3_stick_dir_hi = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_s3_stick_dir_hi"));
        if (fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 == 0
        && FGCModule::cancel_exceptions(
            fighter,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3,
            true
        ).get_bool())
        || (fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 == 0
        && attack_s3_stick_dir_hi < stick_dir
        && FGCModule::cancel_exceptions(
            fighter,
            *FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3,
            true
        ).get_bool()) {
            VarModule::on_flag(fighter.module_accessor, vars::dolly::status::flag::IS_SPECIAL_CANCEL);
            return 1.into();
        }
    }
    if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 == 0
    && FGCModule::chain_cancels(fighter,
        *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3,
        true,
        vars::dolly::status::int::D_TILT_CHAIN_COUNT,
        1
    ).get_bool() {
        let count = VarModule::get_int(fighter.module_accessor, vars::dolly::status::int::D_TILT_CHAIN_COUNT);
        let mot = match count {
            1 => Hash40::new("attack_lw32"),
            _ => Hash40::new("attack_lw3")
        };
        MotionModule::change_motion(
            fighter.module_accessor,
            mot,
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        return 0.into();
    }
    fighter.status_AttackLw3_Main();
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW3, dolly_attack_lw3_main);
}