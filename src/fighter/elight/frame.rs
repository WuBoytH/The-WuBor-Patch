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
        gameplay::*,
        table_const::*
    }
};

#[inline(always)]
pub unsafe fn elight_fgc(fighter: &mut L2CFighterCommon) {
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
    else if status == *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD {
        if MotionModule::frame(fighter.module_accessor) >= 11.0
        && MotionModule::frame(fighter.module_accessor) < 32.0 {
            if ControlModule::check_button_on(fighter.module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END, true);
                CANCEL[entry_id(fighter.module_accessor)] = true;
            }
        }
        else {
            CANCEL[entry_id(fighter.module_accessor)] = false;
        }
    }
    cancel_system(fighter, status, allowed_cancels);
}

#[fighter_frame( agent = FIGHTER_KIND_ELIGHT )]
fn elight_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY) {
            if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_FINAL
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_ELIGHT_STATUS_KIND_FINAL_READY
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_ELIGHT_STATUS_KIND_FINAL_SCENE01
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_ELIGHT_STATUS_KIND_FINAL_SCENE02
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_ELIGHT_STATUS_KIND_FINAL_END
            && is_damage_check(fighter.module_accessor, false) == false {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            }
        }
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_hi_jump") {
            if PostureModule::lr(fighter.module_accessor) == 1.0 && ControlModule::get_stick_x(fighter.module_accessor) < -0.75 {
                PostureModule::reverse_lr(fighter.module_accessor);
            }
            else if PostureModule::lr(fighter.module_accessor) == -1.0 && ControlModule::get_stick_x(fighter.module_accessor) > 0.75 {
                PostureModule::reverse_lr(fighter.module_accessor);
            }
        }

        if CANCEL[entry_id(fighter.module_accessor)] == true && StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END {
            CANCEL[entry_id(fighter.module_accessor)] = false;
            MotionModule::set_frame(fighter.module_accessor, 25.0, false);
        }

        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
            elight_fgc(fighter);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        elight_frame
    );
}