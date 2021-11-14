use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*,
        gameplay::*
    }
};

#[inline(always)]
pub unsafe fn mario_fgc(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    let mut allowed_cancels : Vec<i32> = [].to_vec();
    set_hp(fighter, 105.0);
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
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_AIR
    ].contains(&status) {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_hi") {
            jump_cancel_check_hit(fighter, false);
        }
        else if status == *FIGHTER_STATUS_KIND_ATTACK_S3 {
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
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_n") {
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
    else if status == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARIO_STATUS_SPECIAL_N_FLAG_FGC_CANCEL) {
            jump_cancel_check_exception(fighter);
        }
    }
    cancel_system(fighter, status, allowed_cancels);
}

#[fighter_frame( agent = FIGHTER_KIND_MARIO )]
fn mario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        // if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_DEAD {
        //     IS_BONKER[entry_id(fighter.module_accessor)] = 0;
        // }

        if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("attack_air_lw"){
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
                macros::PLAY_SE(fighter, Hash40::new("se_mario_attackair_l02"));
                let speedx = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(fighter.module_accessor);
                macros::SET_SPEED_EX(fighter, speedx, 0.75, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
        }

        if [
            hash40("attack_air_f"),
            hash40("landing_air_f"),
            hash40("attack_s4_s"),
            hash40("attack_s4_hi"),
            hash40("attack_s4_lw"),
            hash40("attack_s4_hold")
        ].contains(&MotionModule::motion_kind(fighter.module_accessor)) == false {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP) {
                ArticleModule::remove(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            }
        }

        // if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
        //     if IS_BONKER[entry_id(fighter.module_accessor)] == 4 {
        //         IS_BONKER[entry_id(fighter.module_accessor)] = 0;
        //     }
        //     else {
        //         IS_BONKER[entry_id(fighter.module_accessor)] = 4;
        //     }
        // }

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

        if IS_FGC[entry_id(fighter.module_accessor)] {
            mario_fgc(fighter);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        mario_frame
    );
}