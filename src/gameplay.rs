use smash::{
    lua2cpp::{L2CFighterCommon, L2CAgentBase},
    phx::Hash40,
    app::{lua_bind::*, *},
    lib::lua_const::*
};
use smash_script::*;
use crate::{
    commonfuncs::*,
    vars::*
};

pub unsafe fn jump_cancel_check(agent: &mut L2CFighterCommon) {
    if AttackModule::is_infliction(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
        JUMP_CANCEL_HELPER[entry_id(agent.module_accessor)] = MotionModule::frame(agent.module_accessor);
    }
    if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT)
    && MotionModule::frame(agent.module_accessor) > JUMP_CANCEL_HELPER[entry_id(agent.module_accessor)] + 1.0
    && MotionModule::frame(agent.module_accessor) <= JUMP_CANCEL_HELPER[entry_id(agent.module_accessor)] + 10.0
    && ((ControlModule::get_command_flag_cat(agent.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0
    && ControlModule::is_enable_flick_jump(agent.module_accessor))
    || ControlModule::get_command_flag_cat(agent.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0) {
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            // StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
            agent.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), true.into());
        }
        else if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
            // StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
            agent.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), true.into());
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

pub unsafe fn dash_cancel_check(module_accessor: *mut BattleObjectModuleAccessor) {
    if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)
    && ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0 {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DASH, true);
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