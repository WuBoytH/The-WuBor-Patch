use crate::imports::*;

#[skyline::hook(offset = 0x116a3d0)]
pub unsafe extern "C" fn shulk_check_valid_arts_statuses(fighter: *mut Fighter) -> u64 {
    let module_accessor = (*fighter).battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    u64::from([
        *FIGHTER_STATUS_KIND_WAIT,
        *FIGHTER_STATUS_KIND_WALK,
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_RUN,
        *FIGHTER_STATUS_KIND_TURN,
        *FIGHTER_STATUS_KIND_TURN_DASH,
        *FIGHTER_STATUS_KIND_TURN_RUN,
        *FIGHTER_STATUS_KIND_JUMP_SQUAT, // new
        *FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_FLY,
        *FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_FALL_AERIAL,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
        *FIGHTER_STATUS_KIND_GUARD_ON, // new
        *FIGHTER_STATUS_KIND_GUARD, // new
        *FIGHTER_STATUS_KIND_GUARD_OFF, // new
        *FIGHTER_STATUS_KIND_GUARD_DAMAGE, // new
        *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, // new
        *FIGHTER_STATUS_KIND_ATTACK, // new
        *FIGHTER_STATUS_KIND_ATTACK_100, // new
        *FIGHTER_STATUS_KIND_ATTACK_DASH, // new
        *FIGHTER_STATUS_KIND_ATTACK_S3, // new
        *FIGHTER_STATUS_KIND_ATTACK_HI3, // new
        *FIGHTER_STATUS_KIND_ATTACK_LW3, // new
        *FIGHTER_STATUS_KIND_ATTACK_S4_START, // new
        *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, // new
        *FIGHTER_STATUS_KIND_ATTACK_S4, // new
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START, // new
        *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, // new
        *FIGHTER_STATUS_KIND_ATTACK_LW4, // new
        *FIGHTER_STATUS_KIND_ATTACK_HI4_START, // new
        *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, // new
        *FIGHTER_STATUS_KIND_ATTACK_HI4, // new
        *FIGHTER_STATUS_KIND_ATTACK_AIR, // new
        *FIGHTER_STATUS_KIND_CATCH, // new
        *FIGHTER_STATUS_KIND_CATCH_PULL, // new
        *FIGHTER_STATUS_KIND_CATCH_DASH, // new
        *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, // new
        *FIGHTER_STATUS_KIND_CATCH_TURN, // new
        *FIGHTER_STATUS_KIND_CATCH_WAIT, // new
        *FIGHTER_STATUS_KIND_CATCH_ATTACK, // new
        *FIGHTER_STATUS_KIND_CATCH_CUT, // new
        *FIGHTER_STATUS_KIND_CATCH_JUMP, // new
        *FIGHTER_STATUS_KIND_THROW, // new
        // *FIGHTER_STATUS_KIND_DAMAGE,
        // *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_OTTOTTO_WAIT,
        *FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP,
        *FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_ITEM_SCREW_FALL,
        *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP,
        *FIGHTER_STATUS_KIND_ITEM_ROCKETBELT_HOVER_KEEP,
        *FIGHTER_STATUS_KIND_KILLER_JUMP,
        // *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY,
        // *FIGHTER_STATUS_KIND_SAVING_DAMAGE_AIR
    ].contains(&status))
}

