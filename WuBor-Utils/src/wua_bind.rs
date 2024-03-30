use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    custom_var::*,
    crate::{
        vars::*,
        table_const::*,
        cancels::*,
        app::*
    }
};

#[allow(non_snake_case)]
pub mod WarkModule {
    use super::*;

    /// A shortcut to reset i32 variables to 0.
    pub unsafe fn reset_i32(module_accessor: *mut BattleObjectModuleAccessor, flag: i32) {
        WorkModule::set_int(module_accessor, 0, flag);
    }

    /// A shortcut to reset f32 variables to 0.
    pub unsafe fn reset_f32(module_accessor: *mut BattleObjectModuleAccessor, flag: i32) {
        WorkModule::set_float(module_accessor, 0.0, flag);
    }

    /// A shortcut to add a value to an i32 variable.
    pub unsafe fn add_i32(module_accessor: *mut BattleObjectModuleAccessor, flag: i32, amount: i32) {
        let counter = WorkModule::get_int(module_accessor, flag) + amount;
        WorkModule::set_int(module_accessor, counter, flag);
    }

    /// A shortcut to add a value to an f32 variable.
    pub unsafe fn add_f32(module_accessor: *mut BattleObjectModuleAccessor, flag: i32, amount: f32) {
        let counter = WorkModule::get_float(module_accessor, flag) + amount;
        WorkModule::set_float(module_accessor, counter, flag);
    }

    /// A function for incrementing an f32 variable by an amount.
    /// This function takes into account the effects of slowdown, such as from
    /// Bayonett's Witch Time or from the Timer item.
    pub unsafe fn count_down(module_accessor: *mut BattleObjectModuleAccessor, flag: i32, amount: f32) {
        let slow_rate = SlowModule::rate(module_accessor);
        let global_slow_rate = sv_information::slow_rate();
        let counter = WorkModule::get_float(module_accessor, flag) - (amount * slow_rate * global_slow_rate);
        WorkModule::set_float(module_accessor, counter, flag);
    }

    pub unsafe fn is_operation_cpu(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
        if utility::get_category(&mut *module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
            return false;
        }
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as i32;
        let fighterentryid = smash::app::FighterEntryID(entry_id);
        let fighterinformation = smash::app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), fighterentryid);
        smash::app::lua_bind::FighterInformation::is_operation_cpu(fighterinformation)
    }
}

#[allow(non_snake_case)]
pub mod FGCModule {
    use super::*;

    /// A utility function that just checks if you're within the cancel window or not.
    pub unsafe fn check_cancel_window(fighter: &mut L2CFighterCommon) -> bool {
        let hit_frame = VarModule::get_float(fighter.module_accessor, fighter::status::float::HIT_FRAME);
        let motion_frame = fighter.global_table[STATUS_FRAME].get_f32();
        motion_frame - hit_frame <= 20.0 && !fighter.global_table[IS_STOP].get_bool() && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
    }

