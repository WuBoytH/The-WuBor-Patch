use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    crate::{
        vars::*,
        gameplay::*
    }
};

#[inline(always)]
pub unsafe fn mario_fgc(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    let mut special_cancels : Vec<i32> = [].to_vec();
    let mut normal_cancels : Vec<i32> = [].to_vec();
    let mut aerial_cancel = false;
    let mut jump_cancel = 0;
    set_hp(fighter, 105.0);
    if [
        *FIGHTER_STATUS_KIND_ATTACK
    ].contains(&status) {
        special_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
        ].to_vec();
        normal_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3
        ].to_vec();
    }
    else if [
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_AIR
    ].contains(&status) {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_hi") {
            jump_cancel = 1;
        }
        else if status == *FIGHTER_STATUS_KIND_ATTACK_S3 {
            if cancel_exceptions(fighter, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3, true).get_bool() {
                return;
            }
        }
        special_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
        ].to_vec();
        normal_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
        ].to_vec();
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_n") {
            aerial_cancel = true;
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
        special_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
        ].to_vec();
    }
    else if status == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARIO_STATUS_SPECIAL_N_FLAG_FGC_CANCEL) {
            jump_cancel_check_exception(fighter);
        }
    }
    cancel_system(fighter, normal_cancels, special_cancels, aerial_cancel, jump_cancel);
}

#[fighter_frame( agent = FIGHTER_KIND_MARIO )]
fn mario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("attack_air_lw"){
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
                macros::PLAY_SE(fighter, Hash40::new("se_mario_attackair_l02"));
                let speedx = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(fighter.module_accessor);
                macros::SET_SPEED_EX(fighter, speedx, 0.75, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
        }

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF {
            if ![
                *FIGHTER_STATUS_KIND_SPECIAL_LW,
                *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT,
                *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE
            ].contains(&StatusModule::status_kind(fighter.module_accessor)) {
                WorkModule::set_int(fighter.module_accessor, FIGHTER_MARIO_SPECIAL_LW_KIND_LONG_JUMP, FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_KIND);
            }
        }

        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
            mario_fgc(fighter);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        mario_frame
    );
}