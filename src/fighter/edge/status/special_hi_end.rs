use crate::imports::status_imports::*;
use super::helper::*;

unsafe extern "C" fn edge_special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rush_degree = WorkModule::get_float(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_RUSH_DEGREE);
    WorkModule::set_float(fighter.module_accessor, rush_degree, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_DECIDE_ROT_DEGREE);
    let charged_rush = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_CHARGED_RUSH);
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed = sv_kinetic_energy::get_speed(fighter.lua_state_agent);
    let vec2 = fighter.Vector2__create(speed.x.into(), speed.y.into());
    let rush_end_speed_mul = edge_special_hi_param_float_helper(fighter, hash40("rush_end_speed_mul").into(), charged_rush.into()).get_f32();
    let rush_end_brake_x = edge_special_hi_param_float_helper(fighter, hash40("rush_end_brake_x").into(), charged_rush.into()).get_f32();
    let speed_x = vec2["x"].get_f32() * rush_end_speed_mul;
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        speed_x,
        0.0
    );
    sv_kinetic_energy!(
        set_brake,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        rush_end_brake_x.abs(),
        0.0
    );
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
    let rush_end_gravity_accel = edge_special_hi_param_float_helper(fighter, hash40("rush_end_gravity_accel").into(), charged_rush.into()).get_f32();
    let speed_y = speed_y * rush_end_speed_mul;
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        speed_y
    );
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -rush_end_gravity_accel
    );
    let mot = if !charged_rush {
        Hash40::new("special_air_hi1_end")
    }
    else {
        Hash40::new("special_air_hi2_end")
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
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_special_hi_end_main_loop as *const () as _))
}

unsafe extern "C" fn edge_special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    // <WuBor>
    if edge_special_hi_cancel(fighter).get_bool() {
        return 1.into();
    }
    // </WuBor>
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if !MotionModule::is_end(fighter.module_accessor) {
            let charged_rush = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_CHARGED_RUSH);
            let rotate_back_begin_frame = edge_special_hi_param_int_helper(fighter, hash40("rotate_back_begin_frame").into(), charged_rush.into()).get_i32();
            let rotate_back_end_frame = edge_special_hi_param_int_helper(fighter, hash40("rotate_back_end_frame").into(), charged_rush.into()).get_i32();
            let frame = fighter.global_table[STATUS_FRAME].get_f32();
            if rotate_back_begin_frame as f32 <= frame {
                let mut diff = rotate_back_end_frame - rotate_back_begin_frame;
                if diff <= 0 {
                    diff = 1;
                }
                let ratio = 1.0 - ((frame - rotate_back_begin_frame as f32) / diff as f32);
                let clamp = ratio.clamp(0.0, 1.0);
                let degree = WorkModule::get_float(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_DECIDE_ROT_DEGREE);
                let rot_step = clamp * degree;
                WorkModule::set_float(fighter.module_accessor, rot_step, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_RUSH_DEGREE);
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL) {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
                let control_accel_x_mul = edge_special_hi_param_float_helper(fighter, hash40("control_accel_x_mul").into(), charged_rush.into()).get_f32();
                let control_speed_x_max_mul = edge_special_hi_param_float_helper(fighter, hash40("control_speed_x_max_mul").into(), charged_rush.into()).get_f32();
                sv_kinetic_energy!(
                    reset_energy,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                    ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
                    0.0, 0.0, 0.0, 0.0, 0.0
                );
                sv_kinetic_energy!(
                    mul_x_speed_max,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                    control_speed_x_max_mul
                );
                sv_kinetic_energy!(
                    mul_x_accel_mul,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                    control_accel_x_mul
                );
                sv_kinetic_energy!(
                    mul_x_accel_add,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                    control_accel_x_mul
                );
                sv_kinetic_energy!(
                    enable,
                    fighter,
                    *FIGHTER_KINETIC_ENERGY_ID_CONTROL
                );
            }
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END, edge_special_hi_end_main);
}