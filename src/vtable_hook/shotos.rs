use crate::imports::*;

#[skyline::hook(offset = 0x10d4570)]
unsafe extern "C" fn ryu_ken_init(_vtable: u64, fighter: &mut Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    let control_energy = KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    *(control_energy as *mut u8).add(0xa4) = 1;
    FGCModule::set_command_input_button(module_accessor, Cat4::SPECIAL_N_COMMAND, 1);
    FGCModule::set_command_input_button(module_accessor, Cat4::SPECIAL_S_COMMAND, 2);
    FGCModule::set_command_input_button(module_accessor, Cat4::SPECIAL_HI_COMMAND, 1);
    if fighter.battle_object.kind == 0x3c {
        FGCModule::clone_command_input(module_accessor, Cat4::SPECIAL_S_COMMAND, Cat4::SPECIAL_N2_COMMAND);
        FGCModule::set_command_input_button(module_accessor, Cat4::SPECIAL_N2_COMMAND, 1);
    }
    else {
        FGCModule::clone_command_input(module_accessor, Cat4::SPECIAL_HI_COMMAND, Cat4::SPECIAL_N2_COMMAND);
        FGCModule::set_command_input_button(module_accessor, Cat4::SPECIAL_N2_COMMAND, 2);
        FGCModule::clone_command_input(module_accessor, Cat4::SPECIAL_N_COMMAND, Cat4::ATTACK_COMMAND1);
        FGCModule::set_command_input_button(module_accessor, Cat4::ATTACK_COMMAND1, 2);
    }
    FGCModule::clone_command_input(module_accessor, Cat4::SUPER_SPECIAL2_COMMAND, Cat4::SUPER_SPECIAL_COMMAND);
    FGCModule::set_command_input_button(module_accessor, Cat4::SUPER_SPECIAL_COMMAND, 1);
    FGCModule::set_command_input_button(module_accessor, Cat4::SUPER_SPECIAL2_COMMAND, 2);
}

#[skyline::from_offset(0x646fe0)]
extern "C" fn what_is_this(fighter: &mut Fighter) -> *const u64;

#[skyline::from_offset(0x6da350)]
extern "C" fn what_is_this_2(param_1: i32, param_2: i32, param_3: u64, module_accessor: *mut BattleObjectModuleAccessor, huh: *const u64);

#[skyline::from_offset(0x69ae40)]
extern "C" fn ryu_ken_autoturn_handler(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32, param_2: u32);

#[skyline::hook(offset = 0x10d4df0)]
unsafe extern "C" fn ryu_ken_move_strength_autoturn_handler(_vtable: u64, fighter: &mut Fighter) {
    let object = &mut fighter.battle_object;
    let module_accessor = (*object).module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    let mut prevent_turn = 0;
    match status {
        0x1dc | 0x1eb | 0x1ec => ryu_ken_handle_special_strength(object, hash40("param_special_n")),
        0x1dd | 0x1ef => ryu_ken_handle_special_strength(object, hash40("param_special_s")),
        0x1de | 0x1f4 =>
            if !WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH) {
                ryu_ken_handle_special_strength(object, hash40("param_special_hi"));
            },
        // 0x1df | 0x1f5 => prevent_turn = 1,
        _ => ()
    }
    match status {
        0x27 => ryu_ken_handle_light_normals(fighter, hash40("attack_11_s"), hash40("attack_11_w")),
        0x2a => ryu_ken_handle_light_normals(fighter, hash40("attack_s3_s_s"), hash40("attack_s3_s_w")),
        0x2b => ryu_ken_handle_light_normals(fighter, hash40("attack_hi3_s"), hash40("attack_hi3_w")),
        0x2c => ryu_ken_handle_light_normals(fighter, hash40("attack_lw3_s"), hash40("attack_lw3_w")),
        _ => ()
    };
    if prevent_turn == 0 {
        prevent_turn = StopModule::is_damage(module_accessor) as u32;
    }
    ryu_ken_autoturn_handler(module_accessor, prevent_turn & 1, 0);
    // The rest of the function is proximity normal nonsense I don't want.
}

