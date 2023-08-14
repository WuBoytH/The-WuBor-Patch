use crate::imports::status_imports::*;

#[status("demon", FIGHTER_STATUS_KIND_ATTACK_LW3)]
unsafe fn demon_attacklw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackLw3_common();
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_DEMON_STATUS_ATTACK_LW_3_WORK_INT_CANCEL_STATUS);
    fighter.sub_shift_status_main(L2CValue::Ptr(demon_attacklw3_main_loop as *const () as _))
}

unsafe extern "C" fn demon_attacklw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackLw3_Main();
    if !StatusModule::is_changing(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
            let mut status = -1;
            let cat4 = fighter.global_table[CMD_CAT4].get_i32();
            if fighter.sub_check_command_guard().get_bool() {
                status = *FIGHTER_STATUS_KIND_GUARD_ON;
            }
            else if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_1 != 0 {
                status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_3;
            }
            else if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_2 != 0 {
                status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_2;
            }
            else if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_3 != 0 {
                status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_1;
            }
            if status != -1 {
                WorkModule::set_int(fighter.module_accessor, status, *FIGHTER_DEMON_STATUS_ATTACK_LW_3_WORK_INT_CANCEL_STATUS);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_LW_3_FLAG_INC_STEP);
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_LW_3_FLAG_CHECK_STEP)
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_LW_3_FLAG_INC_STEP) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_LW_3_FLAG_INC_STEP);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_LW_3_FLAG_CHECK_STEP);
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
                fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_LW3_CANCEL.into(), false.into());
            }
        }
    }
    0.into()
}

#[status("demon", FIGHTER_DEMON_STATUS_KIND_ATTACK_LW3_CANCEL)]
unsafe fn demon_attacklw3_cancel_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("attack_lw3_cancel"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(demon_attacklw3_cancel_main_loop as *const () as _))
}

unsafe extern "C" fn demon_attacklw3_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor)
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_LW_3_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_GUARD_ON {
        let mut status = -1;
        let cat4 = fighter.global_table[CMD_CAT4].get_i32();
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_1 != 0 {
            status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_3;
        }
        else if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_2 != 0 {
            status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_2;
        }
        else if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_3 != 0 {
            status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_1;
        }
        if status != -1 {
            WorkModule::set_int(fighter.module_accessor, status, *FIGHTER_DEMON_STATUS_ATTACK_LW_3_WORK_INT_CANCEL_STATUS);
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        let status = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_LW_3_WORK_INT_CANCEL_STATUS);
        if ![
            *FIGHTER_STATUS_KIND_WAIT,
            *FIGHTER_STATUS_KIND_GUARD_ON,
            -1
        ].contains(&status) {
            let mut clear_buffer = true;
            if status == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
                clear_buffer = false;
            }
            fighter.change_status(status.into(), clear_buffer.into());
            return 0.into();
        }
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    0.into()
}

pub fn install() {
    demon_attacklw3_main::install();
    demon_attacklw3_cancel_main::install();
}