use super::*;

pub unsafe extern "C" fn guard_cancel_escape_f_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    add_shield_health(fighter, -0.4);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);

    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_F);
    1.into()
}

pub unsafe extern "C" fn guard_cancel_escape_b_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    add_shield_health(fighter, -0.4);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);

    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_B);
    1.into()
}

pub unsafe extern "C" fn guard_cancel_pass_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Pass()
}

pub unsafe extern "C" fn guard_cancel_pass_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_uniq_process_Pass_init()
}

pub unsafe extern "C" fn guard_cancel_pass_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    add_shield_health(fighter, -0.4);

    VarModule::set_int(fighter.module_accessor, vars::fighter::instance::int::GUARD_CANCEL_PASS_FRAME, 20);

    HitModule::set_xlu_frame_global(fighter.module_accessor, 20, 0);

    fighter.status_Pass_common();

    fighter.sub_shift_status_main(L2CValue::Ptr(guard_cancel_pass_main_loop as *const () as _))
}

pub unsafe extern "C" fn guard_cancel_pass_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_PASS_WORK_INT_FRAME) == 0 {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_PASS_FLAG_IS_SET_PASS) {
            if fighter.end_pass_ground().get_bool() {
                return 0.into();
            }
        }

        if fighter.sub_transition_group_check_air_landing().get_bool() {
            return 0.into();
        }

        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }

        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

pub unsafe extern "C" fn guard_cancel_pass_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_uniq_process_Pass_exec_status()
}

pub unsafe extern "C" fn guard_cancel_pass_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();

    if status != *FIGHTER_STATUS_KIND_FALL {
        HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);

        VarModule::set_int(fighter.module_accessor, vars::fighter::instance::int::GUARD_CANCEL_PASS_FRAME, 0);
    }

    0.into()
}