    /// A function used to enable jump-cancels, styled after the special cancel functions that Ryu, Ken, and Terry use.
    pub unsafe fn jump_cancel_check_hit(fighter: &mut L2CFighterCommon, jump_on_block: bool) -> L2CValue {
        if (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        || (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) && jump_on_block))
        && check_cancel_window(fighter) {
            let sit = fighter.global_table[SITUATION_KIND].get_i32();
            jump_cancel_common(fighter, sit.into())
        }
        else {
            false.into()
        }
    }

    /// A function used to enable jump-cancels, but it forces the jump-cancel regardless of if you hit anything.
    pub unsafe fn jump_cancel_check_exception(fighter: &mut L2CFighterCommon) -> L2CValue {
        let sit = fighter.global_table[SITUATION_KIND].get_i32();
        jump_cancel_common(fighter, sit.into())
    }

    /// Used to enable dash-cancels. You need to specify whether you're cancelling into a forward dash (reverse = false) or back dash (reverse = true);
    pub unsafe fn dash_cancel_check(fighter: &mut L2CFighterCommon, dash_on_block: bool, reverse: bool) -> L2CValue {
        let cat;
        let status;
        if reverse {
            cat = *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH;
            status = *FIGHTER_STATUS_KIND_TURN_DASH;
        }
        else {
            cat = *FIGHTER_PAD_CMD_CAT1_FLAG_DASH;
            status = *FIGHTER_STATUS_KIND_DASH;
        }
        if (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        || (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) && dash_on_block))
        && check_cancel_window(fighter)
        && ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & cat != 0 {
            VarModule::on_flag(fighter.module_accessor, fighter::status::flag::IS_DASH_CANCEL);
            StatusModule::change_status_request_from_script(fighter.module_accessor, status, true);
            return true.into();
        }
        false.into()
    }

    /// Used to check air dash cancels. This is set-up so you can only air dash, not air dodge.
    pub unsafe fn air_dash_cancel_check(fighter: &mut L2CFighterCommon, on_block: bool, whiff: bool) -> L2CValue {
        let sit = fighter.global_table[SITUATION_KIND].get_i32();
        if ((AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        || (on_block && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)))
        && check_cancel_window(fighter))
        || whiff {
            VarModule::on_flag(fighter.module_accessor, fighter::status::flag::FORCE_ESCAPE_AIR_SLIDE_IN_STATUS);
            if airdash_cancel_common(fighter, sit.into()).get_bool() {
                return true.into();
            }
        }
        false.into()
    }

    /// Enables cancels that would normally be impossible using the existing cancel system.
    ///
    /// # Arguments
    ///
    /// * `next_status` - The status kind you want to transition to (*FIGHTER_STATUS_KIND_XXXXXX)
    /// * `cat1_compare` - The cat1 flag you wish to check in order to transition to the next status (*FIGHTER_PAD_CMD_CAT1_FLAG_XXXXXX)
    ///
    /// # Example
    ///
    /// ```
    /// // Checks if you are in the Forward Tilt status, then checks if you input another Forward Tilt.
    /// // On hit, it will transition into Dash Attack.
    /// if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_S3 {
    ///     FGCModule::cancel_exceptions(fighter, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3, true);
    /// }
    /// ```
    pub unsafe fn cancel_exceptions(fighter: &mut L2CFighterCommon, next_status: i32, cat1_compare: i32, on_hit: bool) -> L2CValue {
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        if !on_hit
        || ((AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))
        && check_cancel_window(fighter)) {
            if (cat1 & cat1_compare) != 0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, next_status, true);
                return true.into();
            }
        }
        false.into()
    }

    /// Used for moves that are able to cancel into themselves.
    /// # Arguments
    ///
    /// * `cat1_compare` - The cat1 flag you wish to check in order to transition to the next status (*FIGHTER_PAD_CMD_CAT1_FLAG_XXXXXX)
    /// * `counter` - The constant you're using to store the number of times you chain cancelled a move.
    /// * `max` - How many times you will be allowed to chain cancel a move before needed to go into normal endlag.
    ///
    /// # Example
    ///
    /// ```
    /// // Checks if you are in the Down Tilt status, then checks if you input another Down Tilt.
    /// if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
    ///     FGCModule::chain_cancels(fighter, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3, true, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_D_TILT_CHAIN_COUNT, 2);
    /// }
    /// ```
    pub unsafe fn chain_cancels(fighter: &mut L2CFighterCommon, cat1_compare: i32, on_hit: bool, counter: i32, max: i32) -> L2CValue {
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        if !on_hit
        || ((AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))
        && check_cancel_window(fighter)) {
            let count = VarModule::get_int(fighter.module_accessor, counter) + 1;
            if (cat1 & cat1_compare) != 0
            && count <= max {
                fighter.attack_mtrans_pre_process();
                VarModule::inc_int(fighter.module_accessor, counter);
                return 1.into();
            }
        }
        0.into()
    }

    /// The generic cancel system, used to enable special cancels for any character.
    /// # Arguments
    ///
    /// * `normal_cancels` - A vector of all of the ground attack transition terms you can cancel into.
    /// * `special_cancels` - A vector of all of the special move transition terms you can cancel into.
    /// * `aerial_cancel` - Checks if you can cancel into an aerial.
    /// * `jump_cancel` - Checks if you can jump-cancel. 0 = None | 1 = On Hit | 2 = On Hit or Block
    pub unsafe fn cancel_system(fighter: &mut L2CFighterCommon, normal_cancels: Vec<i32>, special_cancels: Vec<i32>, aerial_cancel: bool, jump_cancel: i32) -> L2CValue {
        if (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))
        && check_cancel_window(fighter) {
            if jump_cancel != 0
            && jump_cancel_check_hit(fighter, jump_cancel == 2).get_bool() {
                return true.into();
            }
            let sit = fighter.global_table[SITUATION_KIND].get_i32();
            if special_cancels.is_empty() == false
            && special_cancel_common(fighter, sit.into(), special_cancels).get_bool() {
                return true.into();
            }
            if aerial_cancel
            && sit == *SITUATION_KIND_AIR
            && aerial_cancel_common(fighter).get_bool() {
                return true.into();
            }
            if normal_cancels.is_empty() == false
            && sit == *SITUATION_KIND_GROUND
            && normal_cancel_common(fighter, normal_cancels).get_bool() {
                return true.into();
            }
        }
        false.into()
    }

    /// Checks the direction of the left stick and returns a number between 1 and 9, representing numpad notation.
    /// # Arguments
    /// 
    /// * `command` - Set to true to have the horizontal stick value reversed, so that it checks the input as if you are facing right.
    pub unsafe fn get_command_stick_direction(fighter: &mut L2CFighterCommon, command: bool) -> i32 {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let mut stick_x = fighter.global_table[STICK_X].get_f32();
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        if command {
            stick_x = stick_x * PostureModule::lr(fighter.module_accessor);
            if status_kind == *FIGHTER_STATUS_KIND_TURN_RUN {
                stick_x *= -1.0;
            }
        }
        let length = sv_math::vec2_length(stick_x, stick_y);
        if length < 0.25 {
            return 5;
        }
        let normalize = sv_math::vec2_normalize(stick_x, stick_y);
        let dirx = normalize.x;
        let diry = normalize.y;
        let arctangent = diry.atan2(dirx.abs());
        let degrees = arctangent.to_degrees();
        if degrees.abs() <= 15.0 {
            if stick_x > 0.0 {
                return 6;
            }
            return 4;
        }
        else if 70.0 <= degrees.abs() {
            if stick_y > 0.0 {
                return 8;
            }
            return 2;
        }
        else {
            if stick_x > 0.0 {
                if stick_y > 0.0 {
                    return 9;
                }
                return 3;
            }
            if stick_y > 0.0 {
                return 7;
            }
            return 1;
        }
    }

    /// Used for command inputs. Currently goes unused.
    // pub unsafe fn inc_command(fighter: &mut L2CFighterCommon, flag: i32, timer_flag: i32) {
    //     WorkModule::inc_int(fighter.module_accessor, flag);
    //     WorkModule::set_int(fighter.module_accessor, 0, timer_flag);
    // }
    
    /// Checks the timer for a command input. Resets the command input if the timer exceeds the window, otherwise increments the timer by 1.
    // pub unsafe fn check_command_inc(fighter: &mut L2CFighterCommon, flag: i32, timer_flag: i32, window: i32) {
    //     if WorkModule::get_int(fighter.module_accessor, timer_flag) <= window {
    //         if WorkModule::get_int(fighter.module_accessor, flag) > 1 {
    //             WorkModule::inc_int(fighter.module_accessor, timer_flag);
    //         }
    //     }
    //     else {
    //         WarkModule::reset_i32(fighter.module_accessor, flag);
    //         WorkModule::set_int(fighter.module_accessor, 0, timer_flag);
    //     }
    // }

    /// Disables a grounded attack. Used for cancel systems with complex cancel trees.
    pub unsafe fn disable_ground_normal(fighter: &mut L2CFighterCommon, ground_normal_mask: i32) {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) {
            let mut used_ground_normals = VarModule::get_int(fighter.module_accessor, fighter::instance::int::USED_GROUND_NORMALS);
            if used_ground_normals & ground_normal_mask == 0 {
                used_ground_normals += ground_normal_mask;
            }
            VarModule::set_int(fighter.module_accessor, fighter::instance::int::USED_GROUND_NORMALS, used_ground_normals);
        }
    }

    /// Used when checking for attack inputs to disable certain attacks if they are used in a string.
    /// Used for cancel systems with complex cancel trees.
    pub unsafe fn set_used_ground_normal_transition_terms(fighter: &mut L2CFighterCommon) {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) {
            let used_mask = VarModule::get_int(fighter.module_accessor, fighter::instance::int::USED_GROUND_NORMALS);
            if used_mask & ATTACK_N_MASK != 0 {
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT);
            }
            if used_mask & ATTACK_S3_MASK != 0 {
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_3);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S3);
            }
            if used_mask & ATTACK_HI3_MASK != 0 {
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
            }
            if used_mask & ATTACK_LW3_MASK != 0 {
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
            }
            if used_mask & ATTACK_S4_MASK != 0 {
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4);
            }
            if used_mask & ATTACK_HI4_MASK != 0 {
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
            }
            if used_mask & ATTACK_LW4_MASK != 0 {
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
            }
        }
    }

    /// Resets your current cancel string. Typically only resets if CancelModule::is_enable_cancel is true, but
    /// the ignore flag can be passed to bypass that.
    pub unsafe fn reset_used_ground_normals(fighter: &mut L2CFighterCommon, ignore: bool) {
        if ignore || (CancelModule::is_enable_cancel(fighter.module_accessor)
        || MotionModule::is_end(fighter.module_accessor))
        || ![
            *FIGHTER_STATUS_KIND_ATTACK,
            *FIGHTER_STATUS_KIND_ATTACK_100,
            *FIGHTER_STATUS_KIND_ATTACK_DASH,
            *FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            *FIGHTER_STATUS_KIND_ATTACK_LW3,
            *FIGHTER_STATUS_KIND_ATTACK_S4_START,
            *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
            *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_HI4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
            *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_LW4
        ].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
            VarModule::set_int(fighter.module_accessor, fighter::instance::int::USED_GROUND_NORMALS, 0);
        }
    }

    /// Disables an aerial. Used for characters who can cancel aerials into other aerials.
    pub unsafe fn disable_aerial(fighter: &mut L2CFighterCommon, aerial_mask: i32) {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) {
            let mut used_aerials = VarModule::get_int(fighter.module_accessor, fighter::instance::int::USED_AERIALS);
            if used_aerials & aerial_mask == 0 {
                used_aerials += aerial_mask;
            }
            VarModule::set_int(fighter.module_accessor, fighter::instance::int::USED_AERIALS, used_aerials);
        }
    }

    /// Checks if certain aerials are enabled.
    pub unsafe fn check_enabled_aerial(fighter: &mut L2CFighterCommon) -> bool {
        if VarModule::is_flag(fighter.module_accessor, fighter::status::flag::ENABLE_AERIAL_STRING)
        && !CancelModule::is_enable_cancel(fighter.module_accessor) {
            let enabled_mask = VarModule::get_int(fighter.module_accessor, fighter::status::int::ENABLED_AERIALS);
            let used_mask = VarModule::get_int(fighter.module_accessor, fighter::instance::int::USED_AERIALS);
            let attack_air_kind = ControlModule::get_attack_air_kind(fighter.module_accessor);
            let aerial_flag;
            match attack_air_kind {
                1 => aerial_flag = ATTACK_AIR_N_MASK,
                2 => aerial_flag = ATTACK_AIR_F_MASK,
                3 => aerial_flag = ATTACK_AIR_B_MASK,
                4 => aerial_flag = ATTACK_AIR_HI_MASK,
                5 => aerial_flag = ATTACK_AIR_LW_MASK,
                _ => aerial_flag = 0b00000
            }
            return enabled_mask & aerial_flag != 0 && used_mask & aerial_flag == 0;
        }
        true
    }

    /// Resets your aerial cancel string, enabling all aerials again.
    pub unsafe fn reset_used_aerials(fighter: &mut L2CFighterCommon) {
        VarModule::set_int(fighter.module_accessor, fighter::instance::int::USED_AERIALS, 0);
    }

    /// Handles adding or subtracting meter.
    pub unsafe fn update_meter(module_accessor: *mut BattleObjectModuleAccessor, amount: f32, meter_max: f32, meter_const: i32) {
        let mut meter = VarModule::get_float(module_accessor, meter_const);
        meter += amount;
        if meter < 0.0 {
            meter = 0.0;
        }
        if meter > meter_max {
            meter = meter_max;
        }
        VarModule::set_float(module_accessor, meter_const, meter);
    }

    /// Calls the EX flash graphic.
    #[inline(always)]
    pub unsafe fn ex_flash(fighter: &mut L2CAgentBase) {
        if macros::is_excute(fighter) {
            let lr = PostureModule::lr(fighter.module_accessor);
            macros::EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 8.0 * lr, 10, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.831, 0.686, 0.216);
        }
    }

    /// Sets a command input to only use certain buttons.
    pub unsafe fn set_command_input_button(module_accessor: *mut BattleObjectModuleAccessor, command: usize, buttons: u8) {
        let control_module = *(module_accessor as *const *const u64).add(0x48 / 8);
        let command_input = *control_module.add((0x7f0 + (command * 8)) / 8) as *mut u8;
        *command_input.add(0xb) = buttons;
    }
}

