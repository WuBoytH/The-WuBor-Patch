use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    wubor_utils::{
        wua_bind::*,
        vars::*
    }
};

pub unsafe extern "C" fn bayonetta_fgc(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    let mut special_cancels : Vec<i32> = [].to_vec();
    let mut normal_cancels : Vec<i32> = [].to_vec();
    let mut aerial_cancel = false;
    let mut jump_cancel = 0;
    if [
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_S_HOLD_END
    ].contains(&status)
    || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_100_end") {
        if FGCModule::dash_cancel_check(fighter, false, false).get_bool() {
            return;
        }
    }
    if [
        hash40("attack_s3_s2"),
        hash40("attack_s3_s3")
    ].contains(&MotionModule::motion_kind(fighter.module_accessor)) {
        jump_cancel = 1;
    }
    if [
        *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U
    ].contains(&status) {
        if FGCModule::air_dash_cancel_check(fighter, false).get_bool() {
            return;
        }
    }
    if [
        *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP
    ].contains(&status)
    && MotionModule::frame(fighter.module_accessor) > 31.0 {
        if FGCModule::air_dash_cancel_check(fighter, false).get_bool() {
            return;
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
        MiscModule::set_hp(fighter, 132.0);
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
            *FIGHTER_STATUS_KIND_ATTACK_DASH
        ].contains(&status) {
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
                jump_cancel = 1;
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
        }
        else if [
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F
        ].contains(&status) {
            jump_cancel = 1;
            aerial_cancel = true;
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_STATUS_WORK_ID_FLAG_NORMAL_CANCEL);
            let mot = MotionModule::motion_kind(fighter.module_accessor);
            let flags = ATTACK_AIR_N_MASK + ATTACK_AIR_F_MASK + ATTACK_AIR_B_MASK + ATTACK_AIR_HI_MASK + ATTACK_AIR_LW_MASK;
            if mot == hash40("attack_air_n") {
                FGCModule::disable_aerial(fighter, ATTACK_AIR_N_MASK);
            }
            else if mot == hash40("attack_air_f") {
                FGCModule::disable_aerial(fighter, ATTACK_AIR_F_MASK);
            }
            else if mot == hash40("attack_air_b") {
                FGCModule::disable_aerial(fighter, ATTACK_AIR_B_MASK);
            }
            else if mot == hash40("attack_air_hi") {
                FGCModule::disable_aerial(fighter, ATTACK_AIR_HI_MASK);
            }
            else if mot == hash40("attack_air_lw") {
                FGCModule::disable_aerial(fighter, ATTACK_AIR_LW_MASK);
            }
            WorkModule::set_int(fighter.module_accessor, flags, FIGHTER_STATUS_WORK_ID_INT_ENABLED_AERIALS);
            special_cancels = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL
            ].to_vec();
        }
        else if [
            *FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4
        ].contains(&status) {
            special_cancels = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
            ].to_vec();
        }
    }
    FGCModule::cancel_system(fighter, normal_cancels, special_cancels, aerial_cancel, jump_cancel);
}
