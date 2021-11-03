use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*,
        gameplay::*
    }
};

// #[inline(always)]
// pub unsafe fn dolly_fgc(fighter: &mut L2CFighterCommon) {
//     let status = StatusModule::status_kind(fighter.module_accessor);
//     let mut allowed_cancels : Vec<i32> = [].to_vec();
//     set_hp(fighter, 90.0);
//     if [
//         *FIGHTER_STATUS_KIND_ATTACK
//     ].contains(&status) {
//         allowed_cancels = [
//             *FIGHTER_STATUS_KIND_ATTACK_S3,
//             *FIGHTER_STATUS_KIND_ATTACK_LW3,
//             *FIGHTER_STATUS_KIND_ATTACK_HI3,
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
//         *FIGHTER_STATUS_KIND_ATTACK_DASH,
//         *FIGHTER_STATUS_KIND_ATTACK_AIR
//     ].contains(&status) {
//         if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_hi") {
//             jump_cancel_check_hit(fighter, false);
//         }
//         else if status == *FIGHTER_STATUS_KIND_ATTACK_S3 {
//             cancel_exceptions(fighter, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3, true);
//         }
//         allowed_cancels = [
//             *FIGHTER_STATUS_KIND_ATTACK_S4,
//             *FIGHTER_STATUS_KIND_ATTACK_HI4,
//             *FIGHTER_STATUS_KIND_ATTACK_LW4,
//             *FIGHTER_STATUS_KIND_SPECIAL_N,
//             *FIGHTER_STATUS_KIND_SPECIAL_S,
//             *FIGHTER_STATUS_KIND_SPECIAL_LW,
//             *FIGHTER_STATUS_KIND_SPECIAL_HI
//         ].to_vec();
//         if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_n") {
//             allowed_cancels.append(&mut [*FIGHTER_STATUS_KIND_ATTACK_AIR].to_vec());
//         }
//     }
//     else if [
//         *FIGHTER_STATUS_KIND_ATTACK_S4,
//         *FIGHTER_STATUS_KIND_ATTACK_LW4,
//         *FIGHTER_STATUS_KIND_ATTACK_HI4
//     ].contains(&status) {
//         if status == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
//             jump_cancel_check_hit(fighter, false);
//         }
//         allowed_cancels = [
//             *FIGHTER_STATUS_KIND_SPECIAL_N,
//             *FIGHTER_STATUS_KIND_SPECIAL_S,
//             *FIGHTER_STATUS_KIND_SPECIAL_LW,
//             *FIGHTER_STATUS_KIND_SPECIAL_HI
//         ].to_vec();
//     }
//     else if status == *FIGHTER_STATUS_KIND_SPECIAL_N {
//         if FIREBALL_CANCEL[entry_id(fighter.module_accessor)] {
//             jump_cancel_check_exception(fighter);
//         }
//     }
//     cancel_system(fighter, status, allowed_cancels);
// }

#[fighter_frame( agent = FIGHTER_KIND_DOLLY )]
fn dolly_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
            if DTILT_CHAIN[entry_id(fighter.module_accessor)] == 0 {
                chain_cancels(fighter, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3, true, &mut DTILT_CHAIN);
            }
        }
        // if IS_FGC[entry_id(fighter.module_accessor)] {
        //     dolly_fgc(fighter);
        // }
    }
}

pub fn install() {
    install_agent_frames!(
        dolly_frame
    );
}