unsafe extern "C" fn ryu_ken_handle_special_strength(object: &mut BattleObject, param_parent: u64) {
    let module_accessor = (*object).module_accessor;
    if VarModule::is_flag(module_accessor, ryu::status::flag::SPECIAL_DECIDE_STRENGTH) {
        return;
    }
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    let pad_release_w = WorkModule::get_param_int(module_accessor, param_parent, hash40("pad_release_w"));
    let strength = WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if !StopModule::is_stop(module_accessor) && !SlowModule::is_skip(module_accessor)
    && (*battle_object_slow.add(0x8) == 0 || *(battle_object_slow as *const u32) == 2)
    && (strength == 0 && ControlModule::get_button(module_accessor) & 3 == 0) {
        let button_on_timer = WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_BUTTON_ON_TIMER);
        let strength = if button_on_timer <= pad_release_w {
            2
        }
        else {
            1
        };
        WorkModule::set_int(module_accessor, strength, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    }
}

#[skyline::hook(offset = 0x10d5a80)]
unsafe extern "C" fn ryu_ken_handle_light_normals(fighter: &mut Fighter, heavy_motion: u64, light_motion: u64) {
    let module_accessor = fighter.battle_object.module_accessor;
    let mot = MotionModule::motion_kind(module_accessor);
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    if (!StopModule::is_stop(module_accessor) && !SlowModule::is_skip(module_accessor))
    && (*battle_object_slow.add(0x8) == 0 || *(battle_object_slow as *const u32) == 2) {
        if !WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK) {
            if (mot ^ heavy_motion) & 0xffffffffff == 0 {
                let button = ControlModule::get_button(module_accessor);
                let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
                if button & 1 == 0
                && (cat1 >> 0x15 & 1 == 0 || !WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON)) {
                    HitModule::set_status_all(module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
                    WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
                    WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL);
                    WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_SAME_ATTACK_CANCEL);
                    // MotionModule::change_motion_inherit_frame(
                    //     module_accessor,
                    //     Hash40::new_raw(light_motion),
                    //     -1.0,
                    //     1.0,
                    //     0.0,
                    //     false,
                    //     false
                    // );
                    MotionModule::change_motion(
                        module_accessor,
                        Hash40::new_raw(light_motion),
                        0.0,
                        1.0,
                        false,
                        0.0,
                        false,
                        false
                    );
                    WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK);
                    WorkModule::on_flag(module_accessor, 0x21000022 + 6);
                }
            }
        }
        else if (mot ^ light_motion) & 0xffffffffff == 0
        && !WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_CHANGE_LOG)
        && WorkModule::get_int(module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME) < 2 {
            let something = match mot {
                0xb4823ab96 => 0x15, // attack_11_w
                0xc226d08d1 => 0x17, // attack_hi3_w
                0xc672b8ebd => 0x18, // attack_lw3_w
                0xdebddf8ff => 0x16, // attack_s3_s_w
                0xdf2f03674 => 0x1a, // attack_near_w
                _ => 1
            };
            let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            if (cat1 >> 0x15 & 1 == 0 || !WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON))
            && ControlModule::get_command_flag_cat(module_accessor, 3) == 0 {

                let huh = what_is_this(fighter);

                what_is_this_2(2, something, 1, module_accessor, huh);
            }
            WorkModule::on_flag(module_accessor, 0x21000022 + 7);
        }
    }
}

#[skyline::hook(offset = 0x10d6c10)]
unsafe extern "C" fn ryu_ken_on_situation_change(_vtable: u64, fighter: &mut Fighter, log: u64) {
    if *(log as *const u8).add(0xc) == 2 {
        return;
    }
    let object = &mut fighter.battle_object;
    let module_accessor = (*object).module_accessor;
    WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_N);
    WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_LW);
    WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_N_HOP);
    WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
    VarModule::off_flag(module_accessor, fighter::instance::flag::DISABLE_SPECIAL_LW);
}

