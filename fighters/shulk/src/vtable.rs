use super::*;
use wubor_utils::offsets;

#[skyline::from_offset(offsets::fighter::shulk::ON_ATTACK)]
unsafe extern "C" fn shulk_on_attack_inner(vtable: u64, fighter: *mut Fighter, log: &mut CollisionLogScuffed, damage: f32) -> u64;

// Inline hooked in the main plugin (src/vtable_hook/shulk.rs).
#[no_mangle]
pub unsafe extern "C" fn shulk_inc_arts_wheel_button_timer_inline(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = ctx.registers[22].x() as *mut BattleObjectModuleAccessor;
    if !WorkModule::is_flag(module_accessor, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CIRCLE_MENU) {
        WorkModule::inc_int(module_accessor, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_SPECIAL_BUTTON_TIMER);
    }
}

pub unsafe extern "C" fn shulk_on_attack(vtable: u64, fighter: *mut Fighter, log: &mut CollisionLogScuffed, damage: f32) -> u64 {
    let module_accessor = (*fighter).battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    // if WorkModule::is_flag(module_accessor, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE)
    // && WorkModule::get_int(module_accessor, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE) == *FIGHTER_SHULK_MONAD_TYPE_JUMP {
    //     if [
    //         *FIGHTER_STATUS_KIND_SPECIAL_S,
    //         *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_JUMP,
    //         *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_FALL,
    //         *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_LANDING
    //     ].contains(&status)
    //     && log.collision_kind == 1 {
    //         let opponent_object = MiscModule::get_battle_object_from_id(log.opponent_object_id);
    //         if !opponent_object.is_null()
    //         && sv_battle_object::category(log.opponent_object_id) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
    //             VarModule::on_flag((*opponent_object).module_accessor, vars::fighter::instance::flag::PURGED);
    //             VarModule::set_int((*opponent_object).module_accessor, vars::fighter::instance::int::PURGED_TIMER, 300);
    //         }
    //     }
    // }
    if ![
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_STATUS_KIND_FINAL,
        *FIGHTER_SHULK_STATUS_KIND_SPECIAL_N_ACTION,
        *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_JUMP,
        *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_FALL,
        *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_LANDING,
        *FIGHTER_SHULK_STATUS_KIND_SPECIAL_HI_ADD,
        *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_N,
        *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_ATTACK,
    ].contains(&status)
    && log.collision_kind == 1 {
        for art in 0..5 {
            WorkModule::count_down_int(module_accessor, 0x100000C9 + art, 0);
        }
    }
    shulk_on_attack_inner(vtable, fighter, log, damage)
}

pub fn install() {
    unsafe {
        // Disables the Monado Arts wheel by redirecting the instructions to skip the arts wheel check.
        // let _ = skyline::patching::Patch::in_text(0x1166184).data(0x1400002Au32);

        // Disables a check that causes pushing Special to not bring up the Arts switcher.
        // let _ = skyline::patching::Patch::in_text(0x1165864).data(0x37000140u32);

        // Disables a weird check that forces you to go into wait/fall while holding Special, probably for the Art Wheel
        let _ = skyline::patching::Patch::in_text(offsets::fighter::shulk::SPECIAL_HOLD_WAIT_FALL_PATCH).data(0x14000499u32);

        // Disables a check that delays the art being selected by holding Special
        // let _ = skyline::patching::Patch::in_text(0x1165dd4).data(0x14000004u32);

        // nops the initial checks that put Shulk into the art wheel animation
        let _ = skyline::patching::Patch::in_text(offsets::fighter::shulk::ART_WHEEL_ANIMATION_PATCH_1).nop();
        let _ = skyline::patching::Patch::in_text(offsets::fighter::shulk::ART_WHEEL_ANIMATION_PATCH_2).nop();

        // nops the inc int for the art wheel hold timer so we can run our own logic instead
        let _ = skyline::patching::Patch::in_text(offsets::fighter::shulk::INC_ARTS_WHEEL_BUTTON_TIMER).nop();

        // nops multiplying the params for art cooldown by 60
        let _ = skyline::patching::Patch::in_text(offsets::fighter::shulk::ART_COOLDOWN_MULTIPLY_PATCH_1).nop();
        let _ = skyline::patching::Patch::in_text(offsets::fighter::shulk::ART_COOLDOWN_MULTIPLY_PATCH_2).nop();
        let _ = skyline::patching::Patch::in_text(offsets::fighter::shulk::ART_COOLDOWN_MULTIPLY_PATCH_3).nop();
        let _ = skyline::patching::Patch::in_text(offsets::fighter::shulk::ART_COOLDOWN_MULTIPLY_PATCH_4).nop();
        let _ = skyline::patching::Patch::in_text(offsets::fighter::shulk::ART_COOLDOWN_MULTIPLY_PATCH_5).nop();
        let _ = skyline::patching::Patch::in_text(offsets::fighter::shulk::ART_COOLDOWN_MULTIPLY_PATCH_6).nop();
        let _ = skyline::patching::Patch::in_text(offsets::fighter::shulk::ART_COOLDOWN_MULTIPLY_PATCH_7).nop();
        let _ = skyline::patching::Patch::in_text(offsets::fighter::shulk::ART_COOLDOWN_MULTIPLY_PATCH_8).nop();
        let _ = skyline::patching::Patch::in_text(offsets::fighter::shulk::ART_COOLDOWN_MULTIPLY_PATCH_9).nop();
        let _ = skyline::patching::Patch::in_text(offsets::fighter::shulk::ART_COOLDOWN_MULTIPLY_PATCH_10).nop();
        // for the meter handler
        let _ = skyline::patching::Patch::in_text(offsets::fighter::shulk::ART_COOLDOWN_METER_HANDLER_PATCH_1).nop();
        let _ = skyline::patching::Patch::in_text(offsets::fighter::shulk::ART_COOLDOWN_METER_HANDLER_PATCH_2).nop();

        // nops the timer decrease for art cooldowns
        let _ = skyline::patching::Patch::in_text(offsets::fighter::shulk::ART_COOLDOWN_TIMER_DECREASE_PATCH).nop();
    }

    replace_fighter_vtable(
        *FIGHTER_KIND_SHULK,
        FighterVtableSlot::OnAttack,
        shulk_on_attack as *const () as _
    );
}
