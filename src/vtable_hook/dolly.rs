use crate::imports::*;

extern "C" {
    #[link_name = "add_go"]
    fn add_go(module_accessor: *mut BattleObjectModuleAccessor, amount: f32);
}

#[repr(C)]
pub struct NoSpecial {
    no: i32,
    special: i32
}

pub unsafe extern "C" fn dolly_handle_special_strength(module_accessor: *mut BattleObjectModuleAccessor, no: i32, special: i32) -> NoSpecial {
    if !WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH) {
        let special_pad_release_w = WorkModule::get_param_int(module_accessor, hash40("param_private"), hash40("special_pad_release_w"));
        let strength = WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH);
        let button_on_timer = WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_BUTTON_ON_TIMER);
        if strength == 0 && ControlModule::get_button(module_accessor) & 3 == 0
        && button_on_timer <= special_pad_release_w {
            WorkModule::set_int(module_accessor, *FIGHTER_DOLLY_STRENGTH_W, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH);
            return NoSpecial{no, special};
        }
    }
    NoSpecial{no: -1, special: -1}
}

#[skyline::hook(offset = 0x971490)]
pub unsafe extern "C" fn dolly_per_frame(_vtable: u64, fighter: &mut Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    let thing = match status {
        0x29 | 0x203 => {
            if !WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH) {
                let special_pad_release_w = WorkModule::get_param_int(module_accessor, hash40("param_private"), hash40("special_pad_release_w"));
                let strength = VarModule::get_int(module_accessor, dolly::status::int::ATTACK_DASH_STRENGTH);
                let button_on_timer = WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_BUTTON_ON_TIMER);
                if strength == 0 && ControlModule::get_button(module_accessor) & 3 == 0
                && button_on_timer <= special_pad_release_w {
                    VarModule::set_int(module_accessor, dolly::status::int::ATTACK_DASH_STRENGTH, *FIGHTER_DOLLY_STRENGTH_W);
                }
            }
            NoSpecial{no: -1, special: -1}
        }
        0x1dc | 0x204 => dolly_handle_special_strength(module_accessor, 0, 0),
        0x1dd | 0x1eb => dolly_handle_special_strength(module_accessor, 0, 1),
        0x1de | 0x1f5 => dolly_handle_special_strength(module_accessor, 0, 2),
        0x1df | 0x1f6 => dolly_handle_special_strength(module_accessor, 0, 3),
        0x1ee | 0x1ef => dolly_handle_special_strength(module_accessor, 2, 1),
        _ => NoSpecial{no: -1, special: -1}
    };
    if thing.no != -1 {
        WorkModule::set_customize_no(module_accessor, thing.no, thing.special);
    }

    let something = if [
        0x1f4, 0x1fa, 0x1fb
    ].contains(&status) {
        true
    }
    else {
        StopModule::is_damage(module_accessor)
    };
    dolly_transition_handler(module_accessor, something as u32, 0);

    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    if *battle_object_slow.add(0x8) != 0 && *(battle_object_slow as *const u32) != 2 {
        return;
    }

    if !WorkModule::is_flag(module_accessor, 0x200000E6)
    || !WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
    || ControlModule::get_command_flag_cat(module_accessor, 0) >> 0xc & 1 == 0
    || dolly_what_is_this(*(module_accessor as *const *const u64).add(0x50 / 8)) & 1 == 0 {
        if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR {
            let something = if PostureModule::lr(module_accessor) == -1.0 {
                0x10
            }
            else {
                0x8
            };
            if *(*(module_accessor as *const *const u64).add(0x48 / 8) as *const u32).add(0xaa8 / 4) & something != 0 {
                if StopModule::is_stop(module_accessor) && StopModule::is_hit(module_accessor) {
                    return;
                }
                WorkModule::inc_int(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_AIR_STICK_BACK_FRAME);
                return;
            }
        }
        WorkModule::set_int(module_accessor, 0, *FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_AIR_STICK_BACK_FRAME);
    }
    else {
        if StopModule::is_stop(module_accessor) && StopModule::is_hit(module_accessor) {
            StopModule::cancel_hit_stop(module_accessor);
        }
        if 1 < SlowModule::frame(module_accessor, 1) {
            SlowModule::clear_immediate(module_accessor, 1, false);
        }
        StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_FINAL, true);
    }
}

