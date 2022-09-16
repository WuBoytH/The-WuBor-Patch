use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    wubor_utils::table_const::*
};

#[skyline::hook(replace = L2CFighterCommon_is_enable_passive)]
pub unsafe fn is_enable_passive(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_PACKMAN_EYE) {
        return false.into();
    }
    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("speed_vec_x"));
    sv_information::damage_log_value(fighter.lua_state_agent);
    let speed_vec_x = fighter.pop_lua_stack(1).get_f32();
    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("speed_vec_y"));
    sv_information::damage_log_value(fighter.lua_state_agent);
    let speed_vec_y = fighter.pop_lua_stack(1).get_f32();
    let length = sv_math::vec2_length(speed_vec_x, speed_vec_y);
    let invalid_passive_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("invalid_passive_speed"));
    if invalid_passive_speed <= length {
        return false.into();
    }
    true.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_check_passive_button)]
pub unsafe fn sub_check_passive_button(fighter: &mut L2CFighterCommon, _param_1: L2CValue) -> L2CValue {
    // The basis of the new tech system, teching is now performed by having your stick tilted in
    // any direction that *isn't* down, so neutral and down cause missed techs.
    // This is also why that param_1 argument goes unused, it doesn't matter to check it anymore.

    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let passive_fb_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("passive_fb_cont_value"));
    let fb = passive_fb_cont_value <= stick_x.abs();
    let jump_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_stick_y"));
    let jump = jump_stick_y <= stick_y;
    let guard_button = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD);
    let no_rapid_frame_value = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("no_rapid_frame_value"));
    let passive_input = no_rapid_frame_value <= ControlModule::get_trigger_count_prev(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD as u8) & 0xff;
    (fb || jump || guard_button || passive_input).into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_check_passive_button_for_damage)]
pub unsafe fn sub_check_passive_button_for_damage(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    // Now just calls sub_check_passive_button

    fighter.sub_check_passive_button(param_1)
}

#[skyline::hook(replace = L2CFighterCommon_sub_AirChkPassive)]
pub unsafe fn sub_airchkpassive(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_enable_passive().get_bool() {
        return false.into();
    }
    let tech = fighter.sub_check_passive_button(L2CValue::Void()).get_bool();
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
    && {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let passive_fb_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("passive_fb_cont_value"));
        passive_fb_cont_value <= stick_x.abs()
    }
    && tech {
        fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_FB.into(), true.into());
        return true.into();
    }
    
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
    && tech {
        fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE.into(), true.into());
        return true.into();
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_AirChkPassive_for_damage)]
pub unsafe fn sub_airchkpassive_for_damage(fighter: &mut L2CFighterCommon) -> L2CValue {
    // if !fighter.is_enable_passive().get_bool() {
    //     return false.into();
    // }
    let tech = fighter.sub_check_passive_button(L2CValue::Void()).get_bool();
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
    && {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let passive_fb_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("passive_fb_cont_value"));
        passive_fb_cont_value <= stick_x.abs()
    }
    && !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor)
    && tech {
        fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_FB.into(), true.into());
        return true.into();
    }
    
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
    && !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor)
    && tech {
        fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE.into(), true.into());
        return true.into();
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_AirChkPassiveWall)]
pub unsafe fn sub_airchkpassivewall(fighter: &mut L2CFighterCommon) -> L2CValue {
    // if !fighter.is_enable_passive().get_bool() {
    //     return false.into();
    // }
    let tech = fighter.sub_check_passive_button(L2CValue::Void()).get_bool();
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && FighterUtil::is_touch_passive_ground(fighter.module_accessor, (*GROUND_TOUCH_FLAG_RIGHT | *GROUND_TOUCH_FLAG_LEFT) as u32)
    && !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor)
    && tech {
        fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_WALL.into(), true.into());
        return true.into();
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_AirChkPassiveWallJump)]
pub unsafe fn sub_airchkpassivewalljump(fighter: &mut L2CFighterCommon) -> L2CValue {
    // if !fighter.is_enable_passive().get_bool() {
    //     return false.into();
    // }
    let tech = fighter.sub_check_passive_button(L2CValue::Void()).get_bool();
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON)
    && FighterUtil::is_touch_passive_ground(fighter.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32)
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
    && !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && tech {
        fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP.into(), true.into());
        return true.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP)
    && FighterUtil::is_touch_passive_ground(fighter.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32)
    && {
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let jump_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_stick_y"));
        jump_stick_y <= stick_y
    }
    && !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && tech {
        fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP.into(), true.into());
        return true.into();
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_AirChkPassiveCeil)]
pub unsafe fn sub_airchkpassiveceil(fighter: &mut L2CFighterCommon) -> L2CValue {
    // if !fighter.is_enable_passive().get_bool() {
    //     return false.into();
    // }
    let tech = fighter.sub_check_passive_button(L2CValue::Void()).get_bool();
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32)
    && !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor)
    && tech {
        fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_CEIL.into(), true.into());
        return true.into();
    }
    false.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            is_enable_passive,
            sub_check_passive_button,
            sub_check_passive_button_for_damage,
            sub_airchkpassive,
            sub_airchkpassive_for_damage,
            sub_airchkpassivewall,
            sub_airchkpassivewalljump,
            sub_airchkpassiveceil
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}