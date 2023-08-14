use crate::imports::status_imports::*;

#[status("richter", FIGHTER_STATUS_KIND_ATTACK_AIR)]
unsafe fn richter_attack_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_uniq_process_exec();
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw")
    && !VarModule::is_flag(fighter.battle_object, richter::status::flag::ATTACK_AIR_LW_IGNORE_BOUNCE)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_HIT) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            VarModule::on_flag(fighter.battle_object, richter::status::flag::ATTACK_AIR_LW_IGNORE_BOUNCE);
        }
        else {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("fall_leaning_c"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::set_int64(fighter.module_accessor, hash40("fall_leaning_c") as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP);
        }
    }
    0.into()
}

pub fn install() {
    richter_attack_air_exec::install();
}