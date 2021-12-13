use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::{lua_bind::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    crate::{
        vars::*,
        table_const::*,
        gameplay::*
    }
};

#[inline(always)]
pub unsafe fn toonlink_fgc(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    let mut special_cancels : Vec<i32> = [].to_vec();
    let mut normal_cancels : Vec<i32> = [].to_vec();
    let mut aerial_cancel = false;
    let mut jump_cancel = 0;
    set_hp(fighter, 118.0);
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
            if cancel_exceptions(
                fighter,
                *FIGHTER_STATUS_KIND_ATTACK_DASH,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3,
                true
            ).get_bool() {
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
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
            aerial_cancel = true;
        }
    }
    else if [
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4
    ].contains(&status) {
        if status == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
            jump_cancel = 1;
        }
        special_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
        ].to_vec();
    }
    cancel_system(fighter, normal_cancels, special_cancels, aerial_cancel, jump_cancel);
}

#[fighter_frame( agent = FIGHTER_KIND_TOONLINK )]
fn toonlink_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        // Down Air Bounce

        if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("attack_air_lw"){
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_TOONLINK_STATUS_ATTACK_AIR_LW_FLAG_BOUNCE);
            }
            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_TOONLINK_STATUS_ATTACK_AIR_LW_FLAG_BOUNCE) {
                macros::SET_SPEED_EX(fighter, 0.0, 0.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                KineticModule::suspend_energy_all(fighter.module_accessor);
                if !fighter.global_table[IN_HITLAG].get_bool()
                && MotionModule::frame(fighter.module_accessor) < 65.0 {
                    MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 65.0, true, true, false);
                }
                else if MotionModule::frame(fighter.module_accessor) > 66.0 {
                    KineticModule::resume_energy_all(fighter.module_accessor);
                    WorkModule::off_flag(fighter.module_accessor, FIGHTER_TOONLINK_STATUS_ATTACK_AIR_LW_FLAG_BOUNCE);
                    MotionModule::set_rate(fighter.module_accessor, 0.4);
                }
            }
        }

        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
            toonlink_fgc(fighter);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        toonlink_frame
    );
}