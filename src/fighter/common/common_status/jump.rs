#![allow(unused_must_use)]

use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CAgent, L2CValue}
    },
    smash_script::*,
    wubor_utils::{
        vars::*,
        table_const::*
    },
    super::super::common_param
};

#[skyline::hook(replace = L2CFighterCommon_status_Jump_sub)]
unsafe fn status_jump_sub(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SUPER_JUMP) {
        let super_jump_x_mul;
        let jump_y;
        let mini_jump = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        if mini_jump {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 1, -2, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            super_jump_x_mul = common_param::jump::hyper_hop_speed_x_mul;
            jump_y = 0.0;
        }
        else {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 1, -5, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            super_jump_x_mul = common_param::jump::super_jump_speed_x_mul;
            let jump_initial_y = WorkModule::get_param_float(fighter.module_accessor, hash40("jump_initial_y"), 0) / 10.0;
            let ratio = 2.0 - (jump_initial_y / 2.8);
            jump_y = common_param::jump::super_jump_speed_y_init + (jump_initial_y * ratio);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
        let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        speed_x *= super_jump_x_mul;
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        let air_speed_x_stable = air_speed_x_stable * common_param::jump::hyper_hop_air_speed_x_stable_mul;
        if mini_jump
        && speed_x.abs() < air_speed_x_stable {
            if speed_x == 0.0 {
                speed_x = air_speed_x_stable * PostureModule::lr(fighter.module_accessor);
            }
            else if speed_x < 0.0 {
                speed_x = -air_speed_x_stable;
            }
            else {
                speed_x = air_speed_x_stable;
            }
        }
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_x
        );
        sv_kinetic_energy!(
            mul_x_accel_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            common_param::jump::special_jump_control_mul
        );
        sv_kinetic_energy!(
            mul_x_accel_add,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            common_param::jump::special_jump_control_mul
        );
        if mini_jump {
            sv_kinetic_energy!(
                set_stable_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                speed_x.abs()
            );
        }
        let jump_speed_x_max = WorkModule::get_param_float(fighter.module_accessor, hash40("jump_speed_x_max"), 0);
        if speed_x.abs() > jump_speed_x_max {
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                speed_x.abs()
            );
        }
        if jump_y != 0.0 {
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                jump_y
            );
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -common_param::jump::super_jump_gravity
            );
        }
        WorkModule::set_float(fighter.module_accessor, 10.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SUPER_JUMP_FRAME);
    }
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    fighter.global_table[FLICK_Y].assign(&0xFE.into());
    ControlModule::reset_trigger(fighter.module_accessor);
    fighter.sub_air_check_fall_common_pre();
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_STOP_CEIL);
    let mot;
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_POWBLOCK_QUAKE_JUMP) {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let lr = PostureModule::lr(fighter.module_accessor);
        let jump_neutral_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_x"));
        let mini_jump = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        let stick_x = stick_x * lr * -1.0;
        if !(jump_neutral_x <= stick_x) {
            if !mini_jump {
                mot = Hash40::new("jump_f");
            }
            else {
                mot = Hash40::new("jump_f_mini");
            }
        }
        else {
            if !mini_jump {
                mot = Hash40::new("jump_b");
            }
            else {
                mot = Hash40::new("jump_b_mini");
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP) {
            SoundModule::play_se(
                fighter.module_accessor,
                Hash40::new("se_item_usagihat_jump_01"),
                true,
                false,
                false,
                false,
                enSEType(0)
            );
        }
    }
    else {
        mot = Hash40::new("jump_f_mini");
    }
    let ret : L2CValue;
    if param_1.get_u64() != hash40("invalid") {
        ret = param_1.clone();
    }
    else {
        ret = 0.into();
    }
    MotionModule::change_motion(
        fighter.module_accessor,
        mot,
        0.0,
        1.0,
        false,
        param_2.get_f32(),
        false,
        false
    );
    if fighter.global_table[FALL_PRE].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[FALL_PRE].get_ptr());
        callable(fighter);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_fall_common_uniq as *const () as _));
    ret
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_Jump)]
unsafe fn bind_address_call_status_end_jump(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SUPER_JUMP) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI) {
            let status = fighter.global_table[STATUS_KIND].get_i32();
            if status != *FIGHTER_STATUS_KIND_ATTACK_AIR {
                if status == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                    WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SUPER_JUMP);
                    WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SUPER_JUMP_FRAME);
                }
                else {
                    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
                    sv_kinetic_energy!(
                        set_speed,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                        speed_y * 0.69
                    );
                    WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SUPER_JUMP);
                    WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SUPER_JUMP_FRAME);
                }
            }
        }
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SUPER_JUMP);
        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SUPER_JUMP_FRAME);
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_Jump)]
unsafe fn status_end_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SUPER_JUMP) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI) {
            let status = fighter.global_table[STATUS_KIND].get_i32();
            if status != *FIGHTER_STATUS_KIND_ATTACK_AIR {
                if status == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                    WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SUPER_JUMP);
                    WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SUPER_JUMP_FRAME);
                }
                else {
                    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
                    sv_kinetic_energy!(
                        set_speed,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                        speed_y * 0.69
                    );
                    WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SUPER_JUMP);
                    WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SUPER_JUMP_FRAME);
                }
            }
        }
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SUPER_JUMP);
        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SUPER_JUMP_FRAME);
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_jump_sub,
            bind_address_call_status_end_jump,
            status_end_jump
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}