#[allow(non_snake_case)]
pub mod ThrowUtils {
    use super::*;

    /// Gets the thrown opponent boma.
    pub unsafe fn get_thrown_object(module_accessor: *mut BattleObjectModuleAccessor) -> Option<*mut BattleObject> {
        let link_id = LinkModule::get_node_object_id(module_accessor, *LINK_NO_CAPTURE) as u32;
        let object = MiscModule::get_battle_object_from_id(link_id);
        if object.is_null() {
            None
        }
        else {
            Some(object)
        }
    }

    /// Sets the thrown opponent's rate.
    pub unsafe fn set_thrown_rate(module_accessor: *mut BattleObjectModuleAccessor, rate: f32) {
        if let Some(object) = get_thrown_object(module_accessor) {
            MotionModule::set_rate((*object).module_accessor, rate);
        }
    }

    /// Forces the "launched" knockback state.
    pub unsafe fn set_force_launch(module_accessor: *mut BattleObjectModuleAccessor) {
        if let Some(object) = get_thrown_object(module_accessor) {
            if sv_battle_object::category((*object).battle_object_id) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                VarModule::on_flag(module_accessor, thrown::flag::FORCE_LAUNCHED);
            }
        }
    }
}

#[allow(non_snake_case)]
pub mod MiscModule {
    use super::*;