#[skyline::hook(offset = 0x116d8a0)]
pub unsafe extern "C" fn shulk_check_can_activate_art_wheel(fighter: *mut Fighter) -> u64 {
    let module_accessor = (*fighter).battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    u64::from([
        *FIGHTER_STATUS_KIND_WAIT,
        *FIGHTER_STATUS_KIND_WALK,
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_RUN,
        *FIGHTER_STATUS_KIND_TURN,
        *FIGHTER_STATUS_KIND_TURN_DASH,
        *FIGHTER_STATUS_KIND_TURN_RUN,
        *FIGHTER_STATUS_KIND_JUMP_SQUAT, // new
        *FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_FLY,
        *FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_FALL_AERIAL,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
        *FIGHTER_STATUS_KIND_GUARD_ON, // new
        *FIGHTER_STATUS_KIND_GUARD, // new
        *FIGHTER_STATUS_KIND_GUARD_OFF, // new
        *FIGHTER_STATUS_KIND_GUARD_DAMAGE, // new
        *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, // new
        *FIGHTER_STATUS_KIND_ATTACK, // new
        *FIGHTER_STATUS_KIND_ATTACK_100, // new
        *FIGHTER_STATUS_KIND_ATTACK_DASH, // new
        *FIGHTER_STATUS_KIND_ATTACK_S3, // new
        *FIGHTER_STATUS_KIND_ATTACK_HI3, // new
        *FIGHTER_STATUS_KIND_ATTACK_LW3, // new
        *FIGHTER_STATUS_KIND_ATTACK_S4_START, // new
        *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, // new
        *FIGHTER_STATUS_KIND_ATTACK_S4, // new
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START, // new
        *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, // new
        *FIGHTER_STATUS_KIND_ATTACK_LW4, // new
        *FIGHTER_STATUS_KIND_ATTACK_HI4_START, // new
        *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, // new
        *FIGHTER_STATUS_KIND_ATTACK_HI4, // new
        *FIGHTER_STATUS_KIND_ATTACK_AIR, // new
        *FIGHTER_STATUS_KIND_CATCH, // new
        *FIGHTER_STATUS_KIND_CATCH_PULL, // new
        *FIGHTER_STATUS_KIND_CATCH_DASH, // new
        *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, // new
        *FIGHTER_STATUS_KIND_CATCH_TURN, // new
        *FIGHTER_STATUS_KIND_CATCH_WAIT, // new
        *FIGHTER_STATUS_KIND_CATCH_ATTACK, // new
        *FIGHTER_STATUS_KIND_CATCH_CUT, // new
        *FIGHTER_STATUS_KIND_CATCH_JUMP, // new
        *FIGHTER_STATUS_KIND_THROW, // new
        // *FIGHTER_STATUS_KIND_DAMAGE,
        // *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        // *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        // *FIGHTER_STATUS_KIND_DOWN,
        // *FIGHTER_STATUS_KIND_DOWN_WAIT,
        // *FIGHTER_STATUS_KIND_DOWN_STAND,
        // *FIGHTER_STATUS_KIND_DOWN_STAND_FB,
        // *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK,
        *FIGHTER_STATUS_KIND_OTTOTTO_WAIT,
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_KIRBY_STATUS_KIND_SHULK_SPECIAL_N
    ].contains(&status))
}

#[skyline::hook(offset = 0x1168360, inline)]
unsafe extern "C" fn shulk_inc_arts_wheel_button_timer(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = *ctx.registers[22].x.as_ref() as *mut BattleObjectModuleAccessor;
    if !WorkModule::is_flag(module_accessor, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CIRCLE_MENU) {
        WorkModule::inc_int(module_accessor, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_SPECIAL_BUTTON_TIMER);
    }
}

// #[skyline::hook(offset = 0x116dbb0)]
// pub unsafe extern "C" fn shulk_on_attack(vtable: u64, fighter: *mut Fighter, log: &mut CollisionLogScuffed) -> u64 {
//     let module_accessor = (*fighter).battle_object.module_accessor;
//     let status = StatusModule::status_kind(module_accessor);
//     if WorkModule::is_flag(module_accessor, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE)
//     && WorkModule::get_int(module_accessor, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE) == *FIGHTER_SHULK_MONAD_TYPE_JUMP {
//         if [
//             *FIGHTER_STATUS_KIND_SPECIAL_S,
//             *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_JUMP,
//             *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_FALL,
//             *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_LANDING
//         ].contains(&status)
//         && log.collision_kind == 1 {
//             let opponent_object = MiscModule::get_battle_object_from_id(log.opponent_object_id);
//             if !opponent_object.is_null()
//             && sv_battle_object::category(log.opponent_object_id) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
//                 VarModule::on_flag((*opponent_object).module_accessor, fighter::instance::flag::PURGED);
//                 VarModule::set_int((*opponent_object).module_accessor, fighter::instance::int::PURGED_TIMER, 300);
//             }
//         }
//     }
//     original!()(vtable, fighter, log)
// }

// #[skyline::hook(offset = 0x1171c90)]
// pub unsafe extern "C" fn shulk_shield_art_hit_decrease(_vtable: u64, _fighter: *mut Fighter, _something: u64) {
//     // nothing lol
// }

pub fn install() {
    // Disables the Monado Arts wheel by redirecting the instructions to skip the arts wheel check.
    // skyline::patching::Patch::in_text(0x1166184).data(0x1400002Au32);

    // Disables a check that causes pushing Special to not bring up the Arts switcher.
    // skyline::patching::Patch::in_text(0x1165864).data(0x37000140u32);

    // Disables a weird check that forces you to go into wait/fall while holding Special, probably for the Art Wheel
    skyline::patching::Patch::in_text(0x1167170).data(0x14000499u32);

    // Disables a check that delays the art being selected by holding Special
    // skyline::patching::Patch::in_text(0x1165dd4).data(0x14000004u32);

    // nops the initial checks that put Shulk into the art wheel animation
    skyline::patching::Patch::in_text(0x11684dc).nop();
    skyline::patching::Patch::in_text(0x11684f0).nop();

    // nops the inc int for the art wheel hold timer so we can run our own logic instead
    skyline::patching::Patch::in_text(0x1168360).nop();

    skyline::install_hooks!(
        shulk_check_valid_arts_statuses,
        shulk_check_can_activate_art_wheel,
        shulk_inc_arts_wheel_button_timer,
        // shulk_on_attack,
        // shulk_shield_art_hit_decrease
    );
}