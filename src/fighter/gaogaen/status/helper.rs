use crate::imports::status_imports::*;

pub unsafe fn gaogaen_special_lw_kinetic_helper(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let status = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let special_lw_start_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_start_x_mul"));
    let special_air_lw_speed_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_air_lw_speed_y_mul"));
    let special_air_lw_hit_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_air_lw_hit_accel_y"));
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    let kinetic = if situation == *SITUATION_KIND_GROUND {
        *FIGHTER_KINETIC_TYPE_GROUND_STOP
    }
    else {
        *FIGHTER_KINETIC_TYPE_FALL
    };
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    if status == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if situation != *SITUATION_KIND_GROUND {
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0
            );
            if !param_1.get_bool() {
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    speed_y
                );
            }
            else {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_INVALID_SPECIAL_AIR_LW_SPPED_Y) {
                    sv_kinetic_energy!(
                        set_speed,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                        speed_y * special_air_lw_speed_y_mul
                    );
                    sv_kinetic_energy!(
                        set_accel,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                        -0.01
                    );
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_INVALID_SPECIAL_AIR_LW_SPPED_Y);
                }
            }
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        else {
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fighter.module_accessor);
        }
        if param_1.get_bool() {
            speed_x *= special_lw_start_x_mul;
        }
        if situation != *SITUATION_KIND_GROUND {
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                ENERGY_STOP_RESET_TYPE_AIR,
                speed_x,
                0.0,
                0.0,
                0.0,
                0.0
            );
        }
        else {
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                ENERGY_STOP_RESET_TYPE_GROUND,
                speed_x,
                0.0,
                0.0,
                0.0,
                0.0
            );
        }
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        if situation != *SITUATION_KIND_GROUND {
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
        }
        else {
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                ENERGY_MOTION_RESET_TYPE_GROUND_TRANS,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0
            );
        }
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    }
    else {
        if situation != *SITUATION_KIND_GROUND {
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0
            );
            if !param_1.get_bool() {
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    speed_y
                );
            }
            else {
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    0.0
                );
            }
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -special_air_lw_hit_accel_y
            );
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        else {
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fighter.module_accessor);
        }
        if situation != *SITUATION_KIND_GROUND {
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                ENERGY_STOP_RESET_TYPE_AIR,
                speed_x,
                0.0,
                0.0,
                0.0,
                0.0
            );
        }
        else {
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                ENERGY_STOP_RESET_TYPE_GROUND,
                speed_x,
                0.0,
                0.0,
                0.0,
                0.0
            );
        }
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        if situation != *SITUATION_KIND_GROUND {
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
        }
        else {
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                ENERGY_MOTION_RESET_TYPE_GROUND_TRANS,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0
            );
        }
        if !param_1.get_bool()
        && status == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_LW_TURN {
            sv_kinetic_energy!(
                set_speed_mul,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_MOTION,
                -1.0
            );
        }
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    }
}