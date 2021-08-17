use smash::{
    app::{lua_bind::*, *},
    lib::lua_const::*
};

pub unsafe fn is_damage_check(module_accessor : *mut BattleObjectModuleAccessor) -> bool {
    if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI)
    || [
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_THROWN,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_FINAL,
        *FIGHTER_STATUS_KIND_SLEEP,
        *FIGHTER_STATUS_KIND_ESCAPE_B,
        *FIGHTER_STATUS_KIND_ESCAPE_F,
        *FIGHTER_STATUS_KIND_ESCAPE,
        *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
        *FIGHTER_STATUS_KIND_ESCAPE_AIR,
        *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE,
        *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND,
        *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD,
        *FIGHTER_STATUS_KIND_SWALLOWED,
        *FIGHTER_STATUS_KIND_AIR_LASSO,
        *FIGHTER_STATUS_KIND_CATCHED_REFLET,
        *FIGHTER_STATUS_KIND_CATCHED_RIDLEY,
        *FIGHTER_STATUS_KIND_MISS_FOOT,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_BURY,
        *FIGHTER_STATUS_KIND_BURY_WAIT,
        *FIGHTER_STATUS_KIND_ICE,
        *FIGHTER_STATUS_KIND_DOWN_DAMAGE,
        *FIGHTER_STATUS_KIND_DOWN_STAND_FB,
        *FIGHTER_STATUS_KIND_DOWN_STAND,
        *FIGHTER_STATUS_KIND_DOWN_WAIT,
        *FIGHTER_STATUS_KIND_DOWN_EAT,
        *FIGHTER_STATUS_KIND_LAY_DOWN,
        *FIGHTER_STATUS_KIND_DOWN,
        *FIGHTER_STATUS_KIND_DOWN_SPOT,
        *FIGHTER_STATUS_KIND_PASSIVE,
        *FIGHTER_STATUS_KIND_PASSIVE_WALL,
        *FIGHTER_STATUS_KIND_PASSIVE_CEIL,
        *FIGHTER_STATUS_KIND_PASSIVE_FB,
        *FIGHTER_STATUS_KIND_CATCHED_REFLET,
        *FIGHTER_STATUS_KIND_CATCHED_RIDLEY,
        *FIGHTER_STATUS_KIND_CATCHED_GANON,
        *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON,
        *FIGHTER_STATUS_KIND_CATCHED_AIR_FALL_GANON,
        *FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON
    ].contains(&StatusModule::status_kind(module_accessor)) {
        return true;
    }
    else {
        return false;
    }
}

pub unsafe fn entry_id(module_accessor: *mut BattleObjectModuleAccessor) -> usize {
    if utility::get_kind(&mut *module_accessor) == *WEAPON_KIND_PTRAINER_PTRAINER {
        return WorkModule::get_int(module_accessor, *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_INT_FIGHTER_ENTRY_ID) as usize;
    }
    else if utility::get_category(&mut *module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        return WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }
    else {
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        return WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }
}

pub unsafe fn get_command_stick_direction(module_accessor: *mut BattleObjectModuleAccessor, command: bool) -> i32 {
    let status_kind = StatusModule::status_kind(module_accessor);
    let mut stick_x = ControlModule::get_stick_x(module_accessor);
    if command {
        stick_x = stick_x * PostureModule::lr(module_accessor);
        if status_kind == *FIGHTER_STATUS_KIND_TURN_RUN {
            stick_x *= -1.0;
        }
    }

    if stick_x >= 0.4 {
        if ControlModule::get_stick_y(module_accessor) <= -0.4 {
            return 3;
        }
        else if ControlModule::get_stick_y(module_accessor) >= 0.4 {
            return 9;
        }
        else {
            return 6;
        }
    }
    else if stick_x <= -0.4 {
        if ControlModule::get_stick_y(module_accessor) <= -0.4 {
            return 1;
        }
        else if ControlModule::get_stick_y(module_accessor) >= 0.4 {
            return 7;
        }
        else {
            return 4;
        }
    }
    else {
        if ControlModule::get_stick_y(module_accessor) <= -0.4 {
            return 2;
        }
        else if ControlModule::get_stick_y(module_accessor) >= 0.4 {
            return 8;
        }
        else {
            return 5;
        }
    }
}

// pub unsafe fn hit_cancel(module_accessor: *mut BattleObjectModuleAccessor) {
//     if ControlModule::check_button_on_trriger(module_accessor, *CONTROL_PAD_BUTTON_) {

//     }
// }