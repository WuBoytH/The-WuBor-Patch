use crate::imports::status_imports::*;
use smash_rs::app::{WorkId, work_ids, transition_groups, transition_terms};
use super::vl;

#[skyline::hook(offset = 0x107e950)]
pub unsafe extern "C" fn rockman_vtable_func(vtable: u64, fighter: &mut smash::app::Fighter) {
    let object = &mut fighter.battle_object;
    let module_accessor = object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    if (*battle_object_slow.add(0x8) == 0 || *(battle_object_slow as *const u32) == 0)
    && !StopModule::is_stop(module_accessor) && !SlowModule::is_skip(module_accessor) {
        if ![
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_LW_SHOOT
        ].contains(&status)
        && WorkModule::is_flag(module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LEAFSHIELD) {
            WorkModule::dec_int(module_accessor, 0x100000c3); // FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_SPECIAL_LW_HOLD_FRAME
            if WorkModule::is_flag(module_accessor, 0x200000e1) { // FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_ENABLE_SHOOT
                if WorkModule::get_int(module_accessor, 0x100000c3) <= 0 {
                    LinkModule::send_event_nodes(
                        module_accessor,
                        *LINK_NO_ARTICLE,
                        Hash40::new_raw(0x2435e7c874),
                        0
                    );
                    ArticleModule::remove(module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_LEAFSHIELD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                    FighterSpecializer_Rockman::set_leafshield(module_accessor, false);
                }
                else if WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW)
                && ControlModule::get_button(module_accessor) >> 1 & 1 != 0 {
                    StatusModule::change_status_request(module_accessor, *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_LW_SHOOT, true);
                }
            }
        }

        // New stuff for Charge Shot

        if !rockman_valid_charging_state(module_accessor)
        || WorkModule::is_flag(module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LEAFSHIELD)
        || (
            VarModule::is_flag(object, rockman::instance::flag::CHARGE_SHOT_CHARGING)
            && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW)
        ) {
            if VarModule::is_flag(object, rockman::instance::flag::CHARGE_SHOT_CHARGING)
            && !VarModule::is_flag(object, rockman::status::flag::CHARGE_SHOT_KEEP_CHARGE) {
                let pad_flag = ControlModule::get_pad_flag(module_accessor);
                if !VarModule::is_flag(object, rockman::instance::flag::CHARGE_SHOT_PLAYED_FX)
                || pad_flag & *FIGHTER_PAD_FLAG_SPECIAL_RELEASE == 0 {
                    VarModule::off_flag(object, rockman::instance::flag::CHARGE_SHOT_CHARGING);
                    VarModule::off_flag(object, rockman::instance::flag::CHARGE_SHOT_PLAYED_FX);
                    SoundModule::stop_se(module_accessor, Hash40::new("se_rockman_smash_s02"), 0);
                    VarModule::set_int(object, rockman::instance::int::CHARGE_SHOT_FRAME, 0);
                    let eff_handle = VarModule::get_int(object, rockman::instance::int::CHARGE_SHOT_EFF_HANDLE) as u32;
                    if EffectModule::is_exist_effect(module_accessor, eff_handle) {
                        EffectModule::kill(module_accessor, eff_handle, true, true);
                    }
                }
            }
        }
        else if !VarModule::is_flag(object, rockman::instance::flag::CHARGE_SHOT_CHARGING) {
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
                VarModule::on_flag(object, rockman::instance::flag::CHARGE_SHOT_CHARGING);
            }
        }
        else {
            let charge_frame = VarModule::get_int(object, rockman::instance::int::CHARGE_SHOT_FRAME);
            if charge_frame < vl::private::CHARGE_SHOT_MAX_FRAME + 1
            && !VarModule::is_flag(object, rockman::status::flag::CHARGE_SHOT_KEEP_CHARGE) {
                VarModule::inc_int(object, rockman::instance::int::CHARGE_SHOT_FRAME);
            }
            let charge_frame = VarModule::get_int(object, rockman::instance::int::CHARGE_SHOT_FRAME);
            if charge_frame == vl::private::CHARGE_SHOT_MAX_FRAME {
                FighterUtil::flash_eye_info(module_accessor);
                EffectModule::req_follow(
                    module_accessor,
                    Hash40::new("rockman_chargeshot_max"),
                    Hash40::new("hip"),
                    &ZERO_VECTOR,
                    &ZERO_VECTOR,
                    0.75,
                    false,
                    0,
                    0,
                    0,
                    0,
                    0,
                    false,
                    false
                );
            }
            if charge_frame == vl::private::CHARGE_SHOT_CLEAR_INPUT_FRAME {
                ControlModule::clear_command_one(module_accessor, 0, *FIGHTER_PAD_CMD_CAT1_SPECIAL_N);
            }
            if charge_frame > vl::private::CHARGE_SHOT_DELAY_CHARGE_FRAME {
                if !VarModule::is_flag(object, rockman::instance::flag::CHARGE_SHOT_PLAYED_FX) {
                    SoundModule::play_se(module_accessor, Hash40::new("se_rockman_smash_s02"), true, false, false, false, enSEType(0));
                    EffectModule::req_follow(
                        module_accessor,
                        Hash40::new("rockman_chargeshot_hold"),
                        Hash40::new("handl"),
                        &Vector3f{x: 1.0, y: 0.0, z: 0.0},
                        &ZERO_VECTOR,
                        0.4,
                        false,
                        0,
                        0,
                        0,
                        0,
                        0,
                        false,
                        false
                    );
                    let eff_handle = EffectModule::get_last_handle(module_accessor) as u32;
                    // EffectModule::set_rate(module_accessor, eff_handle, 0.5);
                    VarModule::set_int(object, rockman::instance::int::CHARGE_SHOT_EFF_HANDLE, eff_handle as i32);
                    VarModule::on_flag(object, rockman::instance::flag::CHARGE_SHOT_PLAYED_FX);
                }
            }
        }
    }
    original!()(vtable, fighter);
}

