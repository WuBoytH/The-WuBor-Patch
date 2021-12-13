use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    crate::{
        common_funcs::*,
        vars::*,
        table_const::*,
        cancels::*
    }
};

pub unsafe fn jump_cancel_check_hit(fighter: &mut L2CFighterCommon, jump_on_block: bool) -> L2CValue {
    let cancel_timer = WorkModule::get_float(fighter.module_accessor, FIGHTER_STATUS_WORK_ID_FLOAT_CANCEL_TIMER);
    if (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) && jump_on_block))
    && !fighter.global_table[IN_HITLAG].get_bool()
    && cancel_timer > 0.0 {
        let sit = fighter.global_table[SITUATION_KIND].get_i32();
        jump_cancel_common(fighter, sit.into())
    }
    else {
        false.into()
    }
}

pub unsafe fn jump_cancel_check_exception(fighter: &mut L2CFighterCommon) -> L2CValue {
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    jump_cancel_common(fighter, sit.into())
}

pub unsafe fn wall_jump_check(fighter: &mut L2CFighterCommon) {
    if GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32)
    || GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32) {
        if (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0
        && ControlModule::is_enable_flick_jump(fighter.module_accessor))
        || ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
        }
    }
}

pub unsafe fn dash_cancel_check(fighter: &mut L2CFighterCommon, dash_on_block: bool, reverse: bool) -> L2CValue {
    let dir;
    let cat;
    let status;
    let cancel_timer = WorkModule::get_float(fighter.module_accessor, FIGHTER_STATUS_WORK_ID_FLOAT_CANCEL_TIMER);
    if reverse {
        dir = 4;
        cat = *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH;
        status = *FIGHTER_STATUS_KIND_TURN_DASH;
    }
    else {
        dir = 6;
        cat = *FIGHTER_PAD_CMD_CAT1_FLAG_DASH;
        status = *FIGHTER_STATUS_KIND_DASH;
    }
    if (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) && dash_on_block))
    && !fighter.global_table[IN_HITLAG].get_bool()
    && cancel_timer > 0.0
    && ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & cat != 0
    && get_command_stick_direction(fighter.module_accessor, true) == dir {
        StatusModule::change_status_request_from_script(fighter.module_accessor, status, true);
        return true.into();
    }
    false.into()
}

pub unsafe fn critical_zoom(fighter: &mut L2CFighterCommon, rate : u8, frames : f32, zoom : f32) {
    if !SoundModule::is_playing(fighter.module_accessor, Hash40::new("se_common_finishhit")) {
        macros::EFFECT(fighter, Hash40::new("sys_bg_criticalhit"),Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        if rate != 0 {
            SlowModule::set_whole(fighter.module_accessor, rate, 0);
        }
        if FighterUtil::get_opponent_fighter_num(fighter.module_accessor, true) < 2 {
            macros:: CAM_ZOOM_IN_arg5(fighter, frames, 0.0, zoom, 0.0, 0.0);
        }
        macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
    }
}

pub unsafe fn cancel_system(fighter: &mut L2CFighterCommon, normal_cancels: Vec<i32>, special_cancels: Vec<i32>, aerial_cancel: bool, jump_cancel: i32) {
    let cancel_timer = WorkModule::get_float(fighter.module_accessor, FIGHTER_STATUS_WORK_ID_FLOAT_CANCEL_TIMER);
    if (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))
    && !fighter.global_table[IN_HITLAG].get_bool()
    && cancel_timer > 0.0 {
        if jump_cancel != 0
        && jump_cancel_check_hit(fighter, jump_cancel == 2).get_bool() {
            return;
        }
        let sit = fighter.global_table[SITUATION_KIND].get_i32();
        if special_cancels.is_empty() == false
        && special_cancel_common(fighter, sit.into(), special_cancels).get_bool() {
            return;
        }
        if aerial_cancel
        && sit == *SITUATION_KIND_AIR
        && aerial_cancel_common(fighter).get_bool() {
            return;
        }
        if normal_cancels.is_empty() == false
        && sit == *SITUATION_KIND_GROUND
        && normal_cancel_common(fighter, normal_cancels).get_bool() {
            return;
        }
    }
}

pub unsafe fn cancel_exceptions(fighter: &mut L2CFighterCommon, next_status: i32, cat1_compare: i32, on_hit: bool) -> L2CValue {
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
    let cancel_timer = WorkModule::get_float(fighter.module_accessor, FIGHTER_STATUS_WORK_ID_FLOAT_CANCEL_TIMER);
    if !on_hit
    || ((AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))
    && !fighter.global_table[IN_HITLAG].get_bool()
    && cancel_timer > 0.0) {
        if (cat1 & cat1_compare) != 0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, next_status, true);
            return true.into();
        }
    }
    false.into()
}

pub unsafe fn chain_cancels(fighter: &mut L2CFighterCommon, next_status: i32, cat1_compare: i32, on_hit: bool, counter: i32, max: i32) -> L2CValue {
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
    let cancel_timer = WorkModule::get_float(fighter.module_accessor, FIGHTER_STATUS_WORK_ID_FLOAT_CANCEL_TIMER);
    if !on_hit
    || ((AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))
    && !fighter.global_table[IN_HITLAG].get_bool()
    && cancel_timer > 0.0) {
        if (cat1 & cat1_compare) != 0
        && WorkModule::get_int(fighter.module_accessor, counter) < max {
            StatusModule::change_status_request_from_script(fighter.module_accessor, next_status, true);
            WorkModule::inc_int(fighter.module_accessor, counter);
            return 1.into();
        }
    }
    0.into()
}

pub unsafe fn set_hp(fighter: &mut L2CFighterCommon, hp: f32) {
    if DamageModule::damage(fighter.module_accessor, 0) < hp
    && !smashball::is_training_mode() {
        let dmg = hp - DamageModule::damage(fighter.module_accessor, 0);
        DamageModule::add_damage(fighter.module_accessor, dmg, 0);
    }
}
