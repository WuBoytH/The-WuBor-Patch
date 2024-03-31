use crate::imports::*;
use super::super::param;

#[skyline::hook(replace = L2CFighterCommon_is_enable_passive)]
unsafe extern "C" fn is_enable_passive(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_PACKMAN_EYE) {
        return false.into();
    }
    true.into()
}

pub unsafe extern "C" fn is_bad_passive(fighter: &mut L2CFighterCommon) -> L2CValue {
    // let weight = WorkModule::get_param_float(fighter.module_accessor, hash40("weight"), 0);
    // let damage = DamageModule::damage(fighter.module_accessor, 0);
    // (weight + param::passive::invalid_passive_damage_add <= damage).into()
    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("reaction_frame"));
    sv_information::damage_log_value(fighter.lua_state_agent);
    let reaction_frame = fighter.pop_lua_stack(1).get_f32();
    (reaction_frame >= param::passive::invalid_passive_reaction).into()
}

// #[skyline::hook(replace = L2CFighterCommon_sub_check_passive_button)]
// unsafe extern "C" fn sub_check_passive_button(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
//     let passive_input = ControlModule::get_trigger_count(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD as u8) & 0xFF < param_1.get_i32();
//     passive_input.into()
// }

#[skyline::hook(replace = L2CFighterCommon_sub_check_passive_button_for_damage)]
unsafe extern "C" fn sub_check_passive_button_for_damage(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    let reaction_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
    if reaction_frame <= 0.0
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        return true.into();
    }
    fighter.sub_check_passive_button(param_1)
}

#[skyline::hook(replace = L2CFighterCommon_sub_AirChkPassive)]
unsafe extern "C" fn sub_airchkpassive(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_enable_passive().get_bool() {
        return false.into();
    }
    let passive_trigger_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("passive_trigger_frame")) as f32;
    let passive_trigger_frame_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("passive_trigger_frame_mul"), 0);
    let tech = fighter.sub_check_passive_button_for_damage((passive_trigger_frame * passive_trigger_frame_mul).into()).get_bool();
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
unsafe extern "C" fn sub_airchkpassive_for_damage(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_enable_passive().get_bool() {
        return false.into();
    }
    let tech = check_tech(fighter);
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
unsafe extern "C" fn sub_airchkpassivewall(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_enable_passive().get_bool() {
        return false.into();
    }
    let tech = check_tech(fighter);
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
unsafe extern "C" fn sub_airchkpassivewalljump(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_enable_passive().get_bool() {
        return false.into();
    }
    let tech = check_tech(fighter);
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
unsafe extern "C" fn sub_airchkpassiveceil(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_enable_passive().get_bool() {
        return false.into();
    }
    let tech = check_tech(fighter);
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

unsafe extern "C" fn check_tech(fighter: &mut L2CFighterCommon) -> bool {
    let passive_trigger_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("passive_trigger_frame")) as f32;
    let passive_trigger_frame_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("passive_trigger_frame_mul"), 0);
    fighter.sub_check_passive_button_for_damage((passive_trigger_frame * passive_trigger_frame_mul).into()).get_bool()
}

#[skyline::hook(replace = L2CFighterCommon_status_Passive)]
unsafe extern "C" fn status_passive(fighter: &mut L2CFighterCommon) -> L2CValue {
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x219c184305));
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("passive"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Passive_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_PassiveFB)]
unsafe extern "C" fn status_passivefb(fighter: &mut L2CFighterCommon) -> L2CValue {
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x219c184305));
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    let lr = PostureModule::lr(fighter.module_accessor);
    let mot = if fighter.global_table[STICK_X].get_f32() * lr < 0.0 {
        hash40("passive_stand_b")
    }
    else {
        hash40("passive_stand_f")
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
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_PassiveFB_Main as *const () as _))
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            is_enable_passive,
            // sub_check_passive_button,
            sub_check_passive_button_for_damage,
            sub_airchkpassive,
            sub_airchkpassive_for_damage,
            sub_airchkpassivewall,
            sub_airchkpassivewalljump,
            sub_airchkpassiveceil,
            status_passive,
            status_passivefb
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}