unsafe fn rockman_valid_charging_state(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    if CancelModule::is_enable_cancel(module_accessor) {
        return true;
    }
    if MiscModule::is_damage_check(module_accessor, false) {
        return false;
    }
    let status = StatusModule::status_kind(module_accessor);
    ![
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_MISS_FOOT,
        *FIGHTER_STATUS_KIND_STANDBY,
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_STATUS_KIND_FINAL,
        *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_LW_SHOOT,
        *FIGHTER_ROCKMAN_STATUS_KIND_FINAL_INHALE,
        *FIGHTER_ROCKMAN_STATUS_KIND_FINAL_SCENE01,
        *FIGHTER_ROCKMAN_STATUS_KIND_FINAL_SCENE02,
        *FIGHTER_ROCKMAN_STATUS_KIND_FINAL_END
    ].contains(&status)
}

#[skyline::hook(offset = 0x1083bcc, inline)]
unsafe fn rockman_do_leafshield_things_disable(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = *ctx.registers[19].x.as_ref() as *mut BattleObjectModuleAccessor;
    FighterSpecializer_Rockman::set_leafshield(module_accessor, false);
}

#[skyline::hook(offset = 0x10838c0, inline)]
unsafe fn rockman_do_leafshield_things_enable(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = *ctx.registers[19].x.as_ref() as *mut BattleObjectModuleAccessor;
    FighterSpecializer_Rockman::set_leafshield(module_accessor, true);
}

const ROCKMAN_DISABLE_GROUPS: [WorkId; 2] = [
    // transition_groups::CHECK_GROUND_SPECIAL,
    // transition_groups::CHECK_AIR_SPECIAL,
    transition_groups::CHECK_GROUND_ESCAPE,
    transition_groups::CHECK_AIR_ESCAPE,
];

const ROCKMAN_DISABLE_INDIVI: [WorkId; 8] = [
    transition_terms::CONT_DASH,
    transition_terms::CONT_TURN_DASH,
    transition_terms::CONT_ATTACK_DASH,
    transition_terms::CONT_ATTACK_HI4_START,
    transition_terms::CONT_ATTACK_LW4_START,
    transition_terms::CONT_SPECIAL_N,
    transition_terms::CONT_SPECIAL_S,
    transition_terms::CONT_SPECIAL_HI
];

