use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    crate::{
        vars::*,
        table_const::*,
        cancels::*
    }
};

#[allow(non_snake_case)]
pub mod WarkModule {
    use super::*;
    #[export_name = "WarkModule__reset_i32"]
    pub unsafe extern "Rust" fn reset_i32(module_accessor: *mut BattleObjectModuleAccessor, flag: i32) {
        WorkModule::set_int(module_accessor, 0, flag);
    }
    
    #[export_name = "WarkModule__reset_f32"]
    pub unsafe extern "Rust" fn reset_f32(module_accessor: *mut BattleObjectModuleAccessor, flag: i32) {
        WorkModule::set_float(module_accessor, 0.0, flag);
    }
    
    #[export_name = "WarkModule__add_i32"]
    pub unsafe extern "Rust" fn add_i32(module_accessor: *mut BattleObjectModuleAccessor, flag: i32, amount: i32) {
        let counter = WorkModule::get_int(module_accessor, flag) + amount;
        WorkModule::set_int(module_accessor, counter, flag);
    }
    
    #[export_name = "WarkModule__add_f32"]
    pub unsafe extern "Rust" fn add_f32(module_accessor: *mut BattleObjectModuleAccessor, flag: i32, amount: f32) {
        let counter = WorkModule::get_float(module_accessor, flag) + amount;
        WorkModule::set_float(module_accessor, counter, flag);
    }
    
    #[export_name = "WarkModule__count_down"]
    pub unsafe extern "Rust" fn count_down(module_accessor: *mut BattleObjectModuleAccessor, flag: i32, amount: f32) {
        let slow_rate = SlowModule::rate(module_accessor);
        let global_slow_rate = sv_information::slow_rate();
        let counter = WorkModule::get_float(module_accessor, flag) - (amount * slow_rate * global_slow_rate);
        WorkModule::set_float(module_accessor, counter, flag);
    }

}

#[allow(non_snake_case)]
pub mod FGCModule {
    use super::*;
    #[export_name = "FGCModule__jump_cancel_check_hit"]
    pub unsafe extern "Rust" fn jump_cancel_check_hit(fighter: &mut L2CFighterCommon, jump_on_block: bool) -> L2CValue {
        let cancel_timer = WorkModule::get_float(fighter.module_accessor, FIGHTER_STATUS_WORK_ID_FLOAT_CANCEL_TIMER);
        if (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        || (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) && jump_on_block))
        && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
        && !fighter.global_table[IN_HITLAG].get_bool()
        && cancel_timer > 0.0 {
            let sit = fighter.global_table[SITUATION_KIND].get_i32();
            jump_cancel_common(fighter, sit.into())
        }
        else {
            false.into()
        }
    }

    #[export_name = "FGCModule__jump_cancel_check_exception"]
    pub unsafe extern "Rust" fn jump_cancel_check_exception(fighter: &mut L2CFighterCommon) -> L2CValue {
        let sit = fighter.global_table[SITUATION_KIND].get_i32();
        jump_cancel_common(fighter, sit.into())
    }

    #[export_name = "FGCModule__dash_cancel_check"]
    pub unsafe extern "Rust" fn dash_cancel_check(fighter: &mut L2CFighterCommon, dash_on_block: bool, reverse: bool) -> L2CValue {
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
        && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
        && !fighter.global_table[IN_HITLAG].get_bool()
        && cancel_timer > 0.0
        && ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & cat != 0
        && get_command_stick_direction(fighter.module_accessor, true) == dir {
            StatusModule::change_status_request_from_script(fighter.module_accessor, status, true);
            return true.into();
        }
        false.into()
    }
    
    #[export_name = "FGCModule__cancel_exceptions"]
    pub unsafe extern "Rust" fn cancel_exceptions(fighter: &mut L2CFighterCommon, next_status: i32, cat1_compare: i32, on_hit: bool) -> L2CValue {
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let cancel_timer = WorkModule::get_float(fighter.module_accessor, FIGHTER_STATUS_WORK_ID_FLOAT_CANCEL_TIMER);
        if !on_hit
        || ((AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))
        && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
        && !fighter.global_table[IN_HITLAG].get_bool()
        && cancel_timer > 0.0) {
            if (cat1 & cat1_compare) != 0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, next_status, true);
                return true.into();
            }
        }
        false.into()
    }

    #[export_name = "FGCModule__chain_cancels"]
    pub unsafe extern "Rust" fn chain_cancels(fighter: &mut L2CFighterCommon, next_status: i32, cat1_compare: i32, on_hit: bool, counter: i32, max: i32) -> L2CValue {
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

    #[export_name = "FGCModule__cancel_system"]
    pub unsafe extern "Rust" fn cancel_system(fighter: &mut L2CFighterCommon, normal_cancels: Vec<i32>, special_cancels: Vec<i32>, aerial_cancel: bool, jump_cancel: i32) {
        let cancel_timer = WorkModule::get_float(fighter.module_accessor, FIGHTER_STATUS_WORK_ID_FLOAT_CANCEL_TIMER);
        if (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))
        && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
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

    #[export_name = "FGCModule__get_command_stick_direction"]
    pub unsafe extern "Rust" fn get_command_stick_direction(module_accessor: *mut BattleObjectModuleAccessor, command: bool) -> i32 {
        let status_kind = StatusModule::status_kind(module_accessor);
        let mut stick_x = ControlModule::get_stick_x(module_accessor);
        if command {
            stick_x = stick_x * PostureModule::lr(module_accessor);
            if status_kind == *FIGHTER_STATUS_KIND_TURN_RUN {
                stick_x *= -1.0;
            }
        }
        
        if stick_x >= 0.4 {
            if ControlModule::get_stick_y(module_accessor) <= -0.25 {
                return 3;
            }
            else if ControlModule::get_stick_y(module_accessor) >= 0.25 {
                return 9;
            }
            else {
                return 6;
            }
        }
        else if stick_x <= -0.4 {
            if ControlModule::get_stick_y(module_accessor) <= -0.25 {
                return 1;
            }
            else if ControlModule::get_stick_y(module_accessor) >= 0.25 {
                return 7;
            }
            else {
                return 4;
            }
        }
        else {
            if ControlModule::get_stick_y(module_accessor) <= -0.25 {
                return 2;
            }
            else if ControlModule::get_stick_y(module_accessor) >= 0.25 {
                return 8;
            }
            else {
                return 5;
            }
        }
    }
    
    #[export_name = "FGCModule__inc_command"]
    pub unsafe extern "Rust" fn inc_command(fighter: &mut L2CFighterCommon, flag: i32, timer_flag: i32) {
        WorkModule::inc_int(fighter.module_accessor, flag);
        WorkModule::set_int(fighter.module_accessor, 0, timer_flag);
    }
    
}

