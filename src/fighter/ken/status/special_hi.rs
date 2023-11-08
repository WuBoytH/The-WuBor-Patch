use crate::imports::status_imports::*;

unsafe extern "C" fn ken_special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut mask = *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK;
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() != *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND {
        mask |= *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI;
    }
    else {
        mask |= *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI_COMMAND;
    }
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
        mask as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn ken_special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    let air = sit == *SITUATION_KIND_AIR;
    let step = VarModule::is_flag(fighter.module_accessor, ken::instance::flag::QUICK_STEP_INHERIT);
    let command = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND);
    let mot = if step {
        VarModule::on_flag(fighter.module_accessor, ken::status::flag::QUICK_STEP_INHERITED);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
        if air {
            if command {
                hash40("special_air_hi2_command")
            }
            else {
                hash40("special_air_hi2")
            }
        }
        else {
            if command {
                hash40("special_hi2_command")
            }
            else {
                hash40("special_hi2")
            }
        }
    }
    else {
        if air {
            if command {
                hash40("special_air_hi_command")
            }
            else {
                hash40("special_air_hi")
            }
        }
        else {
            if command {
                hash40("special_hi_command")
            }
            else {
                hash40("special_hi")
            }
        }
    };
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
    VarModule::off_flag(fighter.module_accessor, ken::instance::flag::QUICK_STEP_INHERIT);
    ItemModule::set_change_status_event(fighter.module_accessor, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        ken_special_hi_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ken_special_hi_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ken_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn ken_special_hi_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
            let stick_x = fighter.global_table[STICK_X].get_f32();
            let lr_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("lr_stick_x"));
            if lr_stick_x < stick_x.abs() {
                PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
                PostureModule::update_rot_y_lr(fighter.module_accessor);
            }
        }
    }
    else {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_BUTTON_ON_TIMER);
    }
    0.into()
}

unsafe extern "C" fn ken_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if !StatusModule::is_changing(fighter.module_accessor) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                sv_kinetic_energy!(
                    reset_energy,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_MOTION,
                    ENERGY_MOTION_RESET_TYPE_AIR_TRANS,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                );
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let start_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("start_accel_y"));
                sv_kinetic_energy!(
                    set_accel,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_MOTION,
                    -start_accel_y
                );
            }
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            if !StatusModule::is_changing(fighter.module_accessor) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                sv_kinetic_energy!(
                    reset_energy,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_MOTION,
                    ENERGY_MOTION_RESET_TYPE_GROUND_TRANS,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                );
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP.into(), false.into());
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, ken_special_hi_pre);
    agent.status(smashline::Pre, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND, ken_special_hi_pre);
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, ken_special_hi_main);
    agent.status(smashline::Main, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND, ken_special_hi_main);
}