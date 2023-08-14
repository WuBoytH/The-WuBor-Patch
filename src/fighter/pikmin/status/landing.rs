use crate::imports::status_imports::*;

#[status("pikmin", FIGHTER_STATUS_KIND_LANDING_LIGHT)]
unsafe fn pikmin_landinglight_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    pikmin_landing_init_inner(fighter)
}

#[status("pikmin", FIGHTER_STATUS_KIND_LANDING_LIGHT)]
unsafe fn pikmin_landinglight_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    pikmin_landing_exit_inner(fighter)
}

#[status("pikmin", FIGHTER_STATUS_KIND_LANDING)]
unsafe fn pikmin_landing_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    pikmin_landing_init_inner(fighter)
}

#[status("pikmin", FIGHTER_STATUS_KIND_LANDING)]
unsafe fn pikmin_landing_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_landing_uniq_process_exec();
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_LANDING {
        if VarModule::is_flag(fighter.battle_object, pikmin::instance::flag::ATTACK_HI3_LANDING) {
            if MotionModule::frame(fighter.module_accessor) < 20.0 {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_LANDING_FLAG_STIFF_CANCEL);
            }
        }
    }
    0.into()
}

#[status("pikmin", FIGHTER_STATUS_KIND_LANDING)]
unsafe fn pikmin_landing_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    pikmin_landing_exit_inner(fighter)
}

#[status("pikmin", FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR)]
unsafe fn pikmin_landingattackair_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    pikmin_landing_exit_inner(fighter)
}

#[status("pikmin", FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL)]
unsafe fn pikmin_landingfallspecial_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    pikmin_landing_exit_inner(fighter)
}

unsafe fn pikmin_landing_init_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, pikmin::instance::flag::ATTACK_HI3_LANDING) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_TURN);
    }
    fighter.sub_landing_uniq_process_init();
    0.into()
}

unsafe fn pikmin_landing_exit_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, pikmin::instance::flag::ATTACK_HI3_LANDING);
    fighter.sub_landing_uniq_process_exit();
    0.into()
}

pub fn install() {
    pikmin_landinglight_init::install();
    pikmin_landinglight_exit::install();
    pikmin_landing_init::install();
    pikmin_landing_exec::install();
    pikmin_landing_exit::install();
    pikmin_landingattackair_exit::install();
    pikmin_landingfallspecial_exit::install();
}