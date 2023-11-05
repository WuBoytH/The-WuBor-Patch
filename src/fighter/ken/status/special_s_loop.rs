use crate::imports::status_imports::*;
use crate::fighter::ryu::helper::*;

#[status_script(agent = "ken", status = FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe extern "C" fn ken_specials_loop_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specials_loop_init_main(fighter)
}

#[status_script(agent = "ken", status = FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn ken_specials_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    let eff = if !MotionModule::is_flip(fighter.module_accessor) {
        Hash40::new("ken_tatsumaki_wind_r")
    }
    else {
        Hash40::new("ken_tatsumaki_wind_l")
    };
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
    fighter.sub_shift_status_main(L2CValue::Ptr(ken_specials_loop_main_loop as *const () as _))
}

unsafe extern "C" fn ken_specials_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
                sv_kinetic_energy!(
                    set_accel,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_STOP,
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

pub fn install() {
    install_status_scripts!(
        ken_specials_loop_init,
        ken_specials_loop_main
    );
}