use {
    smash::{
        lua2cpp::L2CFighterCommon,
        // hash40,
        // lib::lua_const::*
    },
    custom_var::*,
    wubor_utils::{
        wua_bind::*,
        vars::*,
        // table_const::*
    }
};

pub unsafe extern "C" fn dolly_fgc(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object, fighter::instance::flag::IS_FGC) {
        // let status = fighter.global_table[STATUS_KIND].get_i32();
        // let mut special_cancels : Vec<i32> = [].to_vec();
        // let mut normal_cancels : Vec<i32> = [].to_vec();
        // let mut jump_cancel = 0;
        MiscModule::set_hp(fighter, 180.0);
        // if [
        //     *FIGHTER_STATUS_KIND_ATTACK,
        //     *FIGHTER_STATUS_KIND_ATTACK_DASH
        // ].contains(&status) {
        //     special_cancels = [
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
        //     ].to_vec();
        //     normal_cancels = [
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3,
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
        //     ].to_vec();
        // }
        // else if [
        //     *FIGHTER_STATUS_KIND_ATTACK_S3,
        //     *FIGHTER_STATUS_KIND_ATTACK_LW3,
        //     *FIGHTER_STATUS_KIND_ATTACK_HI3,
        //     *FIGHTER_STATUS_KIND_ATTACK_AIR
        // ].contains(&status) {
        //     if status == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        //         jump_cancel = 2;
        //     }
        //     special_cancels = [
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
        //     ].to_vec();
        //     normal_cancels = [
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
        //     ].to_vec();
        // }
        // else if [
        //     *FIGHTER_STATUS_KIND_ATTACK_S4,
        //     *FIGHTER_STATUS_KIND_ATTACK_LW4,
        //     *FIGHTER_STATUS_KIND_ATTACK_HI4
        // ].contains(&status) {
        //     if status == *FIGHTER_STATUS_KIND_ATTACK_HI4
        //     || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_s4_hi") {
        //         jump_cancel = 1;
        //     }
        //     special_cancels = [
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        //         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
        //     ].to_vec();
        // }
        // FGCModule::cancel_system(fighter, normal_cancels, special_cancels, false, jump_cancel);
    }
}