#[skyline::hook(replace = FighterSpecializer_Rockman::set_leafshield)]
unsafe extern "C" fn set_leafshield(module_accessor: *mut smash_rs::app::BattleObjectModuleAccessor, set_shield: bool) {
    let work = (*module_accessor).work();
    work.set_flag(set_shield, work_ids::fighter::rockman::instance::SPECIAL_LW_LEAFSHIELD);
    work.set_flag(set_shield, work_ids::fighter::rockman::instance::SPECIAL_LW_ENABLE_SHOOT);
    if !set_shield {
        work.set_int(0, work_ids::fighter::rockman::instance::SPECIAL_LW_HOLD_FRAME);
        for x in ROCKMAN_DISABLE_GROUPS.iter() {
            work.unable_transition_term_forbid_group(*x);
        }
        for x in ROCKMAN_DISABLE_INDIVI.iter() {
            work.unable_transition_term_forbid(*x);
        }
        // work.enable_transition_term_forbid(transition_terms::CONT_SPECIAL_LW);
        if (*module_accessor).status().status_kind() < 0x27 {
            for x in ROCKMAN_DISABLE_GROUPS.iter() {
                work.enable_transition_term_group(*x);
            }
            for x in ROCKMAN_DISABLE_INDIVI.iter() {
                work.enable_transition_term(*x);
            }
        }
    }
    else {
        let hold_frame = work.get_param_int(smash_rs::phx::Hash40::new("param_special_lw"), smash_rs::phx::Hash40::new("hold_frame"));
        work.set_int(hold_frame, work_ids::fighter::rockman::instance::SPECIAL_LW_HOLD_FRAME);
        for x in ROCKMAN_DISABLE_GROUPS.iter() {
            work.enable_transition_term_forbid_group(*x);
        }
        for x in ROCKMAN_DISABLE_INDIVI.iter() {
            work.enable_transition_term_forbid(*x);
        }
    }
}

pub fn install() {
    // Forces the original Leaf Shield handler to not run so we can run the custon one.
    skyline::patching::Patch::in_text(0x107ea84).data(0x1400001E);
    // Removes the check that forces the removal of Leaf Shield if you are not within certain statuses.
    skyline::patching::Patch::in_text(0x107ff4c).data(0x14000007);

    // Disable's the manual checks so it can use FighterSpecializer_Rockman::is_leafshield instead.
    // Disable
    skyline::patching::Patch::in_text(0x1083bcc).nop();
    skyline::patching::Patch::in_text(0x1083bec).nop();
    skyline::patching::Patch::in_text(0x1083c08).nop();
    skyline::patching::Patch::in_text(0x1083c1c).nop();
    skyline::patching::Patch::in_text(0x1083c30).nop();
    skyline::patching::Patch::in_text(0x1083c4c).nop();
    skyline::patching::Patch::in_text(0x1083c60).nop();
    skyline::patching::Patch::in_text(0x1083c74).nop();
    skyline::patching::Patch::in_text(0x1083c88).nop();
    skyline::patching::Patch::in_text(0x1083c9c).nop();
    skyline::patching::Patch::in_text(0x1083cb0).nop();
    skyline::patching::Patch::in_text(0x1083cc4).nop();
    // Enable
    skyline::patching::Patch::in_text(0x10838c0).nop();
    skyline::patching::Patch::in_text(0x10838e0).nop();
    skyline::patching::Patch::in_text(0x1083908).nop();
    skyline::patching::Patch::in_text(0x1083924).nop();
    skyline::patching::Patch::in_text(0x1083938).nop();
    skyline::patching::Patch::in_text(0x108394c).nop();
    skyline::patching::Patch::in_text(0x1083968).nop();
    skyline::patching::Patch::in_text(0x108397c).nop();
    skyline::patching::Patch::in_text(0x1083990).nop();
    skyline::patching::Patch::in_text(0x10839a4).nop();
    skyline::patching::Patch::in_text(0x10839b8).nop();
    skyline::patching::Patch::in_text(0x10839cc).nop();

    skyline::install_hooks!(
        rockman_vtable_func,
        rockman_do_leafshield_things_disable,
        rockman_do_leafshield_things_enable,
        set_leafshield
    );
}