use crate::imports::status_imports::*;

#[skyline::hook(offset = 0x10d4550)]
unsafe extern "C" fn ryu_ken_init(vtable: u64, fighter: &mut Fighter) {
    original!()(vtable, fighter);
    let module_accessor = fighter.battle_object.module_accessor;
    if fighter.battle_object.kind != 0x3d {
        FGCModule::set_command_input_button(module_accessor, 1, 2);
    }
    FGCModule::set_command_input_button(module_accessor, 0, 2);
    FGCModule::set_command_input_button(module_accessor, 2, 2);
    FGCModule::set_command_input_button(module_accessor, 3, 2);
    FGCModule::set_command_input_button(module_accessor, 7, 2);
    FGCModule::set_command_input_button(module_accessor, 8, 2);
    FGCModule::set_command_input_button(module_accessor, 9, 2);
    FGCModule::set_command_input_button(module_accessor, 10, 2);
    FGCModule::set_command_input_button(module_accessor, 11, 2);
}

#[skyline::from_offset(0x646fc0)]
extern "C" fn what_is_this(fighter: &mut Fighter) -> *const u64;

#[skyline::from_offset(0x6da330)]
extern "C" fn what_is_this_2(param_1: i32, param_2: i32, param_3: u64, module_accessor: *mut BattleObjectModuleAccessor, huh: *const u64);

#[skyline::from_offset(0x69ae20)]
extern "C" fn ryu_ken_transition_handler(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32, param_2: u32);

#[skyline::hook(offset = 0x10d4dd0)]
unsafe extern "C" fn ryu_ken_move_strength_autoturn_handler(_vtable: u64, fighter: &mut Fighter) {
    let object = &mut fighter.battle_object;
    let module_accessor = (*object).module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    let mut transition_bool = 0;
    if status - 0x1dc < 0x1a {
        transition_bool = 1;
        match status {
            0x1dc | 0x1eb | 0x1ec => ryu_ken_handle_special_strength(object, hash40("param_special_n")),
            0x1dd | 0x1ef => ryu_ken_handle_special_strength(object, hash40("param_special_s")),
            0x1de | 0x1f4 =>
                if !WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH) {
                    ryu_ken_handle_special_strength(object, hash40("param_special_hi"));
                },
            0x1df | 0x1f5 => transition_bool = 0,
            _ => ()
        }
    }
    match status {
        0x27 => ryu_ken_handle_light_normals(fighter, hash40("attack_11_s"), hash40("attack_11_w")),
        0x2a => ryu_ken_handle_light_normals(fighter, hash40("attack_s3_s_s"), hash40("attack_s3_s_w")),
        0x2b => ryu_ken_handle_light_normals(fighter, hash40("attack_hi3_s"), hash40("attack_hi3_w")),
        0x2c => ryu_ken_handle_light_normals(fighter, hash40("attack_lw3_s"), hash40("attack_lw3_w")),
        _ => ()
    };
    if transition_bool == 0 {
        transition_bool = StopModule::is_damage(module_accessor) as u32;
    }
    ryu_ken_transition_handler(module_accessor, transition_bool & 1, 0);
    // The rest of the function is proximity normal nonsense I don't want.
}

unsafe extern "C" fn ryu_ken_handle_special_strength(object: &mut BattleObject, param_parent: u64) {
    let module_accessor = (*object).module_accessor;
    if VarModule::is_flag(object, ryu::status::flag::SPECIAL_DECIDE_STRENGTH) {
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

#[skyline::hook(offset = 0x10d5a60)]
unsafe extern "C" fn ryu_ken_handle_light_normals(fighter: &mut Fighter, heavy_motion: u64, light_motion: u64) {
    let module_accessor = fighter.battle_object.module_accessor;
    let mot = MotionModule::motion_kind(module_accessor);
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    if !StopModule::is_stop(module_accessor) && !SlowModule::is_skip(module_accessor)
    && (*battle_object_slow.add(0x8) == 0 || *(battle_object_slow as *const u32) == 2) {
        if !WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK) {
            if (mot ^ heavy_motion) & 0xffffffffff == 0 {
                let button = ControlModule::get_button(module_accessor);
                let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
                if button & 1 == 0
                && (cat1 >> 0x15 & 1 == 0 || !WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT)) {
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
            if (cat1 >> 0x15 & 1 == 0 || !WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT))
            && ControlModule::get_command_flag_cat(module_accessor, 3) == 0 {

                let huh = what_is_this(fighter);

                what_is_this_2(2, something, 1, module_accessor, huh);
            }
            WorkModule::on_flag(module_accessor, 0x21000022 + 7);
        }
    }
}

pub fn install() {
    // Patches out the removal of unused command input classes for Ryu and Ken
    skyline::patching::Patch::in_text(0x10d45a4).data(0x14000014u32);

    skyline::install_hooks!(
        ryu_ken_init,
        ryu_ken_handle_light_normals
    );
}