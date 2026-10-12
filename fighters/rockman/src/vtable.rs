use super::*;
use wubor_utils::offsets;

#[skyline::from_offset(offsets::fighter::rockman::VTABLE_FUNC)]
unsafe extern "C" fn rockman_vtable_func_inner(vtable: u64, fighter: &mut Fighter);

pub unsafe extern "C" fn rockman_vtable_func(vtable: u64, fighter: &mut Fighter) {
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
        else if !VarModule::is_flag(module_accessor, vars::rockman::instance::flag::CHARGE_SHOT_CHARGING) {
            if ControlModule::get_button(module_accessor) >> 1 & 1 != 0 {
                VarModule::on_flag(module_accessor, vars::rockman::instance::flag::CHARGE_SHOT_CHARGING);
            }
        }
        else {
            if ControlModule::get_button(module_accessor) >> 1 & 1 == 0 {
                if !VarModule::is_flag(module_accessor, vars::rockman::instance::flag::CHARGE_SHOT_PLAYED_FX) {
                    rockman_kill_charge(module_accessor);
                }
                else if !VarModule::is_flag(module_accessor, vars::rockman::instance::flag::CHARGE_SHOT_RELEASE) {
                    let release_frame = WorkModule::get_param_int(module_accessor, hash40("param_buster_charge"), hash40("release_frame"));
                    VarModule::set_int(module_accessor, vars::rockman::instance::int::CHARGE_SHOT_RELEASE_FRAME, release_frame);
                    VarModule::on_flag(module_accessor, vars::rockman::instance::flag::CHARGE_SHOT_RELEASE);
                }
            }

            if VarModule::get_int(module_accessor, vars::rockman::instance::int::CHARGE_SHOT_RELEASE_FRAME) >= 0 {
                VarModule::dec_int(module_accessor, vars::rockman::instance::int::CHARGE_SHOT_RELEASE_FRAME);
            }
            if VarModule::is_flag(module_accessor, vars::rockman::instance::flag::CHARGE_SHOT_PLAYED_FX)
            && (
                VarModule::is_flag(module_accessor, vars::rockman::instance::flag::CHARGE_SHOT_RELEASE)
                && VarModule::get_int(module_accessor, vars::rockman::instance::int::CHARGE_SHOT_RELEASE_FRAME) == 0
            ) {
                rockman_kill_charge(module_accessor);
            }
        }
        if VarModule::is_flag(module_accessor, vars::rockman::instance::flag::CHARGE_SHOT_CHARGING) {
            let charge_frame = VarModule::get_int(module_accessor, vars::rockman::instance::int::CHARGE_SHOT_FRAME);
            let charge_max_frame = WorkModule::get_param_int(module_accessor, hash40("param_buster_charge"), hash40("charge_max_frame"));
            if charge_frame < charge_max_frame + 1 {
                VarModule::inc_int(module_accessor, vars::rockman::instance::int::CHARGE_SHOT_FRAME);
            }
            let charge_frame = VarModule::get_int(module_accessor, vars::rockman::instance::int::CHARGE_SHOT_FRAME);
            if charge_frame == charge_max_frame {
                FighterUtil::flash_eye_info(module_accessor);
                EffectModule::req_follow(
                    module_accessor,
                    Hash40::new("rockman_chargeshot_max"),
                    Hash40::new("hip"),
                    &vars::ZERO_VECTOR,
                    &vars::ZERO_VECTOR,
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
                let snd_handle = VarModule::get_int(module_accessor, vars::rockman::instance::int::CHARGE_SHOT_SND_HANDLE) as i32;
                SoundModule::set_se_vol(module_accessor, snd_handle, 0.6, 0);
            }
            let clear_input_frame = WorkModule::get_param_int(module_accessor, hash40("param_buster_charge"), hash40("clear_input_frame"));
            if charge_frame == clear_input_frame {
                ControlModule::clear_command_one(module_accessor, 0, *FIGHTER_PAD_CMD_CAT1_SPECIAL_N);
            }
            let delay_charge_frame = WorkModule::get_param_int(module_accessor, hash40("param_buster_charge"), hash40("delay_charge_frame"));
            if charge_frame > delay_charge_frame {
                if !VarModule::is_flag(module_accessor, vars::rockman::instance::flag::CHARGE_SHOT_PLAYED_FX) {
                    let snd_handle = SoundModule::play_se(module_accessor, Hash40::new("se_rockman_smash_s02"), true, false, false, false, enSEType(0)) as u32;
                    VarModule::set_int(module_accessor, vars::rockman::instance::int::CHARGE_SHOT_SND_HANDLE, snd_handle as i32);
                    EffectModule::req_follow(
                        module_accessor,
                        Hash40::new("rockman_chargeshot_hold"),
                        Hash40::new("handl"),
                        &Vector3f{x: 1.0, y: 0.0, z: 0.0},
                        &vars::ZERO_VECTOR,
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
                    VarModule::set_int(module_accessor, vars::rockman::instance::int::CHARGE_SHOT_EFF_HANDLE, eff_handle as i32);
                    VarModule::on_flag(module_accessor, vars::rockman::instance::flag::CHARGE_SHOT_PLAYED_FX);
                }
            }
        }
    }
    rockman_vtable_func_inner(vtable, fighter);
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
    VarModule::off_flag(module_accessor, vars::rockman::instance::flag::CHARGE_SHOT_CHARGING);
    VarModule::off_flag(module_accessor, vars::rockman::instance::flag::CHARGE_SHOT_PLAYED_FX);
    VarModule::off_flag(module_accessor, vars::rockman::instance::flag::CHARGE_SHOT_RELEASE);
    VarModule::set_int(module_accessor, vars::rockman::instance::int::CHARGE_SHOT_RELEASE_FRAME, 0);
    SoundModule::stop_se(module_accessor, Hash40::new("se_rockman_smash_s02"), 0);
    VarModule::set_int(module_accessor, vars::rockman::instance::int::CHARGE_SHOT_FRAME, 0);
    let eff_handle = VarModule::get_int(module_accessor, vars::rockman::instance::int::CHARGE_SHOT_EFF_HANDLE) as u32;
    if EffectModule::is_exist_effect(module_accessor, eff_handle) {
        EffectModule::kill(module_accessor, eff_handle, true, true);
    }
    VarModule::set_int(module_accessor, vars::rockman::instance::int::CHARGE_SHOT_EFF_HANDLE, 0);
    VarModule::set_int(module_accessor, vars::rockman::instance::int::CHARGE_SHOT_SND_HANDLE, 0);
}

pub fn install() {
    replace_fighter_vtable(
        *FIGHTER_KIND_ROCKMAN,
        10,
        rockman_vtable_func as *const () as _
    );
}