    /// Checks if your current status is one where you're being damaged. If is_prev is true, it checks your previous status instead.
    pub unsafe fn is_damage_check(module_accessor: *mut BattleObjectModuleAccessor, is_prev: bool) -> bool {
        let status : i32;
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
        || (*FIGHTER_STATUS_KIND_CAPTURE_PULLED..=*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status)
        || (*FIGHTER_STATUS_KIND_DOWN..=*FIGHTER_STATUS_KIND_LAY_DOWN).contains(&status)
        || (*FIGHTER_STATUS_KIND_DOWN_DAMAGE..=*FIGHTER_STATUS_KIND_BIND).contains(&status)
        || (*FIGHTER_STATUS_KIND_SLIP..=*FIGHTER_STATUS_KIND_SLIP_WAIT).contains(&status)
        || (*FIGHTER_STATUS_KIND_TREAD_DAMAGE..=*FIGHTER_STATUS_KIND_ICE_JUMP).contains(&status)
        || (*FIGHTER_STATUS_KIND_LINK_FINAL..=*FIGHTER_STATUS_KIND_PIT_FALL).contains(&status)
        || (*FIGHTER_STATUS_KIND_SWALLOWED..=*FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI).contains(&status)
        || (*FIGHTER_STATUS_KIND_CATCHED_REFLET..=*FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND).contains(&status)
        || status == *FIGHTER_STATUS_KIND_GIMMICK_EATEN
        || (*FIGHTER_STATUS_KIND_CAPTURE_ITEM..=*FIGHTER_STATUS_KIND_CAPTURE_CLAPTRAP).contains(&status)
        || (*FIGHTER_STATUS_KIND_FINAL_VISUAL_ATTACK_OTHER..=*FIGHTER_STATUS_KIND_RIDLEY_FINAL_TARGET_END).contains(&status)
        || (*FIGHTER_STATUS_KIND_CATCHED_RIDLEY..=*FIGHTER_STATUS_KIND_STABBED_DAMAGE).contains(&status)
        || (*FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED..=*FIGHTER_STATUS_KIND_SWING_GAOGAEN_FAILURE).contains(&status)
        || (*FIGHTER_STATUS_KIND_SHEIK_FINAL_CAPTURE..=*FIGHTER_STATUS_KIND_CAPTURE_WAIT_OCTOPUS).contains(&status)
        || (*FIGHTER_STATUS_KIND_SIMON_FINAL_TARGET_START..=*FIGHTER_STATUS_KIND_YOSHI_FINAL_TARGET_END).contains(&status)
        || (*FIGHTER_STATUS_KIND_SUICIDE_BOMB..=*FIGHTER_STATUS_KIND_TANTAN_FINAL_TARGET_END).contains(&status)
        || (*FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD..=*FIGHTER_STATUS_KIND_EDGE_FINAL_TARGET_END).contains(&status)
        || (*FIGHTER_STATUS_KIND_CAPTURE_TRAIL_KEYHOLE..=*FIGHTER_STATUS_KIND_TRAIL_FINAL_TARGET_END).contains(&status) {
            return true;
        }
        false
    }