#[allow(non_snake_case)]
pub mod MiscModule {
    use super::*;

    #[export_name = "MiscModule__is_damage_check"]
    pub unsafe extern "Rust" fn is_damage_check(module_accessor: *mut BattleObjectModuleAccessor, is_prev: bool) -> bool {
        let status : i32;
        let ret : bool;
        if is_prev {
            status = StatusModule::prev_status_kind(module_accessor, 0);
        }
        else {
            status = StatusModule::status_kind(module_accessor);
        }
        if FighterStopModuleImpl::is_damage_stop(module_accessor)
        || WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI)
        || WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)
        || WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR)
        || [
            *FIGHTER_STATUS_KIND_AIR_LASSO,
            *FIGHTER_STATUS_KIND_BIND,
            *FIGHTER_STATUS_KIND_BURY,
            *FIGHTER_STATUS_KIND_BURY_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_BEETLE,
            *FIGHTER_STATUS_KIND_CAPTURE_CUT,
            *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
            *FIGHTER_STATUS_KIND_CAPTURE_DRIVER,
            *FIGHTER_STATUS_KIND_CAPTURE_ITEM,
            *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE,
            *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND,
            *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD,
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED_FISHINGROD,
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL,
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_YOSHI,
            *FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON,
            *FIGHTER_STATUS_KIND_CATCHED_AIR_FALL_GANON,
            *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON,
            *FIGHTER_STATUS_KIND_CATCHED_GANON,
            *FIGHTER_STATUS_KIND_CATCHED_PICKEL_TROLLEY,
            *FIGHTER_STATUS_KIND_CATCHED_REFLET,
            *FIGHTER_STATUS_KIND_CATCHED_RIDLEY,
            *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN,
            *FIGHTER_STATUS_KIND_CLUNG_DAMAGE_DIDDY,
            *FIGHTER_STATUS_KIND_CLUNG_DIDDY,
            *FIGHTER_STATUS_KIND_CLUNG_GANON,
            *FIGHTER_STATUS_KIND_CLUNG_THROWN_BLANK_DIDDY,
            *FIGHTER_STATUS_KIND_CLUNG_THROWN_DIDDY,
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP,
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END,
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL,
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START,
            *FIGHTER_STATUS_KIND_DAMAGE_SONG,
            *FIGHTER_STATUS_KIND_DAMAGE_SONG_END,
            *FIGHTER_STATUS_KIND_DAMAGE_SONG_FALL,
            *FIGHTER_STATUS_KIND_DAMAGE_SONG_START,
            *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_DOWN,
            *FIGHTER_STATUS_KIND_DOWN_DAMAGE,
            *FIGHTER_STATUS_KIND_DOWN_EAT,
            *FIGHTER_STATUS_KIND_DOWN_SPOT,
            *FIGHTER_STATUS_KIND_DOWN_STAND,
            *FIGHTER_STATUS_KIND_DOWN_STAND_FB,
            *FIGHTER_STATUS_KIND_DOWN_WAIT,
            *FIGHTER_STATUS_KIND_FINAL,
            *FIGHTER_STATUS_KIND_FURAFURA,
            *FIGHTER_STATUS_KIND_FURAFURA_END,
            *FIGHTER_STATUS_KIND_FURAFURA_STAND,
            *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
            *FIGHTER_STATUS_KIND_ICE,
            *FIGHTER_STATUS_KIND_KOOPA_DIVED,
            *FIGHTER_STATUS_KIND_LAY_DOWN,
            *FIGHTER_STATUS_KIND_MEWTWO_THROWN,
            *FIGHTER_STATUS_KIND_MISS_FOOT,
            *FIGHTER_STATUS_KIND_PASSIVE,
            *FIGHTER_STATUS_KIND_PASSIVE_CEIL,
            *FIGHTER_STATUS_KIND_PASSIVE_FB,
            *FIGHTER_STATUS_KIND_PASSIVE_WALL,
            *FIGHTER_STATUS_KIND_REBIRTH,
            *FIGHTER_STATUS_KIND_SAVING_DAMAGE,
            *FIGHTER_STATUS_KIND_SAVING_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_SHIELD_BREAK_DOWN,
            *FIGHTER_STATUS_KIND_SHIELD_BREAK_FALL,
            *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY,
            *FIGHTER_STATUS_KIND_SLEEP,
            *FIGHTER_STATUS_KIND_SLIP,
            *FIGHTER_STATUS_KIND_SLIP_DAMAGE,
            *FIGHTER_STATUS_KIND_STABBED_DAMAGE,
            *FIGHTER_STATUS_KIND_STABBED_RIDLEY,
            *FIGHTER_STATUS_KIND_SWALLOWED,
            *FIGHTER_STATUS_KIND_THROWN
        ].contains(&status) {
            ret = true;
        }
        else {
            ret = false;
        }
        ret
    }

    #[export_name = "MiscModule__wall_jump_check"]
    pub unsafe extern "Rust" fn wall_jump_check(fighter: &mut L2CFighterCommon) {
        if GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32)
        || GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32) {
            if (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0
            && ControlModule::is_enable_flick_jump(fighter.module_accessor))
            || ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
            }
        }
    }

    #[export_name = "MiscModule__critical_zoom"]
    pub unsafe extern "Rust" fn critical_zoom(fighter: &mut L2CFighterCommon, rate: u8, frames: f32, zoom: f32) {
        if !SoundModule::is_playing(fighter.module_accessor, Hash40::new("se_common_finishhit")) {
            macros::EFFECT(fighter, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            if rate != 0 {
                SlowModule::set_whole(fighter.module_accessor, rate, 0);
            }
            if FighterUtil::get_opponent_fighter_num(fighter.module_accessor, true) < 2 {
                macros:: CAM_ZOOM_IN_arg5(fighter, frames, 0.0, zoom, 0.0, 0.0);
            }
            macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
        }
    }

    #[export_name = "MiscModule__set_hp"]
    pub unsafe extern "Rust" fn set_hp(fighter: &mut L2CFighterCommon, hp: f32) {
        if DamageModule::damage(fighter.module_accessor, 0) < hp
        && !smashball::is_training_mode() {
            let dmg = hp - DamageModule::damage(fighter.module_accessor, 0);
            DamageModule::add_damage(fighter.module_accessor, dmg, 0);
        }
    }

}
