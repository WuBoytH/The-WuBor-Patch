use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

// #[inline(always)]
// pub unsafe fn captain_fgc(fighter: &mut L2CFighterCommon) {
//     let status = StatusModule::status_kind(fighter.module_accessor);
//     let mut allowed_cancels : Vec<i32> = [].to_vec();
//     set_hp(fighter, 90.0);
//     if [
//         *FIGHTER_STATUS_KIND_ATTACK,
//         *FIGHTER_STATUS_KIND_ATTACK_DASH
//     ].contains(&status) {
//         allowed_cancels = [
//             *FIGHTER_STATUS_KIND_ATTACK_S3,
//             *FIGHTER_STATUS_KIND_ATTACK_LW3,
//             *FIGHTER_STATUS_KIND_ATTACK_HI3,
//             *FIGHTER_STATUS_KIND_ATTACK_LW4,
//             *FIGHTER_STATUS_KIND_SPECIAL_N,
//             *FIGHTER_STATUS_KIND_SPECIAL_S,
//             *FIGHTER_STATUS_KIND_SPECIAL_LW,
//             *FIGHTER_STATUS_KIND_SPECIAL_HI
//         ].to_vec();
//     }
//     else if [
//         *FIGHTER_STATUS_KIND_ATTACK_S3,
//         *FIGHTER_STATUS_KIND_ATTACK_LW3,
//         *FIGHTER_STATUS_KIND_ATTACK_HI3,
//         *FIGHTER_STATUS_KIND_ATTACK_AIR
//     ].contains(&status) {
//         allowed_cancels = [
//             *FIGHTER_STATUS_KIND_ATTACK_LW4,
//             *FIGHTER_STATUS_KIND_SPECIAL_N,
//             *FIGHTER_STATUS_KIND_SPECIAL_S,
//             *FIGHTER_STATUS_KIND_SPECIAL_LW,
//             *FIGHTER_STATUS_KIND_SPECIAL_HI
//         ].to_vec();
//     }
//     if [
//         *FIGHTER_STATUS_KIND_ATTACK_HI3,
//         *FIGHTER_STATUS_KIND_ATTACK_LW4
//     ].contains(&status)
//     || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_b")
//     || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_hi") {
//         jump_cancel_check_hit(fighter, false);
//     }
//     if status == *FIGHTER_STATUS_KIND_ATTACK_S4 {
//         dash_cancel_check(fighter, false, false);
//     }
//     if status == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
//         dash_cancel_check(fighter, false, true);
//     }
//     cancel_system(fighter, status, allowed_cancels);
// }

// #[fighter_frame( agent = FIGHTER_KIND_CAPTAIN )]
// fn captain_frame(fighter: &mut L2CFighterCommon) {
//     unsafe {
        
//     }
// }

// #[weapon_frame( agent = WEAPON_KIND_CAPTAIN_FALCONPUNCH )]
// fn captain_falconpunch_frame(weapon: &mut L2CFighterBase) {
//     unsafe {
        
//     }
// }

pub fn install() {
    // install_agent_frames!(
    //     captain_frame,
    //     captain_falconpunch_frame
    // );
}