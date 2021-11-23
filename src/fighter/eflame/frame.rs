use {
    smash::{
        lua2cpp::{L2CFighterCommon, L2CFighterBase},
        hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*,
        gameplay::*,
        table_const::*
    }
};

#[inline(always)]
pub unsafe fn eflame_fgc(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    let mut allowed_cancels : Vec<i32> = [].to_vec();
    set_hp(fighter, 112.0);
    if [
        *FIGHTER_STATUS_KIND_ATTACK
    ].contains(&status) {
        allowed_cancels = [
            *FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_STATUS_KIND_ATTACK_LW3,
            *FIGHTER_STATUS_KIND_ATTACK_HI3
        ].to_vec();
    }
    else if [
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_AIR
    ].contains(&status) {
        if status == *FIGHTER_STATUS_KIND_ATTACK_S3 {
            cancel_exceptions(fighter, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3, true);
        }
        else if status == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            jump_cancel_check_hit(fighter, true);
        }
        allowed_cancels = [
            *FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_STATUS_KIND_SPECIAL_HI
        ].to_vec();
    }
    else if [
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_DASH
    ].contains(&status)
    || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_100_end") {
        allowed_cancels = [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_STATUS_KIND_SPECIAL_HI,
            *FIGHTER_STATUS_KIND_SPECIAL_LW
        ].to_vec();
    }
    cancel_system(fighter, status, allowed_cancels);
}

#[fighter_frame( agent = FIGHTER_KIND_EFLAME )]
fn eflame_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_S
        && MotionModule::frame(fighter.module_accessor) >= 14.0 {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                WorkModule::enable_transition_term(fighter.module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
                WorkModule::enable_transition_term(fighter.module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
                WorkModule::enable_transition_term(fighter.module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
            }
            if ControlModule::check_button_trigger(fighter.module_accessor,*CONTROL_PAD_BUTTON_SPECIAL)
            && CALLBACK[entry_id(fighter.module_accessor)] == 0 {
                CALLBACK[entry_id(fighter.module_accessor)] = 1;
            }
        }
        else {
            CALLBACK[entry_id(fighter.module_accessor)] = 0;
        }

        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY) {
            if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_FINAL
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_EFLAME_STATUS_KIND_FINAL_READY
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_EFLAME_STATUS_KIND_FINAL_SCENE01
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_EFLAME_STATUS_KIND_FINAL_SCENE02
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_EFLAME_STATUS_KIND_FINAL_END
            && is_damage_check(fighter.module_accessor, false) == false {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            }
        }

        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
            eflame_fgc(fighter);
        }
    }
}

#[weapon_frame( agent = WEAPON_KIND_EFLAME_ESWORD )]
fn eflame_esword_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        if CALLBACK[entry_id(weapon.module_accessor)] == 1 {
            if StatusModule::status_kind(weapon.module_accessor) != *WEAPON_EFLAME_ESWORD_STATUS_KIND_SPECIAL_S_ROTATE
            && StatusModule::status_kind(weapon.module_accessor) != *WEAPON_EFLAME_ESWORD_STATUS_KIND_SPECIAL_S_BACK {
                StatusModule::change_status_request_from_script(weapon.module_accessor,*WEAPON_EFLAME_ESWORD_STATUS_KIND_SPECIAL_S_ROTATE,true);
            }
            if StatusModule::status_kind(weapon.module_accessor) == *WEAPON_EFLAME_ESWORD_STATUS_KIND_SPECIAL_S_ROTATE {
                CALLBACK[entry_id(weapon.module_accessor)] = 2;
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        eflame_frame,
        eflame_esword_frame
    );
}