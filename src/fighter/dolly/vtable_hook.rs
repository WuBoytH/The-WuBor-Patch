use {
    smash::{
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*}
};

#[skyline::hook(offset = 0x971230)]
pub unsafe extern "C" fn dolly_check_super_special(work: u64, _damage: u64) -> u64 {
    let module_accessor = &mut *(*((work as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    if smashball::is_training_mode() {
        return 1;
    }
    let object_id = module_accessor.battle_object_id;
    let object = MiscModule::get_battle_object_from_id(object_id);
    let go_meter = VarModule::get_float(object, dolly::instance::float::GO_METER);
    // println!("go_meter: {}", go_meter);
    if go_meter >= 100.0 {
        return 1;
    }
    0
}

#[skyline::hook(offset = 0x970fd0)]
pub unsafe extern "C" fn dolly_check_super_special_pre(module_accessor: *mut BattleObjectModuleAccessor, param_2: u8) {
    original!()(module_accessor, param_2)
}

#[skyline::hook(offset = 0x972d90)]
pub unsafe extern "C" fn dolly_handle_special_command_turnaround(_vtable: u64, fighter: &mut Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    let mut some_bool = false;
    let mut some_int = -1;
    let mut some_bool2 = false;
    if 0x1E >= status - 0x1DC {
        match status {
            0x29 => some_int = 1,
            0x1DC..=0x1E0 => some_bool2 = true,
            0x1EB => some_int = 0,
            0x1EF => some_int = 2,
            0x1F5 => some_int = 7,
            0x1F6 => some_int = 3,
            0x1F9 => {
                let cat = WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_CAT4_SPECIAL_COMMAND);
                some_bool = cat & 0x300 == 0x200;
                some_int = 8
            },
            0x1FA => {
                let cat = WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_CAT4_SPECIAL_COMMAND);
                some_bool = cat & 0xC00 == 0x800;
                some_int = 10;
                if some_bool {
                    some_int += 1;
                }
            },
            _ => {}
        }
    }
    let lr = PostureModule::lr(module_accessor);
    let mut skip_reset = false;
    if some_int < 0 {
        if !some_bool2 {
            skip_reset = true;
        }
    }
    else {
        let special_command_lr = ControlModule::get_special_command_lr(module_accessor, some_int);
        let new_lr = if some_bool {
            -special_command_lr
        }
        else {
            special_command_lr
        };
        if new_lr != 0.0 && new_lr != special_command_lr {
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

pub fn install() {
    skyline::install_hooks!(
        dolly_check_super_special,
        dolly_check_super_special_pre,
        dolly_handle_special_command_turnaround
    );
}