use super::*;
use wubor_utils::offsets;

#[skyline::from_offset(offsets::fighter::common::AUTOTURN_HANDLER)]
extern "C" fn ryu_ken_autoturn_handler(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32, param_2: u32);

#[skyline::from_offset(offsets::fighter::common::CHECK_FINAL_CAN_CANCEL)]
fn ryu_ken_check_final_can_cancel(work_module: *const u64) -> bool;

// Hooked in the main plugin (src/vtable_hook/shotos.rs); it is not a vtable function.
#[skyline::from_offset(offsets::fighter::shotos::HANDLE_LIGHT_NORMALS)]
unsafe extern "C" fn ryu_ken_handle_light_normals(fighter: &mut Fighter, heavy_motion: u64, light_motion: u64);

#[skyline::from_offset(offsets::fighter::shotos::ON_HIT)]
unsafe extern "C" fn ken_on_attack_inner(vtable: u64, fighter: &mut Fighter, log: u64, some_float: f32);

#[skyline::from_offset(offsets::fighter::shotos::FRAME)]
unsafe extern "C" fn ken_on_frame5_inner(vtable: u64, fighter: &mut Fighter);

unsafe extern "C" fn ken_reset1(_vtable: u64, fighter: &mut Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    let control_energy = KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    *(control_energy as *mut u8).add(0xa4) = 1;
    FGCModule::set_command_input_button(module_accessor, Cat4::SPECIAL_N_COMMAND, 1);
    FGCModule::set_command_input_button(module_accessor, Cat4::SPECIAL_S_COMMAND, 2);
    FGCModule::set_command_input_button(module_accessor, Cat4::SPECIAL_HI_COMMAND, 1);
    FGCModule::clone_command_input(module_accessor, Cat4::SPECIAL_HI_COMMAND, Cat4::SPECIAL_N2_COMMAND);
    FGCModule::set_command_input_button(module_accessor, Cat4::SPECIAL_N2_COMMAND, 2);
    FGCModule::clone_command_input(module_accessor, Cat4::SPECIAL_N_COMMAND, Cat4::ATTACK_COMMAND1);
    FGCModule::set_command_input_button(module_accessor, Cat4::ATTACK_COMMAND1, 2);
    FGCModule::clone_command_input(module_accessor, Cat4::SUPER_SPECIAL2_COMMAND, Cat4::SUPER_SPECIAL_COMMAND);
    FGCModule::set_command_input_button(module_accessor, Cat4::SUPER_SPECIAL_COMMAND, 1);
    FGCModule::set_command_input_button(module_accessor, Cat4::SUPER_SPECIAL2_COMMAND, 2);
}

unsafe extern "C" fn ken_on_frame4(_vtable: u64, fighter: &mut Fighter) {
    let object = &mut fighter.battle_object;
    let module_accessor = (*object).module_accessor;

    // evil?
    if ControlModule::get_command_flag_cat(module_accessor, 3) & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND != 0 {
        FighterControlModuleImpl::reset_special_command_individual(module_accessor, *FIGHTER_PAD_CMD_CAT4_SPECIAL_N2_COMMAND);
    }
    // end of evil

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
    if !VarModule::is_flag(module_accessor, vars::ryu::status::flag::SPECIAL_DECIDE_STRENGTH) {
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
}

unsafe extern "C" fn ken_on_situation_change(_vtable: u64, fighter: &mut Fighter, log: u64) {
    if *(log as *const u8).add(0xc) == 2 {
        return;
    }
    let object = &mut fighter.battle_object;
    let module_accessor = (*object).module_accessor;
    WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_N);
    WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_LW);
    WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_N_HOP);
    WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
    VarModule::off_flag(module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_LW);
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
        &Vector3f{x: pos.x(), y: pos.y(), z: pos.z()},
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

unsafe extern "C" fn ken_on_attack(vtable: u64, fighter: &mut Fighter, log: u64, some_float: f32) {
    let object = &mut fighter.battle_object;
    let module_accessor = (*object).module_accessor;
    let collision_log = log as *mut CollisionLogScuffed;
    let status = StatusModule::status_kind(module_accessor);
    let collision_kind = (*collision_log).collision_kind;
    let opponent_object_id = (*collision_log).opponent_object_id;
    if [
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND
    ].contains(&status) {
        if !VarModule::is_flag(module_accessor, vars::ryu::status::flag::SPECIAL_HI_SPECIAL_EFFECT) {
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

                let eff = if command {
                    Hash40::new("ken_syoryuken_hit2")
                }
                else {
                    Hash40::new("ken_syoryuken_hit")
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
    if status == vars::fighter::status::GUARD_CANCEL_ATTACK {
        if collision_kind == 1 {
            let opponent_object = MiscModule::get_battle_object_from_id(opponent_object_id);
            if (*opponent_object).battle_object_id >> 0x1c == 0
            && HitModule::get_status((*opponent_object).module_accessor, (*collision_log).receiver_id as i32, 0) == 0 {
                syoryuken_eff_handler(module_accessor, collision_log, Hash40::new("ken_syoryuken_hit"), 0.3, 0.1, 0.1, 0.75);
            }
        }
    }
    ken_on_attack_inner(vtable, fighter, log, some_float);
}

unsafe extern "C" fn ken_on_frame5(vtable: u64, fighter: &mut Fighter) {
    ken_on_frame5_inner(vtable, fighter);
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
                VarModule::on_flag(module_accessor, vars::ryu::instance::flag::SKIP_FINAL_PROX_CHECK);
                status = *FIGHTER_RYU_STATUS_KIND_FINAL2;
            }
            else if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND != 0 {
                VarModule::on_flag(module_accessor, vars::ryu::instance::flag::SKIP_FINAL_PROX_CHECK);
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

unsafe extern "C" fn ken_on_status_change(_vtable: u64, fighter: &mut Fighter) {
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
    replace_fighter_vtable(
        *FIGHTER_KIND_KEN,
        FighterVtableSlot::Reset1,
        ken_reset1 as *const () as _
    );
    replace_fighter_vtable(
        *FIGHTER_KIND_KEN,
        FighterVtableSlot::OnFrame4,
        ken_on_frame4 as *const () as _
    );
    replace_fighter_vtable(
        *FIGHTER_KIND_KEN,
        FighterVtableSlot::OnFrame5,
        ken_on_frame5 as *const () as _
    );
    replace_fighter_vtable(
        *FIGHTER_KIND_KEN,
        FighterVtableSlot::OnStatusChange,
        ken_on_status_change as *const () as _
    );
    replace_fighter_vtable(
        *FIGHTER_KIND_KEN,
        FighterVtableSlot::OnSituationChange,
        ken_on_situation_change as *const () as _
    );
    replace_fighter_vtable(
        *FIGHTER_KIND_KEN,
        FighterVtableSlot::OnAttack,
        ken_on_attack as *const () as _
    );
}