unsafe extern "C" fn syoryuken_eff_handler(
    module_accessor: *mut BattleObjectModuleAccessor,
    collision_log: *mut CollisionLogScuffed,
    eff: Hash40,
    r: f32,
    g: f32,
    b: f32,
    rate: f32
) {
    let lr = PostureModule::lr(module_accessor);

    let rot = lr * 1.570796;

    let pos = &(*collision_log).location;

    let eff = EffectModule::req(
        module_accessor,
        eff,
        &Vector3f{x: pos.x, y: pos.y, z: pos.z},
        &Vector3f{x: 0.0, y: 0.0, z: rot},
        1.0,
        0,
        -1,
        false,
        2
    ) as u32;
    EffectModule::set_rgb(module_accessor, eff, r, g, b);
    EffectModule::set_rate(module_accessor, eff, rate);
}

#[skyline::hook(offset = 0x10d6ca0)]
unsafe extern "C" fn ryu_ken_on_hit(vtable: u64, fighter: &mut Fighter, log: u64, some_float: f32) {
    let object = &mut fighter.battle_object;
    let module_accessor = (*object).module_accessor;
    let kind = (*object).kind;
    let collision_log = log as *mut CollisionLogScuffed;
    let status = StatusModule::status_kind(module_accessor);
    let collision_kind = (*collision_log).collision_kind;
    let opponent_object_id = (*collision_log).opponent_object_id;
    if kind == 0x3c {
        if [
            *FIGHTER_STATUS_KIND_ATTACK,
            *FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            *FIGHTER_STATUS_KIND_ATTACK_LW3,
            *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_STATUS_KIND_ATTACK_AIR
        ].contains(&status)
        && VarModule::is_flag(module_accessor, ryu::status::flag::USED_DENJIN_CHARGE)
        && collision_kind == 1 {
            let opponent_object = MiscModule::get_battle_object_from_id(opponent_object_id);
            if (*opponent_object).battle_object_id >> 0x1c == 0
            && HitModule::get_status((*opponent_object).module_accessor, (*collision_log).receiver_id as i32, 0) == 0 {
                syoryuken_eff_handler(module_accessor, collision_log, Hash40::new("ryu_syoryuken_hit"), 0.1, 0.45, 0.2, 1.2);
            }
        }
        if status == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B {
            if VarModule::get_int(module_accessor, ryu::status::int::GUARD_SPECIAL_LW_KIND) == ryu::GUARD_SPECIAL_LW_KIND_IMPACT
            && !VarModule::is_flag(module_accessor, ryu::status::flag::SPECIAL_LW_IMPACT_HIT) {
                if collision_kind == 1
                && opponent_object_id >> 0x1c == 0 {
                    let armor_count = WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SUPER_ARMOUR_COUNT);
                    if armor_count != 2 {
                        SoundModule::play_se(
                            module_accessor,
                            Hash40::new("se_ryu_drive_impact_punish"),
                            true,
                            false,
                            false,
                            false,
                            enSEType(0)
                        );
                        syoryuken_eff_handler(module_accessor, collision_log, Hash40::new("ryu_syoryuken_hit"), 0.1, 0.1, 0.1, 0.75);
                        VarModule::on_flag(module_accessor, ryu::status::flag::SPECIAL_LW_IMPACT_HIT);
                    }
                }
                if collision_kind == 2
                && (*collision_log).opponent_object_category == 0 {
                    let opponent_object = MiscModule::get_battle_object_from_id(opponent_object_id);
                    let func: extern "C" fn(*mut BattleObject) -> u64 = std::mem::transmute(*((*(opponent_object as *const u64) + 0x2d0) as *const u64));
                    let flag = if func(opponent_object) == 0 {
                        ryu::status::flag::SPECIAL_LW_IMPACT_SHIELD
                    }
                    else {
                        ryu::status::flag::SPECIAL_LW_IMPACT_JUST_SHIELD
                    };
                    VarModule::on_flag(module_accessor, flag);
                }
            }
            else if VarModule::get_int(module_accessor, ryu::status::int::GUARD_SPECIAL_LW_KIND) == ryu::GUARD_SPECIAL_LW_KIND_REVERSAL {
                if collision_kind == 1 {
                    let opponent_object = MiscModule::get_battle_object_from_id(opponent_object_id);
                    if (*opponent_object).battle_object_id >> 0x1c == 0
                    && HitModule::get_status((*opponent_object).module_accessor, (*collision_log).receiver_id as i32, 0) == 0 {
                        syoryuken_eff_handler(module_accessor, collision_log, Hash40::new("ryu_syoryuken_hit"), 0.3, 0.1, 0.1, 0.75);
                    }
                }
            }
        }
        if status == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F {
            let opponent_object = MiscModule::get_battle_object_from_id(opponent_object_id);
            let opponent_module_accessor = (*opponent_object).module_accessor;
            let slow_frame = SlowModule::frame(opponent_module_accessor, 0);
            if slow_frame < 10 {
                SlowModule::set(opponent_module_accessor, 0, 50, 10, false, 0x50000000);
            }
        }
    }
    if [
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND
    ].contains(&status) {
        if !VarModule::is_flag(module_accessor, ryu::status::flag::SPECIAL_HI_SPECIAL_EFFECT) {
            return;
        }
        if collision_kind == 1 {
            let opponent_object = MiscModule::get_battle_object_from_id(opponent_object_id);
            if (*opponent_object).battle_object_id >> 0x1c == 0
            && HitModule::get_status((*opponent_object).module_accessor, (*collision_log).receiver_id as i32, 0) == 0 {
                // let attack_data = AttackModule::attack_data(module_accessor, (*collision_log).collider_id as i32, (*collision_log).x35);
                // let mut angle = 0.0;
                // if (*attack_data).vector != 0x169 {
                //     angle = (*attack_data).vector as f32 * 0.01745329;
                // }

                let command = WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND);

                let eff = if kind == 0x3c {
                    if command {
                        Hash40::new("ryu_syoryuken_hit2")
                    }
                    else {
                        Hash40::new("ryu_syoryuken_hit")
                    }
                }
                else {
                    if command {
                        Hash40::new("ken_syoryuken_hit2")
                    }
                    else {
                        Hash40::new("ken_syoryuken_hit")
                    }
                };
                syoryuken_eff_handler(module_accessor, collision_log, eff, 1.0, 1.0, 1.0, 1.0);
            }
        }
        return;
    }
    if status == *FIGHTER_STATUS_KIND_FINAL {
        if collision_kind != 1 {
            return;
        }
    }
    original!()(vtable, fighter, log, some_float);
}