    /// Checks if your current status is considered getting damaged, but is different from just normally getting hit.
    /// If is_prev is true, it checks your previous status instead.
    pub unsafe fn is_illegal_status(module_accessor: *mut BattleObjectModuleAccessor, is_prev: bool) -> bool {
        let status : i32;
        if is_prev {
            status = StatusModule::prev_status_kind(module_accessor, 0);
        }
        else {
            status = StatusModule::status_kind(module_accessor);
        }
        if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI)
        || (*FIGHTER_STATUS_KIND_CAPTURE_PULLED..=*FIGHTER_STATUS_KIND_THROWN).contains(&status)
        || (*FIGHTER_STATUS_KIND_FURAFURA_STAND..=*FIGHTER_STATUS_KIND_BIND).contains(&status)
        || (*FIGHTER_STATUS_KIND_SLIP..=*FIGHTER_STATUS_KIND_SLIP_WAIT).contains(&status)
        || (*FIGHTER_STATUS_KIND_CLUNG_GANON..=*FIGHTER_STATUS_KIND_ICE_JUMP).contains(&status)
        || (*FIGHTER_STATUS_KIND_LINK_FINAL..=*FIGHTER_STATUS_KIND_PIT_FALL).contains(&status)
        || (*FIGHTER_STATUS_KIND_SWALLOWED..=*FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI).contains(&status)
        || (*FIGHTER_STATUS_KIND_CATCHED_REFLET..=*FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND).contains(&status)
        || status == *FIGHTER_STATUS_KIND_GIMMICK_EATEN
        || (*FIGHTER_STATUS_KIND_CAPTURE_ITEM..=*FIGHTER_STATUS_KIND_CAPTURE_CLAPTRAP).contains(&status)
        || (*FIGHTER_STATUS_KIND_FINAL_VISUAL_ATTACK_OTHER..=*FIGHTER_STATUS_KIND_RIDLEY_FINAL_TARGET_END).contains(&status)
        || (*FIGHTER_STATUS_KIND_CATCHED_RIDLEY..=*FIGHTER_STATUS_KIND_STABBED_DAMAGE).contains(&status)
        || (*FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED..=*FIGHTER_STATUS_KIND_SWING_GAOGAEN_FAILURE).contains(&status)
        || (*FIGHTER_STATUS_KIND_SHEIK_FINAL_CAPTURE..=*FIGHTER_STATUS_KIND_CAPTURE_WAIT_OCTOPUS).contains(&status)
        || (*FIGHTER_STATUS_KIND_SIMON_FINAL_TARGET_START..=*FIGHTER_STATUS_KIND_YOSHI_FINAL_TARGET_END).contains(&status)
        || (*FIGHTER_STATUS_KIND_SUICIDE_BOMB..=*FIGHTER_STATUS_KIND_TANTAN_FINAL_TARGET_END).contains(&status)
        || (*FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD..=*FIGHTER_STATUS_KIND_EDGE_FINAL_TARGET_END).contains(&status)
        || (*FIGHTER_STATUS_KIND_CAPTURE_TRAIL_KEYHOLE..=*FIGHTER_STATUS_KIND_TRAIL_FINAL_TARGET_END).contains(&status) {
            return true;
        }
        false
    }

    /// Forces a wall jump. Will be moved to FGCModule eventually.
    pub unsafe fn wall_jump_check(fighter: &mut L2CFighterCommon) {
        let is_right = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
        let is_left = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
        let lr = PostureModule::lr(fighter.module_accessor);
        if is_right {
            let cat1 = fighter.global_table[CMD_CAT1].get_i32();
            let dash = if lr < 0.0 { *FIGHTER_PAD_CMD_CAT1_FLAG_DASH } else { *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH };
            if cat1 & (dash | *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
                return;
            }
        }
        if is_left {
            let cat1 = fighter.global_table[CMD_CAT1].get_i32();
            let dash = if lr < 0.0 { *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH } else { *FIGHTER_PAD_CMD_CAT1_FLAG_DASH };
            if cat1 & (dash | *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
                return;
            }
        }
    }

    /// Sets all of the values needed for Taunt Hold/Loops.
    pub unsafe fn set_appeal_loop(module_accessor: *mut BattleObjectModuleAccessor, is_loop: bool, loop_mot: u64, restart_frame: i32) {
        VarModule::set_int(module_accessor, appeal::int::RESTART_FRAME, restart_frame);
        VarModule::set_int64(module_accessor, appeal::int64::LOOP_MOT, loop_mot);
        VarModule::set_int64(module_accessor, appeal::int64::ACTION_MOT, smash::hash40("invalid"));
        if is_loop {
            VarModule::on_flag(module_accessor, appeal::flag::LOOP);
        }
        else {
            VarModule::on_flag(module_accessor, appeal::flag::HOLD);
        }
    }

    pub fn get_active_battle_object_id_from_entry_id(entry_id: u32) -> Option<u32> {
        use smash::lib::lua_const::*;
        use smash::app::lua_bind::*;
        let object = get_battle_object_from_entry_id(entry_id)?;
        if object.is_null() { return None; }
        let object = unsafe { &mut *object };
        let kind = object.kind as i32;
        let status = unsafe {
            StatusModule::status_kind(object.module_accessor)
        };
        if status != *FIGHTER_STATUS_KIND_NONE && status != *FIGHTER_STATUS_KIND_STANDBY {
            return Some(object.battle_object_id);
        }
        if kind == *FIGHTER_KIND_ELIGHT || kind == *FIGHTER_KIND_EFLAME {
            Some(object.battle_object_id + 0x10000)
        } else if kind == *FIGHTER_KIND_PZENIGAME || kind == *FIGHTER_KIND_PFUSHIGISOU || kind == *FIGHTER_KIND_PLIZARDON {
            let next_id = object.battle_object_id + 0x10000;
            let next_object = unsafe { &mut *get_battle_object_from_id(next_id) };
            let next_status = unsafe {
                StatusModule::status_kind(next_object.module_accessor)
            };
            if next_status != *FIGHTER_STATUS_KIND_NONE && next_status != *FIGHTER_STATUS_KIND_STANDBY {
                Some(next_id)
            } else {
                Some(next_id + 0x10000)
            }
        } else {
            Some(object.battle_object_id)
        }
    }
    
    pub fn get_battle_object_from_entry_id(entry_id: u32) -> Option<*mut BattleObject> {
        unsafe {
            let entry = get_fighter_entry(singletons::FighterManager(), entry_id);
            if entry.is_null() {
                None
            } else {
                Some(*(entry.add(0x4160) as *mut *mut BattleObject))
            }
        }
    }
    
    #[skyline::from_offset(0x3ac560)]
    pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

    pub unsafe fn get_vars_from_pocket(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
        // println!("Weapon ID: {:#x}", (*module_accessor).battle_object_id);
        if (*module_accessor).battle_object_id >> 0x1c != *BATTLE_OBJECT_CATEGORY_WEAPON as u32 {
            return false;
        }
        let owner_object_id = WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        // println!("Owner ID: {:#x}", owner_object_id);
        let owner_cat = sv_battle_object::category(owner_object_id);
        // println!("Owner Category: {:#x}", owner_cat);
        let owner_kind = sv_battle_object::kind(owner_object_id);
        // println!("Owner Kind: {:#x}", owner_kind);
        if owner_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER
        && [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_KIRBY].contains(&owner_kind) {
            // println!("Owner objects are either Villager or Isabelle!");
            let owner_module_accessor = sv_battle_object::module_accessor(owner_object_id);
            if owner_kind == *FIGHTER_KIND_KIRBY
            && ![*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE].contains(&WorkModule::get_int(owner_module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA)) {
                return false;
            }
            let pocket_object_id = WorkModule::get_int(owner_module_accessor, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_OBJECT_ID) as u32;
            // println!("Pocketed Item Object ID: {:#x}", pocket_object_id);
            if (*module_accessor).battle_object_id == pocket_object_id {
                // println!("Pocket object and new object match! Retrieving pocketed vars...");
                VarModule::retrieve_pocketed_vars(module_accessor, owner_object_id);
                VarModule::on_flag(module_accessor, weapon::instance::flag::FROM_POCKET);
                return true;
            }
        }
        false
    }

    pub unsafe fn get_vars_from_pocket_by_object_id(object_id: u32, owner_object_id: u32) -> bool {
        if object_id >> 0x1c != *BATTLE_OBJECT_CATEGORY_WEAPON as u32 {
            return false;
        }
        // println!("Owner ID: {:#x}", owner_object_id);
        let owner_cat = sv_battle_object::category(owner_object_id);
        // println!("Owner Category: {:#x}", owner_cat);
        let owner_kind = sv_battle_object::kind(owner_object_id);
        // println!("Owner Kind: {:#x}", owner_kind);
        if owner_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER
        && [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_KIRBY].contains(&owner_kind) {
            // println!("Owner objects are either Villager or Isabelle!");
            let owner_module_accessor = sv_battle_object::module_accessor(owner_object_id);
            if owner_kind == *FIGHTER_KIND_KIRBY
            && ![*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE].contains(&WorkModule::get_int(owner_module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA)) {
                return false;
            }
            let pocket_object_id = WorkModule::get_int(owner_module_accessor, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_OBJECT_ID) as u32;
            // println!("Pocketed Item Object ID: {:#x}", pocket_object_id);
            if object_id == pocket_object_id {
                // println!("Pocket object and new object match! Retrieving pocketed vars...");
                VarModule::retrieve_pocketed_vars_by_object_id(object_id, owner_object_id);
                VarModule::set_flag_from_object_id(object_id, weapon::instance::flag::FROM_POCKET, true);
                return true;
            }
        }
        false
    }

    #[inline(always)]
    pub unsafe fn calc_motion_rate_from_cancel_frame(fighter: &mut L2CAgentBase, current_frame: f32, adjust_frame: f32) {
        let mot = MotionModule::motion_kind(fighter.module_accessor);
        let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(mot), true);
        let adjusted_frame = cancel_frame - current_frame;
        if adjusted_frame > 0.0
        && adjusted_frame + adjust_frame > 0.0 {
            macros::FT_MOTION_RATE(fighter, (adjusted_frame + adjust_frame) / adjusted_frame);
            sv_animcmd::frame(fighter.lua_state_agent, cancel_frame);
            macros::FT_MOTION_RATE(fighter, 1.0);
        }
    }

    #[inline(always)]
    pub unsafe fn calc_motion_rate_from_end_frame(fighter: &mut L2CAgentBase, current_frame: f32, adjust_frame: f32) {
        let end_frame = MotionModule::end_frame(fighter.module_accessor);
        let adjusted_frame = end_frame - current_frame;
        if adjusted_frame > 0.0
        && adjusted_frame + adjust_frame > 0.0 {
            macros::FT_MOTION_RATE(fighter, (adjusted_frame + adjust_frame) / adjusted_frame);
            sv_animcmd::frame(fighter.lua_state_agent, end_frame);
            macros::FT_MOTION_RATE(fighter, 1.0);
        }
    }
    
    #[skyline::from_offset(0x696720)]
    pub fn call_critical(
        module_accessor: *mut BattleObjectModuleAccessor,
        unk: u64,
        unk2: i32,
        param_struct: u64,
        unk3: i32,
        unk4: i32,
        unk5: i32,
        unk6: i32
    ) -> u64;

    pub fn patch_vtable_function(offset: usize, function: u64) {
        // println!("function ptr: {:#x}", function as u64);
        let low = (function as u64 & 0xFFFFFFFF) as u32;
        // println!("Low to Big Endian: {:#x} > {:#?}", low, low.to_be_bytes());
        let high = (function as u64 >> 32) as u32;
        // println!("High to Big Endian: {:#x} > {:#?}", high, high.to_be_bytes());
        let _ = skyline::patching::Patch::in_text(offset).data(low);
        let _ = skyline::patching::Patch::in_text(offset + 0x4).data(high);
    }

    /// Used to create custom shield boxes that can be called in ACMD. Below is an example of how it would be created.
    /// ```
    /// #[skyline::hook(offset = 0xcd98a0)]
    /// unsafe extern "C" fn marth_lucina_init(vtable: u64, fighter: &mut Fighter) {
    ///     original!()(vtable, fighter);
    ///     if (*fighter).battle_object.kind == *FIGHTER_KIND_LUCINA as u32 {
    ///         let shield_data = ShieldDataResource::new(
    ///             0.0,
    ///             0.0,
    ///             0.0,
    ///             0.0,
    ///             0.0,
    ///             0.0,
    ///             10.0,
    ///             Hash40::new("hip"),
    ///             *COLLISION_SHAPE_TYPE_CAPSULE as u8,
    ///             *SHIELD_TYPE_UNDEFINED as u8
    ///         );
    ///         let shield_datas = &mut (ShieldDatas::new().add(shield_data, 0));
    ///         let resource = &mut ShieldGroupResource::new(
    ///             shield_datas,
    ///             1,
    ///             0,
    ///             false,
    ///             false,
    ///             false
    ///         );
    ///         let module_accessor = (*fighter).battle_object.module_accessor;
    ///         MiscModule::add_shield_group(module_accessor, resource, 2);
    ///     }
    /// }
    /// ````
    pub unsafe fn add_shield_group(module_accessor: *mut BattleObjectModuleAccessor, resource: *mut ShieldGroupResource, group_id: i32) {
        // Get Vtable Function
        let shield_module = (module_accessor as *mut u64).add(0x100 / 8);
        let vtable = *shield_module as *const u64;
        let ptr = *((*vtable + 0x58) as *const u64);
        let set_shield_group: extern "C" fn(*mut u64, *mut ShieldGroupResource, i32) = std::mem::transmute(ptr);

        // Redefine ShieldModule
        let shield_module = *(module_accessor as *mut *mut u64).add(0x100 / 8);

        set_shield_group(shield_module, resource, group_id);
        let count = (*resource).count as i32;
        if count > 0 {
            for x in 0..count {
                ShieldModule::set_status(module_accessor, x, ShieldStatus(*SHIELD_STATUS_NONE), group_id);
            }
        }
    }

    /// Used to create custom reflector boxes that can be called in ACMD. Below is an example of how it would be created.
    /// ```
    /// #[skyline::hook(offset = 0xcd98a0)]
    /// unsafe extern "C" fn marth_lucina_init(vtable: u64, fighter: &mut Fighter) {
    ///     original!()(vtable, fighter);
    ///     if (*fighter).battle_object.kind == *FIGHTER_KIND_LUCINA as u32 {
    ///         let shield_data = ShieldData::new(
    ///             0.0,
    ///             0.0,
    ///             0.0,
    ///             0.0,
    ///             0.0,
    ///             0.0,
    ///             10.0,
    ///             Hash40::new("hip"),
    ///             *COLLISION_SHAPE_TYPE_CAPSULE as u8,
    ///             *SHIELD_TYPE_UNDEFINED as u8
    ///         );
    ///         let shield_datas = &mut (ShieldDatas2::new().add(shield_data, 0));
    ///         let resource = &mut ShieldGroupResource2::new(
    ///             shield_datas,
    ///             1,
    ///             4.0,
    ///             4.0,
    ///             100.0,
    ///             20.0,
    ///             false,
    ///             0
    ///         );
    ///         let module_accessor = (*fighter).battle_object.module_accessor;
    ///         MiscModule::add_reflector_group(module_accessor, resource, 1);
    ///         ReflectorModule::set_hop(module_accessor, true, 45.0, 1);
    ///     }
    /// }
    /// ````
    pub unsafe fn add_reflector_group(module_accessor: *mut BattleObjectModuleAccessor, resource: *mut ShieldGroupResource2, group_id: i32) {
        // Get Vtable Function
        let reflector_module = (module_accessor as *mut u64).add(0x108 / 8);
        let vtable = *reflector_module as *const u64;
        let ptr = *((*vtable + 0x60) as *const u64);
        let set_shield_group2: extern "C" fn(*mut u64, *mut ShieldGroupResource2, i32) = std::mem::transmute(ptr);

        // Redefine ReflectorModule
        let reflector_module = *(module_accessor as *mut *mut u64).add(0x108 / 8);

        set_shield_group2(reflector_module, resource, group_id);
        let count = (*resource).count as i32;
        if count > 0 {
            for x in 0..count {
                ReflectorModule::set_status(module_accessor, x, ShieldStatus(*SHIELD_STATUS_NONE), group_id);
            }
        }
    }
}

extern "C" {
    #[link_name = "\u{1}_ZN3app8lua_bind38FighterManager__get_fighter_entry_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
    fn get_fighter_entry(manager: *mut smash::app::FighterManager, entry_id: u32) -> *mut u8;
}
