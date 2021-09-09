use smash::{
    lua2cpp::L2CAgentBase,
    phx::Hash40,
    app::{lua_bind::*, *},
    lib::lua_const::*
};
use smash_script::*;
use crate::{
    commonfuncs::*,
    vars::*
};

pub unsafe fn jump_cancel_check_hit(agent: &mut L2CAgentBase, jump_on_block: bool) {
    if (AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT)
    || (AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_SHIELD) && jump_on_block))
    && MotionModule::frame(agent.module_accessor) > HIT_FRAME[entry_id(agent.module_accessor)] + 1.0
    && MotionModule::frame(agent.module_accessor) <= HIT_FRAME[entry_id(agent.module_accessor)] + 11.0
    && ((ControlModule::get_command_flag_cat(agent.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0
    && ControlModule::is_enable_flick_jump(agent.module_accessor))
    || ControlModule::get_command_flag_cat(agent.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0) {
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
        }
        else if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
            StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
        }
    }
}

pub unsafe fn jump_cancel_check_exception(agent: &mut L2CAgentBase) {
    if (ControlModule::get_command_flag_cat(agent.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0
    && ControlModule::is_enable_flick_jump(agent.module_accessor))
    || ControlModule::get_command_flag_cat(agent.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0 {
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
        }
        else if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
            StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
        }
    }
}

pub unsafe fn wall_jump_check(module_accessor: *mut BattleObjectModuleAccessor) {
    if GroundModule::is_wall_touch_line(module_accessor, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32)
    || GroundModule::is_wall_touch_line(module_accessor, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32) {
        if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0
        && ControlModule::is_enable_flick_jump(module_accessor))
        || ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0 {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
        }
    }
}

pub unsafe fn dash_cancel_check(agent: &mut L2CAgentBase, dash_on_block: bool, reverse: bool) {
    let dir;
    let cat;
    let status;
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
    if (AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT)
    || (AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_SHIELD) && dash_on_block))
    && MotionModule::frame(agent.module_accessor) > HIT_FRAME[entry_id(agent.module_accessor)] + 1.0
    && MotionModule::frame(agent.module_accessor) <= HIT_FRAME[entry_id(agent.module_accessor)] + 11.0
    && ControlModule::get_command_flag_cat(agent.module_accessor, 0) & cat != 0
    && get_command_stick_direction(agent.module_accessor, true) == dir {
        StatusModule::change_status_request_from_script(agent.module_accessor, status, true);
    }
}

pub unsafe fn critical_zoom(agent: &mut L2CAgentBase, rate : u8, frames : f32, zoom : f32) {
    if !SoundModule::is_playing(agent.module_accessor, Hash40::new("se_common_finishhit")) {
        macros::EFFECT(agent, Hash40::new("sys_bg_criticalhit"),Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        SlowModule::set_whole(agent.module_accessor, rate, 0);
        macros:: CAM_ZOOM_IN_arg5(agent, frames, 0.0, zoom, 0.0, 0.0);
        macros::PLAY_SE(agent, Hash40::new("se_common_criticalhit"));
    }
}

pub unsafe fn cancel_system(agent: &mut L2CAgentBase, status: i32, allowed_cancels: Vec<i32>) {
    let cat1 = ControlModule::get_command_flag_cat(agent.module_accessor, 0);
    let level : i32;
    if [
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_AIR
    ].contains(&status) {
        level = 1;
    }
    else if [
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_DASH
    ].contains(&status) {
        level = 2;
    }
    else if [
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4
    ].contains(&status) {
        level = 3;
    }
    else {
        level = 4;
    }

    if (AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_SHIELD))
    && MotionModule::frame(agent.module_accessor) > HIT_FRAME[entry_id(agent.module_accessor)] + 1.0
    && MotionModule::frame(agent.module_accessor) <= HIT_FRAME[entry_id(agent.module_accessor)] + 11.0 {
        if level <= 4 {
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 && allowed_cancels.contains(&*FIGHTER_STATUS_KIND_SPECIAL_N) {
                StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, false);
            }
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 && allowed_cancels.contains(&*FIGHTER_STATUS_KIND_SPECIAL_S) {
                StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
            }
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 && allowed_cancels.contains(&*FIGHTER_STATUS_KIND_SPECIAL_HI)
            && DISABLE_SPECIAL_HI[entry_id(agent.module_accessor)] == false {
                StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
            }
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 && allowed_cancels.contains(&*FIGHTER_STATUS_KIND_SPECIAL_LW) {
                StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
            }
        }
        if level <= 3 && StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 && allowed_cancels.contains(&*FIGHTER_STATUS_KIND_ATTACK_S4) {
                StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
            }
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 && allowed_cancels.contains(&*FIGHTER_STATUS_KIND_ATTACK_HI4) {
                StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, false);
            }
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 && allowed_cancels.contains(&*FIGHTER_STATUS_KIND_ATTACK_LW4) {
                StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, false);
            }
        }
        if level <= 2 {
            if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 && allowed_cancels.contains(&*FIGHTER_STATUS_KIND_ATTACK_S3) {
                    StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
                }
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 && allowed_cancels.contains(&*FIGHTER_STATUS_KIND_ATTACK_HI3) {
                    StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI3, false);
                }
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 && allowed_cancels.contains(&*FIGHTER_STATUS_KIND_ATTACK_LW3) {
                    StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
                }
            }
            else {
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 && allowed_cancels.contains(&*FIGHTER_STATUS_KIND_ATTACK_AIR) {
                    StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                }
            }
        }
    }
}

pub unsafe fn cancel_exceptions(agent: &mut L2CAgentBase, next_status: i32, cat1_compare: i32) {
    let cat1 = ControlModule::get_command_flag_cat(agent.module_accessor, 0);

    if (AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_SHIELD))
    && MotionModule::frame(agent.module_accessor) > HIT_FRAME[entry_id(agent.module_accessor)] + 2.0
    && MotionModule::frame(agent.module_accessor) <= HIT_FRAME[entry_id(agent.module_accessor)] + 12.0 {
        if (cat1 & cat1_compare) != 0 {
            StatusModule::change_status_request_from_script(agent.module_accessor, next_status, false);
        }
    }
}

pub unsafe fn set_hp(agent: &mut L2CAgentBase, hp: f32) {
    if DamageModule::damage(agent.module_accessor, 0) < hp
    && !smashball::is_training_mode() {
        let dmg = hp - DamageModule::damage(agent.module_accessor, 0);
        DamageModule::add_damage(agent.module_accessor, dmg, 0);
    }
}
