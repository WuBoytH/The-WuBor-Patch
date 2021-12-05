use {
    smash::{
        lua2cpp::{L2CFighterCommon, L2CFighterBase},
        hash40,
        phx::Vector3f,
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
pub unsafe fn samusd_fgc(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    let mut special_cancels : Vec<i32> = [].to_vec();
    let mut normal_cancels : Vec<i32> = [].to_vec();
    let mut jump_cancel = 0;
    set_hp(fighter, 110.0);
    if [
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_DASH
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
        *FIGHTER_STATUS_KIND_ATTACK_AIR
    ].contains(&status) {
        special_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
        ].to_vec();
        normal_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START
        ].to_vec();
    }
    else if [
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW4
    ].contains(&status)
    || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_b")
    || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_hi") {
        jump_cancel = 1;
    }
    else if status == *FIGHTER_STATUS_KIND_ATTACK_S4 {
        if dash_cancel_check(fighter, false, false).get_bool() {
            return;
        }
    }
    else if status == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
        if dash_cancel_check(fighter, false, true).get_bool() {
            return;
        }
    }
    cancel_system(fighter, normal_cancels, special_cancels, false, jump_cancel);
}

#[fighter_frame( agent = FIGHTER_KIND_SAMUSD )]
fn samusd_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
    
        // Morph Ball Drop Bounce
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_lw")
        || MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw") {
            if (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))
            && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_SAMUSD_STATUS_SPECIAL_LW_FLAG_BOUNCE) {
                MotionModule::set_frame_sync_anim_cmd(
                    fighter.module_accessor,
                    44.0,
                    true,
                    true,
                    false
                );
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0,y: 0.5,z: 0.0});
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_SAMUSD_STATUS_SPECIAL_LW_FLAG_BOUNCE);
            }
        }

        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
            samusd_fgc(fighter);
        }
    }
}

#[weapon_frame( agent = WEAPON_KIND_SAMUSD_CSHOT )]
fn samusd_cshot_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        if MotionModule::motion_kind(weapon.module_accessor) == smash::hash40("shoot") {
            let slowdownvec : Vector3f = Vector3f{x: 0.9,y: 0.0,z: 0.0};
            KineticModule::mul_speed(weapon.module_accessor, &slowdownvec, *WEAPON_KINETIC_TYPE_NONE);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        samusd_frame,
        samusd_cshot_frame
    );
}