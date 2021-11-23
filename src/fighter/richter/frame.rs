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
pub unsafe fn richter_fgc(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    let mut allowed_cancels : Vec<i32> = [].to_vec();
    set_hp(fighter, 102.0);
    if [
        *FIGHTER_STATUS_KIND_ATTACK
    ].contains(&status)
    || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_100_end") {
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
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_AIR
    ].contains(&status) {
        if status == *FIGHTER_STATUS_KIND_ATTACK_S3 {
            cancel_exceptions(fighter, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3, true);
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
        if status == *FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32 {
            allowed_cancels.append(&mut [*FIGHTER_STATUS_KIND_ATTACK_AIR].to_vec());
        }
    }
    else if [
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4
    ].contains(&status) {
        if status == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
            jump_cancel_check_hit(fighter, false);
        }
        allowed_cancels = [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_STATUS_KIND_SPECIAL_HI
        ].to_vec();
    }
    else if status == *FIGHTER_STATUS_KIND_SPECIAL_HI
    && MotionModule::frame(fighter.module_accessor) > 22.0 {
        jump_cancel_check_hit(fighter, false);
    }
    cancel_system(fighter, status, allowed_cancels);
}

#[fighter_frame( agent = FIGHTER_KIND_RICHTER )]
fn richter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI)
        && (StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH
        || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        || is_damage_check(fighter.module_accessor, false)) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI);
        }
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
            richter_fgc(fighter);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        richter_frame
    );
}