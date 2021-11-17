use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    crate::table_const::*,
    super::super::common::common_status::fgc_dashback_main
};

#[status_script(agent = "ryu", status = FIGHTER_RYU_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ryu_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

#[status_script(agent = "ryu", status = FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ryu_specialsloop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let start_sit = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION);
    if start_sit != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_s"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_s"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    let eff : u64;
    if !MotionModule::is_flip(fighter.module_accessor) {
        eff = 0x1441b51880;
    }
    else {
        eff = 0x14bbba25e3;
    }
    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_EFFECT_REQUEST_FOLLOW, eff, hash40("rot"), 0.0, 1.5, 0.0, 0.0, 0.0, 0.0, 1.0, false, *EFFECT_SUB_ATTRIBUTE_SYNC_STOP, 0, -1);
    sv_module_access::effect(fighter.lua_state_agent);
    let spineffect = fighter.pop_lua_stack(1).get_u32();
    WorkModule::set_int(fighter.module_accessor, spineffect as i32, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_EFFECT_HANDLE);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        let alpha = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("wind_alpha")) * 0.01;
        EffectModule::set_alpha(fighter.module_accessor, spineffect, alpha);
    }
    else {
        let alpha = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("command_wind_alpha")) * 0.01;
        EffectModule::set_alpha(fighter.module_accessor, spineffect, alpha);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_specialsloop_main_loop as *const () as _))
}

unsafe extern "C" fn ryu_specialsloop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            }
            else {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
                    WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                    return 1.into();
                }
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            }
        }
    }
    else {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            let start_sit = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION);
            if start_sit != *SITUATION_KIND_GROUND {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                sv_kinetic_energy::set_accel(fighter.lua_state_agent);
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                sv_kinetic_energy::set_brake(fighter.lua_state_agent);
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT);
        let loop_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT);
        if loop_count > 0 {
            let start_sit = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION);
            if start_sit != *SITUATION_KIND_GROUND {
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new("special_air_s"),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
            }
            else {
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new("special_s"),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
            }
        }
        else {
            fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_FLAG_GROUND);
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_FLAG_GROUND);
    }
    0.into()
}

// #[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
// unsafe fn ryu_wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.status_pre_Wait()
// }

// #[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// unsafe fn ryu_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.status_Wait()
// }

// #[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_SQUAT_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// unsafe fn ryu_squatwait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.status_SquatWait()
// }

// #[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_WALK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// unsafe fn ryu_walk_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.status_Walk()
// }

// #[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_TURN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
// unsafe fn ryu_turn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.status_pre_Turn()
// }

// #[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_TURN_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
// unsafe fn ryu_turndash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.status_pre_TurnDash()
// }

// #[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
// unsafe fn ryu_landing_init(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.sub_landing_uniq_process_init()
// }

// #[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_LANDING_LIGHT, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
// unsafe fn ryu_landinglight_init(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.sub_landing_uniq_process_init()
// }

// #[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
// unsafe fn ryu_landingfallspecial_init(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.sub_landing_uniq_process_init()
// }

pub fn install() {
    install_status_scripts!(
        ryu_dashback_main,
        ryu_specialsloop_main,
        // ryu_wait_pre,
        // ryu_wait_main,
        // ryu_squatwait_main,
        // ryu_walk_main,
        // ryu_turn_pre,
        // ryu_turndash_pre,
        // ryu_landing_init,
        // ryu_landinglight_init,
        // ryu_landingfallspecial_init
    );
}