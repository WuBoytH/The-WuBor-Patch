use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;

pub unsafe fn is_damage_check(boma : &mut BattleObjectModuleAccessor) -> bool {
    if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_AIR
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_THROWN
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CAPTURE_WAIT
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FLY 
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR 
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U 
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FALL
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_FINAL
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SLEEP
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ESCAPE_B
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ESCAPE_F
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ESCAPE
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CLIFF_ESCAPE
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ESCAPE_AIR
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SWALLOWED
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_AIR_LASSO
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CATCHED_REFLET
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CATCHED_RIDLEY
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_AIR
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_MISS_FOOT
    || WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI)
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DEAD
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_BURY
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_BURY_WAIT
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ICE {
        return true;
    }
    else {
        return false;
    }
}

pub unsafe fn get_player_number(module_accessor:  &mut BattleObjectModuleAccessor) -> usize {
    if utility::get_kind(module_accessor) == *WEAPON_KIND_PTRAINER_PTRAINER {
        let player_number = WorkModule::get_int(module_accessor, *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_INT_FIGHTER_ENTRY_ID) as usize;
        return player_number;
    }
    else if utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        let player_number = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        return player_number;
    }
    else {
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let player_number = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        return player_number;
    }
}

pub unsafe fn get_command_stick_direction(module_accessor: &mut BattleObjectModuleAccessor, command: bool) -> i32 {
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