#[skyline::from_offset(0x69ae40)]
unsafe extern "C" fn dolly_transition_handler(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32, param_2: u32);

#[skyline::from_offset(0x695c80)]
unsafe extern "C" fn dolly_what_is_this(workmodule: *const u64) -> u32;

#[skyline::hook(offset = 0x971250)]
pub unsafe extern "C" fn dolly_check_super_special(work: u64, _damage: u64) -> u64 {
    let module_accessor = &mut *(*((work as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    // if smashball::is_training_mode() {
    //     return 1;
    // }
    let go_meter = VarModule::get_float(module_accessor, dolly::instance::float::GO_METER);
    // println!("go_meter: {}", go_meter);
    if go_meter >= 100.0 {
        return 1;
    }
    0
}

#[skyline::hook(offset = 0x972db0)]
pub unsafe extern "C" fn dolly_handle_special_command_turnaround(_vtable: u64, fighter: &mut Fighter) {
    let object = &mut fighter.battle_object;
    let module_accessor = object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    let mut reverse_special_input = false;
    let mut command_kind = -1;
    let mut force_reset = false;
    // if 0x1E >= status - 0x1DC {
        match status {
            0x1DC..=0x1E0 => force_reset = true,
            0x1EB => command_kind = 6,
            0x1EF => command_kind = 2,
            0x1F5 => command_kind = 7,
            0x1F6 => command_kind = 3,
            0x1F9 => {
                let cat = WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_CAT4_SPECIAL_COMMAND);
                reverse_special_input = cat & 0x300 == 0x200;
                command_kind = 8;
                if reverse_special_input {
                    command_kind += 1;
                }
            },
            0x1FA => {
                let cat = WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_CAT4_SPECIAL_COMMAND);
                reverse_special_input = cat & 0xC00 == 0x800;
                command_kind = 10;
                if reverse_special_input {
                    command_kind += 1;
                }
            },
            // 0x204 => some_int = 0,
            _ => {}
        }
    // }
    // println!("command: {}", command_kind);
    let lr = PostureModule::lr(module_accessor);
    // println!("lr: {}", lr);
    let mut skip_reset = false;
    if command_kind < 0
    || VarModule::is_flag(module_accessor, dolly::instance::flag::DISABLE_INPUT_SPECIAL_REVERSE) {
        if !force_reset {
            skip_reset = true;
        }
        VarModule::off_flag(module_accessor, dolly::instance::flag::DISABLE_INPUT_SPECIAL_REVERSE);
    }
    else {
        let special_command_lr = ControlModule::get_special_command_lr(module_accessor, command_kind);
        // println!("command LR: {}", special_command_lr);
        let new_lr = if reverse_special_input {
            -special_command_lr
        }
        else {
            special_command_lr
        };
        // println!("command LR post reverse: {}", special_command_lr);
        if new_lr != 0.0 && new_lr != lr {
            PostureModule::set_lr(module_accessor, new_lr);
            PostureModule::update_rot_y_lr(module_accessor);
        }
    }
    if !skip_reset {
        ControlModule::reset_special_command(module_accessor, false);
    }
    WorkModule::off_flag(module_accessor, 0x200000E6);
    WorkModule::off_flag(module_accessor, 0x200000E5);
}

unsafe extern "C" fn dolly_on_attack(vtable: u64, fighter: &mut Fighter, log: u64, damage: f32) {
    let module_accessor = fighter.battle_object.module_accessor;
    let collision_log = log as *mut CollisionLogScuffed;
    let collision_kind = (*collision_log).collision_kind;
    if [1, 2].contains(&collision_kind) {
        let mul = if collision_kind == 2 {
            0.1
        }
        else {
            1.0
        };
        add_go(module_accessor, damage * mul);
    }
    dolly_on_attack_inner(vtable, fighter, log)
}

#[skyline::from_offset(0x9720a0)]
unsafe extern "C" fn dolly_on_attack_inner(vtable: u64, fighter: &mut Fighter, log: u64);

pub fn install() {
    skyline::install_hooks!(
        dolly_per_frame,
        dolly_check_super_special,
        dolly_handle_special_command_turnaround
    );
    MiscModule::patch_vtable_function(0x4fa7a28, dolly_on_attack as u64);
}