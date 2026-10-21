use crate::imports::*;
use wubor_utils::offsets;

#[skyline::hook(offset = offsets::fighter::shulk::CHECK_VALID_ARTS_STATUSES)]
pub unsafe extern "C" fn shulk_check_valid_arts_statuses(fighter: *mut Fighter) -> u64 {
    let module_accessor = (*fighter).battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    u64::from([
        *FIGHTER_STATUS_KIND_WAIT,
        *FIGHTER_STATUS_KIND_WALK,
        *FIGHTER_STATUS_KIND_WALK_BRAKE, // new
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_RUN,
        *FIGHTER_STATUS_KIND_RUN_BRAKE, // new
        *FIGHTER_STATUS_KIND_TURN,
        *FIGHTER_STATUS_KIND_TURN_DASH,
        *FIGHTER_STATUS_KIND_TURN_RUN,
        *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE, // new
        *FIGHTER_STATUS_KIND_JUMP_SQUAT, // new
        *FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_FLY,
        *FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_FALL_AERIAL,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
        *FIGHTER_STATUS_KIND_GUARD_ON, // new
        *FIGHTER_STATUS_KIND_GUARD, // new
        *FIGHTER_STATUS_KIND_GUARD_OFF, // new
        *FIGHTER_STATUS_KIND_GUARD_DAMAGE, // new
        *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, // new
        *FIGHTER_STATUS_KIND_ATTACK, // new
        *FIGHTER_STATUS_KIND_ATTACK_100, // new
        *FIGHTER_STATUS_KIND_ATTACK_DASH, // new
        *FIGHTER_STATUS_KIND_ATTACK_S3, // new
        *FIGHTER_STATUS_KIND_ATTACK_HI3, // new
        *FIGHTER_STATUS_KIND_ATTACK_LW3, // new
        *FIGHTER_STATUS_KIND_ATTACK_S4_START, // new
        *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, // new
        *FIGHTER_STATUS_KIND_ATTACK_S4, // new
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START, // new
        *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, // new
        *FIGHTER_STATUS_KIND_ATTACK_LW4, // new
        *FIGHTER_STATUS_KIND_ATTACK_HI4_START, // new
        *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, // new
        *FIGHTER_STATUS_KIND_ATTACK_HI4, // new
        *FIGHTER_STATUS_KIND_ATTACK_AIR, // new
        *FIGHTER_STATUS_KIND_CATCH, // new
        *FIGHTER_STATUS_KIND_CATCH_PULL, // new
        *FIGHTER_STATUS_KIND_CATCH_DASH, // new
        *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, // new
        *FIGHTER_STATUS_KIND_CATCH_TURN, // new
        *FIGHTER_STATUS_KIND_CATCH_WAIT, // new
        *FIGHTER_STATUS_KIND_CATCH_ATTACK, // new
        *FIGHTER_STATUS_KIND_CATCH_CUT, // new
        *FIGHTER_STATUS_KIND_CATCH_JUMP, // new
        *FIGHTER_STATUS_KIND_THROW, // new
        // *FIGHTER_STATUS_KIND_DAMAGE,
        // *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_OTTOTTO_WAIT,
        *FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP,
        *FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_ITEM_SCREW_FALL,
        *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP,
        *FIGHTER_STATUS_KIND_ITEM_ROCKETBELT_HOVER_KEEP,
        *FIGHTER_STATUS_KIND_KILLER_JUMP,
        // *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY,
        // *FIGHTER_STATUS_KIND_SAVING_DAMAGE_AIR
    ].contains(&status))
}

