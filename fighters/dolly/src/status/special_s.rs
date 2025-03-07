use super::*;

unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_f_command_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND);

    special_s_main(fighter)
}

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::dolly::instance::flag::SPECIAL_CANCEL) {
        VarModule::on_flag(fighter.module_accessor, vars::dolly::status::flag::IS_SPECIAL_CANCEL);
        VarModule::off_flag(fighter.module_accessor, vars::dolly::instance::flag::SPECIAL_CANCEL);
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        VarModule::on_flag(fighter.module_accessor, vars::dolly::status::flag::SPECIAL_F_CHECK_FEINT);
        if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_GUARD_TRIGGER != 0 {
            VarModule::on_flag(fighter.module_accessor, vars::dolly::status::flag::SPECIAL_F_FEINT);
        }
    }

    WorkModule::set_int(fighter.module_accessor, *FIGHTER_DOLLY_STRENGTH_S, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH);

    WorkModule::set_int64(fighter.module_accessor, hash40("special_f_start") as i64, *FIGHTER_DOLLY_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_f_start") as i64, *FIGHTER_DOLLY_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR);

    let addition = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_02
    }
    else {
        *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01
    };

    notify_event_msc_cmd!(
        fighter,
        Hash40::new_raw(0x20cbc92683),
        1,
        FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND,
        addition - 1
    );

    WorkModule::set_customize_no(fighter.module_accessor, 1, 1);
    fighter.sub_set_special_start_common_kinetic_setting(Hash40::new("param_special_s").into());

    let func = smashline::api::get_target_function("lua2cpp_dolly.nrs", 0x1b620).unwrap();
    let func : fn(&mut L2CFighterCommon, L2CValue, L2CValue, L2CValue, L2CValue, L2CValue, L2CValue) = std::mem::transmute(func);
    func(fighter, true.into(), false.into(), true.into(), 0.into(), 0.into(), false.into());

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        let mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("command_power_mul"));
        AttackModule::set_power_mul_status(fighter.module_accessor, mul);
    }

    if !StopModule::is_stop(fighter.module_accessor) {
        special_s_substatus(fighter, false.into());
    }

    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_s_substatus as *const () as _));

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if VarModule::is_flag(fighter.module_accessor, vars::dolly::status::flag::SPECIAL_F_CHECK_FEINT)
        && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_GUARD_TRIGGER != 0 {
            VarModule::on_flag(fighter.module_accessor, vars::dolly::status::flag::SPECIAL_F_FEINT);
        }
    }
    0.into()
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::dolly::status::flag::SPECIAL_F_FEINT)
    && MotionModule::motion_kind(fighter.module_accessor) == hash40("special_f_start") {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            Hash40::new("special_f_feint"),
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }

    if VarModule::is_flag(fighter.module_accessor, vars::dolly::status::flag::SPECIAL_F_FEINT)
    && CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        let status = if VarModule::is_flag(fighter.module_accessor, vars::dolly::status::flag::SPECIAL_F_FEINT) {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK
        };
        fighter.change_status(status.into(), false.into());
        return 0.into();
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        let func = smashline::api::get_target_function("lua2cpp_dolly.nrs", 0x1b620).unwrap();
        let func : fn(&mut L2CFighterCommon, L2CValue, L2CValue, L2CValue, L2CValue, L2CValue, L2CValue) = std::mem::transmute(func);
        func(fighter, false.into(), true.into(), true.into(), 0.into(), 0.into(), false.into());
    }

    0.into()
}

unsafe extern "C" fn dolly_special_sb_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::dolly::instance::flag::SPECIAL_CANCEL) {
        VarModule::on_flag(fighter.module_accessor, vars::dolly::status::flag::IS_SPECIAL_CANCEL);
        VarModule::off_flag(fighter.module_accessor, vars::dolly::instance::flag::SPECIAL_CANCEL);
    }
    let status = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let original = original_status(Main, fighter, status);
    original(fighter)
}

// unsafe extern "C" fn dolly_special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
//     let status = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
//     if status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK
//     && status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK {
//         VarModule::off_flag(fighter.module_accessor, vars::dolly::status::flag::IS_SPECIAL_CANCEL);
//         WorkModule::set_customize_no(fighter.module_accessor, 0, 1);
//     }
//     0.into()
// }

unsafe extern "C" fn dolly_special_f_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.module_accessor, vars::dolly::status::flag::IS_SPECIAL_CANCEL);
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END {
        WorkModule::set_customize_no(fighter.module_accessor, 0, 1);
    }
    0.into()
}

unsafe extern "C" fn dolly_special_b_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_LANDING {
        VarModule::off_flag(fighter.module_accessor, vars::dolly::status::flag::IS_SPECIAL_CANCEL);
        WorkModule::set_customize_no(fighter.module_accessor, 0, 1);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
    agent.status(Pre, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND, special_s_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);
    agent.status(Main, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND, special_f_command_main);
    agent.status(Main, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, dolly_special_sb_main);
    agent.status(Main, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND, dolly_special_sb_main);
    // agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, dolly_special_s_end);
    // agent.status(End, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND, dolly_special_s_end);

    agent.status(End, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK, dolly_special_f_attack_end);

    // agent.status(End, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, dolly_special_s_end);
    // agent.status(End, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND, dolly_special_s_end);

    agent.status(End, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK, dolly_special_b_attack_end);
}