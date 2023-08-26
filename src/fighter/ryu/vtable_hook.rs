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
                    MotionModule::change_motion_inherit_frame(
                        module_accessor,
                        Hash40::new_raw(light_motion),
                        -1.0,
                        1.0,
                        0.0,
                        false,
                        false
                    );
                    // MotionModule::change_motion(
                    //     module_accessor,
                    //     Hash40::new_raw(light_motion),
                    //     0.0,
                    //     1.0,
                    //     false,
                    //     0.0,
                    //     false,
                    //     false
                    // );
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

    // Patches the check that enables proximity normals
    skyline::patching::Patch::in_text(0x10d51bc).data(0x1400021Au32);

    skyline::install_hooks!(
        ryu_ken_init,
        ryu_ken_handle_light_normals
    );
}