#[skyline::hook(offset = 0x10d7420)]
unsafe extern "C" fn ryu_ken_on_hit_2(vtable: u64, fighter: &mut Fighter, log: u64) {
    let object = &mut fighter.battle_object;
    if (*object).kind == 0x3c {
        let module_accessor = (*object).module_accessor;
        let status = StatusModule::status_kind(module_accessor);
        if status == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B
        && VarModule::get_int(module_accessor, ryu::status::int::GUARD_SPECIAL_LW_KIND) == ryu::GUARD_SPECIAL_LW_KIND_IMPACT
        && !VarModule::is_flag(module_accessor, ryu::status::flag::SPECIAL_LW_IMPACT_SHIELD_WIND)
        && !VarModule::is_flag(module_accessor, ryu::status::flag::SPECIAL_LW_IMPACT_JUST_SHIELD)
        && VarModule::is_flag(module_accessor, ryu::status::flag::SPECIAL_LW_IMPACT_SHIELD) {
            MotionAnimcmdModule::call_script_single(module_accessor, *FIGHTER_ANIMCMD_GAME, Hash40::new("game_speciallwimpactonshield"), -1);
            VarModule::on_flag(module_accessor, ryu::status::flag::SPECIAL_LW_IMPACT_SHIELD_WIND);
        }
    }
    original!()(vtable, fighter, log);
}

