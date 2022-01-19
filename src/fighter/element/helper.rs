use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    wubor_utils::wua_bind::*
};

#[inline(always)]
pub unsafe fn element_fgc(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    let mut special_cancels : Vec<i32> = [].to_vec();
    let mut normal_cancels : Vec<i32> = [].to_vec();
    let mut jump_cancel = 0;
    MiscModule::set_hp(fighter, 112.0);
    if [
        *FIGHTER_STATUS_KIND_ATTACK
    ].contains(&status) {
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
        if status == *FIGHTER_STATUS_KIND_ATTACK_S3 {
            if FGCModule::cancel_exceptions(fighter, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3, true).get_bool() {
                return;
            }
        }
        else if status == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            jump_cancel = 2;
        }
        special_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI
        ].to_vec();
        normal_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
        ].to_vec();
    }
    else if [
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_DASH
    ].contains(&status)
    || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_100_end") {
        special_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
        ].to_vec();
    }
    FGCModule::cancel_system(fighter, normal_cancels, special_cancels, false, jump_cancel);
}
