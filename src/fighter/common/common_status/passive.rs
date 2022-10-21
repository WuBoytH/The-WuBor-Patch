use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        phx::*,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    wubor_utils::table_const::*,
    super::super::common_param::*
};

#[skyline::hook(replace = L2CFighterCommon_is_enable_passive)]
unsafe fn is_enable_passive(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_PACKMAN_EYE) {
        return false.into();
    }
    true.into()
}

pub unsafe fn is_bad_passive(fighter: &mut L2CFighterCommon) -> L2CValue {
    let weight = WorkModule::get_param_float(fighter.module_accessor, hash40("weight"), 0);
    let damage = DamageModule::damage(fighter.module_accessor, 0);
    (weight + passive::invalid_passive_damage_add <= damage).into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_check_passive_button)]
unsafe fn sub_check_passive_button(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    // The basis of the new tech system, teching is now performed by having your stick tilted in
    // any direction that *isn't* down, so neutral causes missed techs.
    // This is also why that param_1 argument goes unused, it doesn't matter to check it anymore.

    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let flick_x = fighter.global_table[FLICK_X].get_i32();
    let flick_y = fighter.global_table[FLICK_Y].get_i32();
    let passive_fb_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("passive_fb_cont_value"));
    let fb = passive_fb_cont_value <= stick_x.abs() && flick_x < 0xf0;
    let jump_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_stick_y"));
    let jump = jump_stick_y <= stick_y.abs() && flick_y < 0xf0;
    let guard_button = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD);
    let passive_input = ControlModule::get_trigger_count(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD as u8) < param_1.get_i32();
    (fb || jump || guard_button || passive_input).into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_check_passive_button_for_damage)]
unsafe fn sub_check_passive_button_for_damage(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    // Now just calls sub_check_passive_button

    fighter.sub_check_passive_button(param_1)
}

#[skyline::hook(replace = L2CFighterCommon_sub_AirChkPassive)]
unsafe fn sub_airchkpassive(fighter: &mut L2CFighterCommon) -> L2CValue {
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
unsafe fn sub_airchkpassive_for_damage(fighter: &mut L2CFighterCommon) -> L2CValue {
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
unsafe fn sub_airchkpassivewall(fighter: &mut L2CFighterCommon) -> L2CValue {
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
unsafe fn sub_airchkpassivewalljump(fighter: &mut L2CFighterCommon) -> L2CValue {
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
unsafe fn sub_airchkpassiveceil(fighter: &mut L2CFighterCommon) -> L2CValue {
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

unsafe fn check_tech(fighter: &mut L2CFighterCommon) -> bool {
    let passive_trigger_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("passive_trigger_frame")) as f32;
    let passive_trigger_frame_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("passive_trigger_frame_mul"), 0);
    fighter.sub_check_passive_button_for_damage((passive_trigger_frame * passive_trigger_frame_mul).into()).get_bool()
}

#[skyline::hook(replace = L2CFighterCommon_sub_uniq_process_Passive_init)]
unsafe fn sub_uniq_process_passive_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!()(fighter);
    let status = StatusModule::status_kind(fighter.module_accessor);
    if [
        *FIGHTER_STATUS_KIND_PASSIVE_WALL,
        *FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP,
        *FIGHTER_STATUS_KIND_PASSIVE_CEIL
    ].contains(&status) {
        if is_bad_passive(fighter).get_bool() {
            MotionModule::set_rate(fighter.module_accessor, passive::bad_passive_rate);
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);
        }
    }
    ret
}

#[skyline::hook(replace = L2CFighterCommon_status_Passive)]
unsafe fn status_passive(fighter: &mut L2CFighterCommon) -> L2CValue {
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x219c184305));
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    let bad_passive = is_bad_passive(fighter).get_bool();
    let rate = if bad_passive {
        passive::bad_passive_rate
    }
    else {
        1.0
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("passive"),
        0.0,
        rate,
        false,
        0.0,
        false,
        false
    );
    if bad_passive {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Passive_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_PassiveFB)]
unsafe fn status_passivefb(fighter: &mut L2CFighterCommon) -> L2CValue {
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x219c184305));
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    let lr = PostureModule::lr(fighter.module_accessor);
    let mot = if fighter.global_table[STICK_X].get_f32() * lr < 0.0 {
        hash40("passive_stand_b")
    }
    else {
        hash40("passive_stand_f")
    };
    let bad_passive = is_bad_passive(fighter).get_bool();
    let rate = if bad_passive {
        passive::bad_passive_rate
    }
    else {
        1.0
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(mot),
        0.0,
        rate,
        false,
        0.0,
        false,
        false
    );
    if bad_passive {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_PassiveFB_Main as *const () as _))
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
            sub_airchkpassiveceil,
            sub_uniq_process_passive_init,
            status_passive,
            status_passivefb
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}