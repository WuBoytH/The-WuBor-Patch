use crate::imports::status_imports::*;
use smash_rs::app::{WorkId, work_ids, transition_groups, transition_terms};
// use std::arch::asm;

#[skyline::hook(offset = 0x107e970)]
pub unsafe extern "C" fn rockman_vtable_func(vtable: u64, fighter: &mut smash::app::Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    let is_not_slow = (*battle_object_slow.add(0x8) == 0 || *(battle_object_slow as *const u32) == 0)
    && !StopModule::is_stop(module_accessor) && !SlowModule::is_skip(module_accessor);
    if is_not_slow {
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
        || WorkModule::is_flag(module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LEAFSHIELD) {
            rockman_kill_charge(module_accessor);
        }
        else if !VarModule::is_flag(module_accessor, rockman::instance::flag::CHARGE_SHOT_CHARGING) {
            if ControlModule::get_button(module_accessor) >> 1 & 1 != 0 {
                VarModule::on_flag(module_accessor, rockman::instance::flag::CHARGE_SHOT_CHARGING);
            }
        }
        else {
            if ControlModule::get_button(module_accessor) >> 1 & 1 == 0 {
                if !VarModule::is_flag(module_accessor, rockman::instance::flag::CHARGE_SHOT_PLAYED_FX) {
                    rockman_kill_charge(module_accessor);
                }
                else if !VarModule::is_flag(module_accessor, rockman::instance::flag::CHARGE_SHOT_RELEASE) {
                    let release_frame = WorkModule::get_param_int(module_accessor, hash40("param_buster_charge"), hash40("release_frame"));
                    VarModule::set_int(module_accessor, rockman::instance::int::CHARGE_SHOT_RELEASE_FRAME, release_frame);
                    VarModule::on_flag(module_accessor, rockman::instance::flag::CHARGE_SHOT_RELEASE);
                }
            }
            
            if VarModule::get_int(module_accessor, rockman::instance::int::CHARGE_SHOT_RELEASE_FRAME) >= 0 {
                VarModule::dec_int(module_accessor, rockman::instance::int::CHARGE_SHOT_RELEASE_FRAME);
            }
            if VarModule::is_flag(module_accessor, rockman::instance::flag::CHARGE_SHOT_PLAYED_FX)
            && (
                VarModule::is_flag(module_accessor, rockman::instance::flag::CHARGE_SHOT_RELEASE)
                && VarModule::get_int(module_accessor, rockman::instance::int::CHARGE_SHOT_RELEASE_FRAME) == 0
            ) {
                rockman_kill_charge(module_accessor);
            }
        }
        if VarModule::is_flag(module_accessor, rockman::instance::flag::CHARGE_SHOT_CHARGING) {
            let charge_frame = VarModule::get_int(module_accessor, rockman::instance::int::CHARGE_SHOT_FRAME);
            let charge_max_frame = WorkModule::get_param_int(module_accessor, hash40("param_buster_charge"), hash40("charge_max_frame"));
            if charge_frame < charge_max_frame + 1 {
                VarModule::inc_int(module_accessor, rockman::instance::int::CHARGE_SHOT_FRAME);
            }
            let charge_frame = VarModule::get_int(module_accessor, rockman::instance::int::CHARGE_SHOT_FRAME);
            if charge_frame == charge_max_frame {
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
            let clear_input_frame = WorkModule::get_param_int(module_accessor, hash40("param_buster_charge"), hash40("clear_input_frame"));
            if charge_frame == clear_input_frame {
                ControlModule::clear_command_one(module_accessor, 0, *FIGHTER_PAD_CMD_CAT1_SPECIAL_N);
            }
            let delay_charge_frame = WorkModule::get_param_int(module_accessor, hash40("param_buster_charge"), hash40("delay_charge_frame"));
            if charge_frame > delay_charge_frame {
                if !VarModule::is_flag(module_accessor, rockman::instance::flag::CHARGE_SHOT_PLAYED_FX) {
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
                    VarModule::set_int(module_accessor, rockman::instance::int::CHARGE_SHOT_EFF_HANDLE, eff_handle as i32);
                    VarModule::on_flag(module_accessor, rockman::instance::flag::CHARGE_SHOT_PLAYED_FX);
                }
            }
        }
    }
    original!()(vtable, fighter);
}

unsafe extern "C" fn rockman_valid_charging_state(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    if WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) {
        return true;
    }
    // if MiscModule::is_damage_check(module_accessor, false) {
    //     return false;
    // }
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

unsafe extern "C" fn rockman_kill_charge(module_accessor: *mut BattleObjectModuleAccessor) {
    VarModule::off_flag(module_accessor, rockman::instance::flag::CHARGE_SHOT_CHARGING);
    VarModule::off_flag(module_accessor, rockman::instance::flag::CHARGE_SHOT_PLAYED_FX);
    VarModule::off_flag(module_accessor, rockman::instance::flag::CHARGE_SHOT_RELEASE);
    VarModule::set_int(module_accessor, rockman::instance::int::CHARGE_SHOT_RELEASE_FRAME, 0);
    SoundModule::stop_se(module_accessor, Hash40::new("se_rockman_smash_s02"), 0);
    VarModule::set_int(module_accessor, rockman::instance::int::CHARGE_SHOT_FRAME, 0);
    let eff_handle = VarModule::get_int(module_accessor, rockman::instance::int::CHARGE_SHOT_EFF_HANDLE) as u32;
    if EffectModule::is_exist_effect(module_accessor, eff_handle) {
        EffectModule::kill(module_accessor, eff_handle, true, true);
    }
}

#[skyline::hook(offset = 0x1083bec, inline)]
unsafe extern "C" fn rockman_do_leafshield_things_disable(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = *ctx.registers[19].x.as_ref() as *mut BattleObjectModuleAccessor;
    FighterSpecializer_Rockman::set_leafshield(module_accessor, false);
}

#[skyline::hook(offset = 0x10838e0, inline)]
unsafe extern "C" fn rockman_do_leafshield_things_enable(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = *ctx.registers[19].x.as_ref() as *mut BattleObjectModuleAccessor;
    FighterSpecializer_Rockman::set_leafshield(module_accessor, true);
}

const LEAFSHIELD_DISABLE_GROUPS: [WorkId; 6] = [
    // transition_groups::CHECK_GROUND_SPECIAL,
    // transition_groups::CHECK_AIR_SPECIAL,
    transition_groups::CHECK_GROUND_ESCAPE,
    // transition_groups::CHECK_AIR_ESCAPE,
    transition_groups::CHECK_GROUND_GUARD,
    transition_groups::CHECK_GROUND_ATTACK,
    transition_groups::CHECK_GROUND_CATCH,
    transition_groups::CHECK_AIR_ATTACK,
    transition_groups::CHECK_AIR_CLIFF
];

const LEAFSHIELD_DISABLE_INDIVI: [WorkId; 8] = [
    // transition_terms::CONT_DASH,
    // transition_terms::CONT_TURN_DASH,
    transition_terms::CONT_ATTACK_DASH,
    transition_terms::CONT_CATCH_DASH,
    transition_terms::CONT_CATCH_TURN,
    // transition_terms::CONT_ATTACK_HI4_START,
    // transition_terms::CONT_ATTACK_LW4_START,
    transition_terms::CONT_SPECIAL_N,
    transition_terms::CONT_SPECIAL_S,
    transition_terms::CONT_SPECIAL_HI,
    transition_terms::CONT_CLIFF_ATTACK,
    transition_terms::CONT_CLIFF_ESCAPE
];

#[skyline::hook(replace = FighterSpecializer_Rockman::set_leafshield)]
unsafe extern "C" fn set_leafshield(module_accessor: *mut smash_rs::app::BattleObjectModuleAccessor, set_shield: bool) {
    let work = (*module_accessor).work();
    work.set_flag(set_shield, work_ids::fighter::rockman::instance::SPECIAL_LW_LEAFSHIELD);
    work.set_flag(set_shield, work_ids::fighter::rockman::instance::SPECIAL_LW_ENABLE_SHOOT);
    if !set_shield {
        work.set_int(0, work_ids::fighter::rockman::instance::SPECIAL_LW_HOLD_FRAME);
        for x in LEAFSHIELD_DISABLE_GROUPS.iter() {
            work.unable_transition_term_forbid_group_indivi(*x);
        }
        for x in LEAFSHIELD_DISABLE_INDIVI.iter() {
            work.unable_transition_term_forbid_indivi(*x);
        }
        // work.enable_transition_term_forbid(transition_terms::CONT_SPECIAL_LW);
        // if (*module_accessor).status().status_kind() < 0x27 {
        //     for x in LEAFSHIELD_DISABLE_GROUPS.iter() {
        //         work.enable_transition_term_group(*x);
        //     }
        //     for x in LEAFSHIELD_DISABLE_INDIVI.iter() {
        //         work.enable_transition_term(*x);
        //     }
        // }
    }
    else {
        let hold_frame = work.get_param_int(smash_rs::phx::Hash40::new("param_special_lw"), smash_rs::phx::Hash40::new("hold_frame"));
        work.set_int(hold_frame, work_ids::fighter::rockman::instance::SPECIAL_LW_HOLD_FRAME);
        for x in LEAFSHIELD_DISABLE_GROUPS.iter() {
            work.enable_transition_term_forbid_group_indivi(*x);
        }
        for x in LEAFSHIELD_DISABLE_INDIVI.iter() {
            work.enable_transition_term_forbid_indivi(*x);
        }
    }
}

pub fn install() {
    // Forces the original Leaf Shield handler to not run so we can run the custom one.
    skyline::patching::Patch::in_text(0x107eaa4).data(0x1400001Eu32);
    // Removes the check that forces the removal of Leaf Shield if you are not within certain statuses.
    skyline::patching::Patch::in_text(0x107ff6c).data(0x14000007u32);

    // Disable's the manual checks so it can use FighterSpecializer_Rockman::is_leafshield instead.
    // Disable
    skyline::patching::Patch::in_text(0x1083bec).nop();
    skyline::patching::Patch::in_text(0x1083c0c).nop();
    skyline::patching::Patch::in_text(0x1083c28).nop();
    skyline::patching::Patch::in_text(0x1083c3c).nop();
    skyline::patching::Patch::in_text(0x1083c50).nop();
    skyline::patching::Patch::in_text(0x1083c6c).nop();
    skyline::patching::Patch::in_text(0x1083c80).nop();
    skyline::patching::Patch::in_text(0x1083c94).nop();
    skyline::patching::Patch::in_text(0x1083ca8).nop();
    skyline::patching::Patch::in_text(0x1083cbc).nop();
    skyline::patching::Patch::in_text(0x1083cd0).nop();
    skyline::patching::Patch::in_text(0x1083ce4).nop();
    // Enable
    skyline::patching::Patch::in_text(0x10838e0).nop();
    skyline::patching::Patch::in_text(0x1083900).nop();
    skyline::patching::Patch::in_text(0x1083928).nop();
    skyline::patching::Patch::in_text(0x1083944).nop();
    skyline::patching::Patch::in_text(0x1083958).nop();
    skyline::patching::Patch::in_text(0x108396c).nop();
    skyline::patching::Patch::in_text(0x1083988).nop();
    skyline::patching::Patch::in_text(0x108399c).nop();
    skyline::patching::Patch::in_text(0x10839b0).nop();
    skyline::patching::Patch::in_text(0x10839c4).nop();
    skyline::patching::Patch::in_text(0x10839d8).nop();
    skyline::patching::Patch::in_text(0x10839ec).nop();

    // Patches which status to compare to for Metal Blade.
    skyline::patching::Patch::in_text(0x1080284).nop();
    skyline::patching::Patch::in_text(0x1080288).nop();

    skyline::install_hooks!(
        rockman_vtable_func,
        rockman_do_leafshield_things_disable,
        rockman_do_leafshield_things_enable,
        set_leafshield
    );
}