#[skyline::hook(offset = 0x10d7b80)]
unsafe extern "C" fn ryu_ken_on_search(vtable: u64, fighter: &mut Fighter, log: u64, some_float: f32) {
    let object = &mut fighter.battle_object;
    if (*object).kind != 0x3c {
        return original!()(vtable, fighter, log, some_float);
    }
    let module_accessor = (*object).module_accessor;
    let collision_log = *(log as *const u64).add(0x10 / 0x8);
    let collision_log = collision_log as *const CollisionLogScuffed;
    let status = StatusModule::status_kind(module_accessor);
    if status == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F {
        let opponent_object = MiscModule::get_battle_object_from_id((*collision_log).opponent_object_id);
        let opponent_module_accessor = (*opponent_object).module_accessor;
        let slow_frame = SlowModule::frame(opponent_module_accessor, 0);
        if slow_frame < 10 {
            SlowModule::set(opponent_module_accessor, 0, 50, 10, false, 0x50000000);
        }
    }
}

#[skyline::hook(offset = 0x10d7760)]
unsafe extern "C" fn ryu_ken_on_damage(vtable: u64, fighter: &mut Fighter, on_damage: u64) {
    let object = &mut fighter.battle_object;
    if (*object).kind == 0x3d {
        original!()(vtable, fighter, on_damage);
        return;
    }
    let module_accessor = (*object).module_accessor;
    if *(on_damage as *const u8).add(0x18) != 0 {
        let control_energy = KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        if *(control_energy as *const u8).add(0xa4) != 0 {
            *(control_energy as *mut u8).add(0xa5) = 0;
        }
        WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
    }
    let status = StatusModule::status_kind(module_accessor);
    if status == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B
    && VarModule::is_flag(module_accessor, ryu::status::flag::SPECIAL_LW_IMPACT_ENABLED_ARMOR)
    && *(*(on_damage as *const *const u64).offset(0x10 / 0x8) as *const u8).add(0xd0) == 0 {
        let mut armor_count = WorkModule::get_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SUPER_ARMOUR_COUNT);
        if 0 < armor_count {
            SoundModule::play_se(
                module_accessor,
                Hash40::new("se_ryu_drive_impact_armor"),
                true,
                false,
                false,
                false,
                enSEType(0)
            );
            MotionAnimcmdModule::call_script_single(module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_speciallwimpactarmor"), -1);
        }
        armor_count -= 1;
        WorkModule::dec_int(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SUPER_ARMOUR_COUNT);
        if armor_count < 1 {
            DamageModule::reset_no_reaction_mode_status(module_accessor);
            HitModule::set_hit_stop_mul(module_accessor, 1.0, HitStopMulTarget{ _address: *HIT_STOP_MUL_TARGET_ALL as u8 }, 0.0);
            HitModule::set_defense_mul_status(module_accessor, 1.0);
            WorkModule::set_float(module_accessor, 0.0, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_ENERGY_TYPE_ATTACK_SPEED_MUL);
        }
    }
}

