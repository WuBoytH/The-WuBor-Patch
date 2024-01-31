use crate::imports::status_imports::*;

unsafe extern "C" fn ryu_special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_special_s_init_inner(fighter)
}

unsafe extern "C" fn ryu_special_s_command_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND);
    ryu_special_s_init_inner(fighter)
}

unsafe extern "C" fn ryu_special_s_init_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    WorkModule::set_int(fighter.module_accessor, sit, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION);
    let command = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_RYU_STRENGTH_S, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_x_mul = if sit == *SITUATION_KIND_AIR {
        1.0
    }
    else {
        WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("speed_x_mul"))
    };
    let speed_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("speed_y_mul"));
    let mut speed_x = sum_speed_x * speed_x_mul;
    let mut speed_y = sum_speed_y * speed_y_mul;
    let add_speed_x = if sit == *SITUATION_KIND_AIR {
        0.0
    }
    else {
        WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("add_speed_x"))
    };
    let lr = PostureModule::lr(fighter.module_accessor);
    speed_x += add_speed_x * lr;
    let stop_type = if sit != *SITUATION_KIND_GROUND {
        let add_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("add_speed_y"));
        speed_y += add_speed_y;
        ENERGY_STOP_RESET_TYPE_AIR
    }
    else {
        ENERGY_STOP_RESET_TYPE_NONE
    };
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        stop_type,
        speed_x,
        0.0,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_brake,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.0,
        0.0
    );
    if sit == *SITUATION_KIND_GROUND {
        let ground_speed_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("ground_speed_limit"));
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ground_speed_limit,
            0.0
        );
    }
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    if sit != *SITUATION_KIND_GROUND {
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0,
            speed_y, 
            0.0,
            0.0,
            0.0
        );
        // let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_accel_y"));
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -0.05
        );
        let air_max_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_max_speed_y"));
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            air_max_speed_y
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    if command {
        let command_power_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("command_power_mul"));
        AttackModule::set_power_mul_status(fighter.module_accessor, command_power_mul);
    }
    let boma = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, boma);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, boma);
    0.into()
}

pub unsafe extern "C" fn ryu_special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ryu_special_s_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn ryu_special_s_substatus(fighter: &mut L2CFighterCommon, param_2: L2CValue) -> L2CValue {
    if param_2.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_BUTTON_ON_TIMER);
    }
    0.into()
}

unsafe extern "C" fn ryu_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        ryu_special_s_mot_helper(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn ryu_special_s_mot_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if StatusModule::is_changing(fighter.module_accessor) {
            let mot = if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION) == *SITUATION_KIND_GROUND {
                hash40("special_air_s_start")
            }
            else {
                hash40("special_air_s2_start")
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
        }
        if StatusModule::is_changing(fighter.module_accessor) {
            return;
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_accel_y"));
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y
        );
        let air_max_speed_y = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
            1.0
        }
        else {
            WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_max_speed_y"))
        };
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            air_max_speed_y
        );
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if StatusModule::is_changing(fighter.module_accessor) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_s_start"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
        if StatusModule::is_changing(fighter.module_accessor) {
            return;
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            0.0,
            0.0
        );
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Init, *FIGHTER_STATUS_KIND_SPECIAL_S, ryu_special_s_init);
    agent.status(smashline::Init, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, ryu_special_s_command_init);
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_S, ryu_special_s_main);
    agent.status(smashline::Main, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, ryu_special_s_main);
}