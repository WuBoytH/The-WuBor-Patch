use crate::imports::status_imports::*;

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn dolly_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_speciallw_pre_main(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn dolly_speciallw_command_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_speciallw_pre_main(fighter)
}

unsafe extern "C" fn dolly_speciallw_pre_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_cancel = VarModule::is_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW);
    }
    let attr = if VarModule::is_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL)
    && fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_DASH {
        0
    }
    else {
        *FIGHTER_STATUS_ATTR_START_TURN
    };
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
        *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        attr as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    VarModule::set_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL, is_cancel);
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_speciallw_main_inner(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_speciallw_command_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND);
    dolly_speciallw_main_inner(fighter)
}

unsafe extern "C" fn dolly_speciallw_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    WorkModule::set_int(fighter.module_accessor, situation, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_INT_START_SITUATION);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_DOLLY_STRENGTH_S, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH);
    WorkModule::set_customize_no(fighter.module_accessor, 1, 3);
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_lw").into());
    dolly_speciallw_mot_helper(fighter, true.into());
    let additions = if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_07
    }
    else {
        let command_power_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("command_power_mul"));
        AttackModule::set_power_mul_status(fighter.module_accessor, command_power_mul);
        *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_08
    };
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, additions - 1);
    if !StopModule::is_stop(fighter.module_accessor) {
        dolly_speciallw_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(dolly_speciallw_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_speciallw_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_speciallw_mot_helper(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    let mot;
    let reset;
    let correct;
    if situation == *SITUATION_KIND_GROUND {
        mot = Hash40::new("special_lw_start");
        reset = ENERGY_MOTION_RESET_TYPE_GROUND_TRANS;
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP;
    }
    else {
        mot = Hash40::new("special_air_lw_start");
        reset = ENERGY_MOTION_RESET_TYPE_AIR_TRANS;
        correct = *GROUND_CORRECT_KIND_AIR;
    }
    if param_1.get_bool() {
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
    }
    else {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            mot,
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        reset,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
}

unsafe extern "C" fn dolly_speciallw_substatus(fighter: &mut L2CFighterCommon, _param_1: L2CValue) -> L2CValue {
    let start_sit = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_INT_START_SITUATION);
    if start_sit == *SITUATION_KIND_GROUND
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        VarModule::on_flag(fighter.battle_object, dolly::status::flag::SPECIAL_LW_ENABLE_BREAK);
    }
    0.into()
}

unsafe extern "C" fn dolly_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if !MotionModule::is_end(fighter.module_accessor) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_JUMP)
        && StatusModule::is_changing(fighter.module_accessor) {
            dolly_speciallw_mot_helper(fighter, false.into());
            fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_lw").into());
        }
    }
    if !fighter.global_table[IS_STOP].get_bool()
    && MotionModule::is_end(fighter.module_accessor)
    || VarModule::is_flag(fighter.battle_object, dolly::status::flag::SPECIAL_LW_CHECK_BREAK) {
        if VarModule::is_flag(fighter.battle_object, dolly::status::flag::SPECIAL_LW_BREAK) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 0.into()
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_JUMP) {
            let frame = MotionModule::frame(fighter.module_accessor);
            WorkModule::set_int(fighter.module_accessor, frame as i32, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_INT_START_FRAME);
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK.into(), false.into());
            return 0.into();
        }
        if !VarModule::is_flag(fighter.battle_object, dolly::status::flag::SPECIAL_LW_ENABLE_BREAK) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_JUMP);
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_DOLLY_SPECIAL_LW_JUMP);
            let strength = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH);
            let mot = if strength == *FIGHTER_DOLLY_STRENGTH_W {
                Hash40::new("special_lw_w")
            }
            else {
                Hash40::new("special_lw")
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
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            VarModule::on_flag(fighter.battle_object, dolly::status::flag::SPECIAL_LW_BREAK);
        }
        VarModule::off_flag(fighter.battle_object, dolly::status::flag::SPECIAL_LW_CHECK_BREAK);
    }
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_speciallw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_speciallw_end_main(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_speciallw_command_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_speciallw_end_main(fighter)
}

unsafe extern "C" fn dolly_speciallw_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK {
        VarModule::off_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
    }
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_speciallw_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_attack"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    let log = if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_07
    }
    else {
        let command_power_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("command_power_mul"));
        AttackModule::set_power_mul_status(fighter.module_accessor, command_power_mul);
        *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_08
    };
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), log - 1);
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_speciallw_attack_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_speciallw_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_LANDING_HEAVY) {
            let param = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
                hash40("landing_heavy_frame_command")
            }
            else {
                hash40("landing_heavy_frame")
            };
            let landing_heavy_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), param);
            WorkModule::set_int(fighter.module_accessor, landing_heavy_frame, *FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_LANDING_STIFFNESS_FRAME);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            return 0.into();
        }
        let frame = fighter.global_table[STATUS_FRAME].get_f32() as i32;
        let start_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_INT_START_FRAME);
        let attack_no_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("attack_no_landing_frame"));
        if frame + start_frame >= attack_no_landing_frame {
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_LANDING.into(), false.into());
            return 0.into();
        }
    }
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_speciallw_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let param = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_HIT) {
        hash40("landing_frame_hit")
    }
    else {
        hash40("landing_frame_fail")
    };
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), param);
    WorkModule::set_float(fighter.module_accessor, landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    VarModule::off_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        dolly_speciallw_pre,
        dolly_speciallw_command_pre,
        dolly_speciallw_main,
        dolly_speciallw_command_main,
        dolly_speciallw_end,
        dolly_speciallw_command_end,
        dolly_speciallw_attack_main, dolly_speciallw_attack_end
    );
}