#[skyline::hook(offset = 0x10d5f30)]
unsafe extern "C" fn ryu_ken_frame(vtable: u64, fighter: &mut Fighter) {
    original!()(vtable, fighter);
    let object = &mut fighter.battle_object;
    let module_accessor = (*object).module_accessor;

    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    if *battle_object_slow.add(0x8) != 0 && *(battle_object_slow as *const u32) != 2 {
        return;
    }

    if WorkModule::is_flag(module_accessor, 0x200000e9)
    && WorkModule::is_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL) {
        let work_module = *(module_accessor as *const *const u64).add(0x50 / 8);
        if ryu_ken_check_final_can_cancel(work_module) {
            let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            let cat4 = ControlModule::get_command_flag_cat(module_accessor, 3);
            let mut status = -1;
            if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND != 0 {
                VarModule::on_flag(module_accessor, ryu::instance::flag::SKIP_FINAL_PROX_CHECK);
                status = *FIGHTER_RYU_STATUS_KIND_FINAL2;
            }
            else if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND != 0 {
                VarModule::on_flag(module_accessor, ryu::instance::flag::SKIP_FINAL_PROX_CHECK);
                status = *FIGHTER_STATUS_KIND_FINAL;
            }
            else if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N != 0 {
                status = *FIGHTER_STATUS_KIND_FINAL;
            }
            if status != -1 {
                if StopModule::is_stop(module_accessor) && StopModule::is_hit(module_accessor) {
                    StopModule::cancel_hit_stop(module_accessor);
                }
                if SlowModule::frame(module_accessor, 1) > 1 {
                    SlowModule::clear_immediate(module_accessor, 1, false);
                }
                StatusModule::change_status_request(module_accessor, status, true);
            }
        }
    }
}

#[skyline::from_offset(0x695c80)]
fn ryu_ken_check_final_can_cancel(work_module: *const u64) -> bool;

#[skyline::hook(offset = 0x10d6a10)]
unsafe extern "C" fn ryu_ken_status_change_callback(_vtable: u64, fighter: &mut Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    let force_reset;
    let command_kind;
    if fighter.battle_object.kind == *FIGHTER_KIND_KIRBY as u32 {
        if status == *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N2_COMMAND {
            force_reset = false;
            command_kind = 1;
        }
        else if status == *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND {
            force_reset = false;
            command_kind = 0;
        }
        else if status == *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N {
            force_reset = false;
            command_kind = -1;
        }
        else {
            force_reset = true;
            command_kind = -1;
        }
    }
    else {
        if [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_STATUS_KIND_SPECIAL_HI,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_STATUS_KIND_FINAL,
            *FIGHTER_RYU_STATUS_KIND_FINAL2
        ].contains(&status) {
            force_reset = true;
            command_kind = -1;
        }
        else if status == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND {
            force_reset = false;
            command_kind = 0;
        }
        else if status == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND {
            force_reset = false;
            command_kind = 1;
        }
        else if status == *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND {
            force_reset = false;
            command_kind = 2;
        }
        else if status == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND {
            force_reset = false;
            command_kind = 3;
        }
        else {
            force_reset = true;
            command_kind = -1;
        }
    }
    let opponent_lr = WorkModule::get_float(module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    let lr = PostureModule::lr(module_accessor);
    if opponent_lr != 0.0
    && lr != opponent_lr {
        return;
    }

    if command_kind < 0 {
        if !force_reset {
            return;
        }
    }
    else {
        let special_command_lr = ControlModule::get_special_command_lr(module_accessor, command_kind);
        if special_command_lr != 0.0 && special_command_lr != lr {
            PostureModule::set_lr(module_accessor, special_command_lr);
            PostureModule::update_rot_y_lr(module_accessor);
        }
    }
    
    // CommandModule::reset_special_command(module_accessor, false);
}

pub fn install() {
    // Patches the original final smash cancel check
    let _ = skyline::patching::Patch::in_text(0x10d6324).data(0x14000039u32);

    // Allows any status over 0x1de to be final smash cancelable
    let _ = skyline::patching::Patch::in_text(0x10d67d8).data(0x1400000Au32);

    // Some Kind of Transition Check
    // Ryu
    let _ = skyline::patching::Patch::in_text(0x5033eb0 + 0x1F8).data(1u8);

    skyline::install_hooks!(
        ryu_ken_init,
        ryu_ken_move_strength_autoturn_handler,
        ryu_ken_handle_light_normals,
        ryu_ken_on_situation_change,
        ryu_ken_on_hit,
        ryu_ken_on_hit_2,
        ryu_ken_on_search,
        ryu_ken_on_damage,
        ryu_ken_frame,
        ryu_ken_status_change_callback
    );
}