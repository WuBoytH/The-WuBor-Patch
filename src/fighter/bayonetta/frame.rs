use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*,
        gameplay::*
    }
};

#[inline(always)]
pub unsafe fn bayonetta_fgc(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    let mut allowed_cancels : Vec<i32> = [].to_vec();
    set_hp(fighter, 132.0);
    if [
        *FIGHTER_STATUS_KIND_ATTACK
    ].contains(&status) {
        allowed_cancels = [
            *FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_STATUS_KIND_ATTACK_LW3,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_STATUS_KIND_SPECIAL_HI
        ].to_vec();
    }
    else if [
        hash40("attack_100_end"),
        hash40("special_s_hold_end")
    ].contains(&MotionModule::motion_kind(fighter.module_accessor)) {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            cancel_exceptions(fighter, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F, false);
        }
    }
    else if [
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F
    ].contains(&status) {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
            jump_cancel_check_hit(fighter, false);
        }
        else if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                cancel_exceptions(fighter, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F, false);
            }
        }
        allowed_cancels = [
            *FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_STATUS_KIND_SPECIAL_HI
        ].to_vec();
    }
    else if [
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4
    ].contains(&status) {
        allowed_cancels = [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_STATUS_KIND_SPECIAL_HI
        ].to_vec();
    }
    cancel_system(fighter, status, allowed_cancels);
}

#[fighter_frame( agent = FIGHTER_KIND_BAYONETTA )]
fn bayonetta_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if [
            hash40("attack_lw3"),
            hash40("attack_100_end"),
            hash40("special_s_hold_end")
        ].contains(&MotionModule::motion_kind(fighter.module_accessor)) {
            dash_cancel_check(fighter, false, false);
        }

        if [hash40("attack_s3_s2"),
            hash40("attack_s3_s3")
        ].contains(&MotionModule::motion_kind(fighter.module_accessor)) {
            jump_cancel_check_hit(fighter, false);
        }

        else if [hash40("attack_air_f3")
        ].contains(&MotionModule::motion_kind(fighter.module_accessor))
        && MotionModule::frame(fighter.module_accessor) < 18.0 {
            cancel_exceptions(fighter, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S, true);
        }

        else if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_DASH {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_100, false);
            }
        }

        if IS_FGC[entry_id(fighter.module_accessor)] {
            bayonetta_fgc(fighter);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        bayonetta_frame
    );
}