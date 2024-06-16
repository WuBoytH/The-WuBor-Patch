use super::*;

extern "C" {
    #[link_name = "ryu_final_hit_cancel"]
    pub fn ryu_final_hit_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue;
}

unsafe extern "C" fn ken_attack_command1_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attr = if VarModule::is_flag(fighter.module_accessor, vars::ken::instance::flag::QUICK_STEP_INHERIT) {
        0
    }
    else {
        *FIGHTER_STATUS_ATTR_START_TURN
    };

    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
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
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON |
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_COMMAND1
        ) as u64,
        attr as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn ken_attack_command1_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    0.into()
}

unsafe extern "C" fn ken_attack_command1_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_int64(fighter.module_accessor, vars::ken::status::int64::SPECIAL_N2_GROUND_BRANCH_MOTION, hash40("invalid"));
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_n2_start"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        ken_attack_command1_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ken_attack_command1_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ken_attack_command1_main_loop as *const () as _))
}

unsafe extern "C" fn ken_attack_command1_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_BUTTON_ON_TIMER);
    }
    if VarModule::is_flag(fighter.module_accessor, vars::ken::status::flag::SPECIAL_N2_GROUND_BRANCH_CHECK) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            let stick_x = fighter.global_table[STICK_X].get_f32();
            let lr = PostureModule::lr(fighter.module_accessor);
            let stick_y = fighter.global_table[STICK_Y].get_f32();
            let squat_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"));
            let mot = if stick_y > squat_stick_y.abs() {
                hash40("special_n2_hi")
            }
            else if stick_y < squat_stick_y {
                hash40("special_n2_lw")
            }
            else if stick_x * lr > squat_stick_y.abs() {
                hash40("special_n2_s")
            }
            else {
                hash40("invalid")
            };
            if mot != hash40("invalid") {
                VarModule::set_int64(fighter.module_accessor, vars::ken::status::int64::SPECIAL_N2_GROUND_BRANCH_MOTION, mot);
                VarModule::off_flag(fighter.module_accessor, vars::ken::status::flag::SPECIAL_N2_GROUND_BRANCH_CHECK);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn ken_attack_command1_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT) {
        let sitation = fighter.global_table[SITUATION_KIND].get_i32();
        if ryu_final_hit_cancel(fighter, sitation.into()).get_bool() {
            return 1.into();
        }
    }

    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    if VarModule::is_flag(fighter.module_accessor, vars::ken::status::flag::SPECIAL_N2_GROUND_BRANCH_H) {
        let mot = VarModule::get_int64(fighter.module_accessor, vars::ken::status::int64::SPECIAL_N2_GROUND_BRANCH_MOTION);
        if mot == hash40("special_n2_s") {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            VarModule::off_flag(fighter.module_accessor, vars::ken::status::flag::SPECIAL_N2_GROUND_BRANCH_H);
            return 0.into();
        }
    }

    if VarModule::is_flag(fighter.module_accessor, vars::ken::status::flag::SPECIAL_N2_GROUND_BRANCH_LM) {
        let mot = VarModule::get_int64(fighter.module_accessor, vars::ken::status::int64::SPECIAL_N2_GROUND_BRANCH_MOTION);
        if mot == hash40("special_n2_hi")
        || mot == hash40("special_n2_lw") {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            VarModule::off_flag(fighter.module_accessor, vars::ken::status::flag::SPECIAL_N2_GROUND_BRANCH_LM);
            return 0.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 0.into();
    }

    0.into()
}

pub unsafe extern "C" fn ken_attack_command1_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(
        fighter.module_accessor,
        Hash40::new("ken_syoryuken_fire"),
        false,
        true
    );
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1, ken_attack_command1_pre);
    agent.status(Init, *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1, ken_attack_command1_init);
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1, ken_attack_command1_main);
    agent.status(End, *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1, ken_attack_command1_end);
}