use crate::imports::*;

pub unsafe extern "C" fn jack_doyle_set_flags(weapon: &mut L2CWeaponCommon) {
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLAG_FOLLOW_DAMAGE);
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLAG_FOLLOW_DAMAGE_VANISH);
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLAG_FOLLOW_NEXT_DAMAGE_VANISH_END);
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLAG_FOLLOW_VANISH_CANCEL);
    WorkModule::set_int64(weapon.module_accessor, hash40("invalid") as i64, *WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_INT_FOLLOW_MOTION);
}