#[skyline::hook(offset = offsets::fighter::shulk::CHECK_CAN_ACTIVATE_ART_WHEEL)]
pub unsafe extern "C" fn shulk_check_can_activate_art_wheel(fighter: *mut Fighter) -> u64 {
    let module_accessor = (*fighter).battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    u64::from([
        *FIGHTER_STATUS_KIND_WAIT,
        *FIGHTER_STATUS_KIND_WALK,
        *FIGHTER_STATUS_KIND_WALK_BRAKE, // new
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_RUN,
        *FIGHTER_STATUS_KIND_RUN_BRAKE, // new
        *FIGHTER_STATUS_KIND_TURN,
        *FIGHTER_STATUS_KIND_TURN_DASH,
        *FIGHTER_STATUS_KIND_TURN_RUN,
        *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE, // new
        *FIGHTER_STATUS_KIND_JUMP_SQUAT, // new
        *FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_FLY,
        *FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_FALL_AERIAL,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
        *FIGHTER_STATUS_KIND_GUARD_ON, // new
        *FIGHTER_STATUS_KIND_GUARD, // new
        *FIGHTER_STATUS_KIND_GUARD_OFF, // new
        *FIGHTER_STATUS_KIND_GUARD_DAMAGE, // new
        *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, // new
        *FIGHTER_STATUS_KIND_ATTACK, // new
        *FIGHTER_STATUS_KIND_ATTACK_100, // new
        *FIGHTER_STATUS_KIND_ATTACK_DASH, // new
        *FIGHTER_STATUS_KIND_ATTACK_S3, // new
        *FIGHTER_STATUS_KIND_ATTACK_HI3, // new
        *FIGHTER_STATUS_KIND_ATTACK_LW3, // new
        *FIGHTER_STATUS_KIND_ATTACK_S4_START, // new
        *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, // new
        *FIGHTER_STATUS_KIND_ATTACK_S4, // new
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START, // new
        *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, // new
        *FIGHTER_STATUS_KIND_ATTACK_LW4, // new
        *FIGHTER_STATUS_KIND_ATTACK_HI4_START, // new
        *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, // new
        *FIGHTER_STATUS_KIND_ATTACK_HI4, // new
        *FIGHTER_STATUS_KIND_ATTACK_AIR, // new
        *FIGHTER_STATUS_KIND_CATCH, // new
        *FIGHTER_STATUS_KIND_CATCH_PULL, // new
        *FIGHTER_STATUS_KIND_CATCH_DASH, // new
        *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, // new
        *FIGHTER_STATUS_KIND_CATCH_TURN, // new
        *FIGHTER_STATUS_KIND_CATCH_WAIT, // new
        *FIGHTER_STATUS_KIND_CATCH_ATTACK, // new
        *FIGHTER_STATUS_KIND_CATCH_CUT, // new
        *FIGHTER_STATUS_KIND_CATCH_JUMP, // new
        *FIGHTER_STATUS_KIND_THROW, // new
        // *FIGHTER_STATUS_KIND_DAMAGE,
        // *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        // *FIGHTER_STATUS_KIND_DOWN,
        // *FIGHTER_STATUS_KIND_DOWN_WAIT,
        // *FIGHTER_STATUS_KIND_DOWN_STAND,
        // *FIGHTER_STATUS_KIND_DOWN_STAND_FB,
        // *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK,
        *FIGHTER_STATUS_KIND_OTTOTTO_WAIT,
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_KIRBY_STATUS_KIND_SHULK_SPECIAL_N
    ].contains(&status))
}

#[skyline::hook(offset = offsets::fighter::shulk::CAN_TRANSITION_TO_SPECIAL_N)]
pub unsafe extern "C" fn shulk_can_transition_to_special_n(fighter: *mut Fighter) -> u64 {
    let module_accessor = (*fighter).battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    u64::from([
        *FIGHTER_STATUS_KIND_WAIT,
        *FIGHTER_STATUS_KIND_WALK,
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_RUN,
        *FIGHTER_STATUS_KIND_RUN_BRAKE, // new
        *FIGHTER_STATUS_KIND_TURN, // new
        *FIGHTER_STATUS_KIND_TURN_DASH, // new
        *FIGHTER_STATUS_KIND_TURN_RUN, // new
        *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE, // new
        *FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_FLY,
        *FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_FALL_AERIAL,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
        // *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
        // *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
        *FIGHTER_STATUS_KIND_OTTOTTO_WAIT,
        *FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP,
        *FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_ITEM_SCREW_FALL,
        *FIGHTER_STATUS_KIND_ITEM_SCREW_FALL,
        *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP,
        *FIGHTER_STATUS_KIND_ITEM_ROCKETBELT_HOVER_KEEP,
        *FIGHTER_STATUS_KIND_KILLER_JUMP,
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_KIRBY_STATUS_KIND_SHULK_SPECIAL_N
    ].contains(&status))
}

extern "C" {
    #[link_name = "shulk_inc_arts_wheel_button_timer_inline"]
    fn shulk_inc_arts_wheel_button_timer_inline(ctx: &mut skyline::hooks::InlineCtx);
}

#[skyline::hook(offset = offsets::fighter::shulk::INC_ARTS_WHEEL_BUTTON_TIMER, inline)]
unsafe extern "C" fn shulk_inc_arts_wheel_button_timer(ctx: &mut skyline::hooks::InlineCtx) {
    shulk_inc_arts_wheel_button_timer_inline(ctx);
}

// #[skyline::hook(offset = 0x1171c90)]
// pub unsafe extern "C" fn shulk_shield_art_hit_decrease(_vtable: u64, _fighter: *mut Fighter, _something: u64) {
//     // nothing lol
// }

pub fn install() {
    skyline::install_hooks!(
        shulk_check_valid_arts_statuses,
        shulk_check_can_activate_art_wheel,
        shulk_can_transition_to_special_n,
        shulk_inc_arts_wheel_button_timer,
        // shulk_shield_art_hit_decrease
    );
}