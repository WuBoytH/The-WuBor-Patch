#![allow(unused_must_use)]

use smash::{
    lua2cpp::L2CFighterCommon,
    hash40,
    phx::Vector3f,
    app::{lua_bind::*/*, sv_animcmd::**/, *},
    lib::{lua_const::*, L2CValue}
};
use smash_script::*;
use smashline::*;
use crate::{
    // vars::*,
    table_const::*
};

#[common_status_script(status = FIGHTER_STATUS_KIND_DAMAGE_FALL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn common_status_damagefall(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_DamageFall_common();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f {x: 1.0, y: 0.0, z: 0.0});
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(common_status_damagefall_main as *const () as _))
}

unsafe extern "C" fn common_status_damagefall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() == true
    || fighter.check_damage_fall_transition().get_bool() == true {
        return L2CValue::I32(0);
    }
    let tech : bool;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND) == false {
        tech = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD);
    }
    else {
        // let mut flame_choke_tech_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("ganon_special_s_passive_trigger_frame")) as f32;
        // let tech_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("passive_trigger_frame_mul"), 0 as u64);
        // flame_choke_tech_frame *= tech_mul;
        tech = fighter.sub_check_passive_button(L2CValue::I32(0x30)).get_bool();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB) {
        if FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
            if WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("passive_fb_cont_value")) <= fighter.global_table[STICK_X].get_f32().abs() {
                if tech {
                    fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_FB.into(), true.into());
                    return L2CValue::Bool(true);
                }
            }
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE) {
        if FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
            if FighterStopModuleImpl::is_damage_stop(fighter.module_accessor) == false {
                if tech {
                    fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE.into(), true.into());
                    return L2CValue::Bool(true);
                }
            }
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_DOWN.into(), true.into());
            return L2CValue::I32(0);
        }
    }
    fighter.sub_damage_fall_uniq_process_exec_fix_pos();
    L2CValue::I32(0)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_DamageAir_Main)]
pub unsafe fn damage_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_transition_group_check_air_tread_jump)]
pub unsafe fn sub_transition_group_check_air_tread_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    let somebool = false;
    if fighter.global_table[0x30].get_bool() == false {
    }
    else {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[0x30].get_ptr());
        callable(fighter);
    }
    if somebool == false {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            if fighter.global_table[CMD_CAT1].get_i32() == *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP)
                && ControlModule::is_enable_flick_jump(fighter.module_accessor) {
                    let do_footstool;
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_NO_TREAD_FRAME) != 0 {
                        do_footstool = false;
                    }
                    else {
                        let tread_speed_y = fighter.FL_sub_fighter_float_next_tread_speed_y().get_f32();
                        let tread_jump_speed_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("tread_jump_speed_limit"));
                        if !(tread_jump_speed_limit <= tread_speed_y) {
                            do_footstool = false;
                        }
                        else {
                            fighter.clear_lua_stack();
                            lua_args!(fighter, 0x21bfbd3f83u64);
                            smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                            do_footstool = fighter.pop_lua_stack(1).get_bool();
                        }
                    }
                    if do_footstool {
                        fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), true.into());
                        return true.into();
                    }
                }
            }
            if fighter.global_table[PAD_FLAG].get_i32() == *FIGHTER_PAD_FLAG_JUMP_TRIGGER
            || [
                *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI,
                *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L,
                *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R,
                *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW
            ].contains(&fighter.global_table[CMD_CAT2].get_i32()) /* this is the addition */ {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP_BUTTON) {
                    let do_footstool;
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_NO_TREAD_FRAME) != 0 {
                        do_footstool = false;
                    }
                    else {
                        let tread_speed_y = fighter.FL_sub_fighter_float_next_tread_speed_y().get_f32();
                        let tread_jump_speed_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("tread_jump_speed_limit"));
                        if !(tread_jump_speed_limit <= tread_speed_y) {
                            do_footstool = false;
                        }
                        else {
                            fighter.clear_lua_stack();
                            lua_args!(fighter, 0x21bfbd3f83u64);
                            smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                            do_footstool = fighter.pop_lua_stack(1).get_bool();
                        }
                    }
                    if do_footstool {
                        fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), true.into());
                        return true.into();
                    }
                }
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP_NO_TRIGGER) {
                fighter.clear_lua_stack();
                lua_args!(fighter, 0x21bfbd3f83u64, true);
                smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                if fighter.pop_lua_stack(1).get_bool() {
                    fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), false.into());
                    return true.into();
                }
            }
        }
    }
    false.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            damage_air_main,
            sub_transition_group_check_air_tread_jump
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
    install_status_scripts!(
        common_status_damagefall
    );
}