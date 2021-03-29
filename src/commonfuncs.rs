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
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_BURY {
        return true;
    }
    else {
        return false;
    }
}