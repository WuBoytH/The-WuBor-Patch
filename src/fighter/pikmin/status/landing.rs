use crate::imports::*;

unsafe extern "C" fn pikmin_landing_light_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    pikmin_landing_init_inner(fighter)
}

unsafe extern "C" fn pikmin_landing_light_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    pikmin_landing_exit_inner(fighter)
}

unsafe extern "C" fn pikmin_landing_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    pikmin_landing_init_inner(fighter)
}

unsafe extern "C" fn pikmin_landing_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_landing_uniq_process_exec();
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_LANDING {
        if VarModule::is_flag(fighter.module_accessor, pikmin::instance::flag::ATTACK_HI3_LANDING) {
            if MotionModule::frame(fighter.module_accessor) < 20.0 {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_LANDING_FLAG_STIFF_CANCEL);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn pikmin_landing_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    pikmin_landing_exit_inner(fighter)
}

unsafe extern "C" fn pikmin_landing_attack_air_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    pikmin_landing_exit_inner(fighter)
}

unsafe extern "C" fn pikmin_landing_fall_special_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    pikmin_landing_exit_inner(fighter)
}

unsafe extern "C" fn pikmin_landing_init_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, pikmin::instance::flag::ATTACK_HI3_LANDING) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_TURN);
    }
    fighter.sub_landing_uniq_process_init();
    0.into()
}

unsafe extern "C" fn pikmin_landing_exit_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.module_accessor, pikmin::instance::flag::ATTACK_HI3_LANDING);
    fighter.sub_landing_uniq_process_exit();
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_LANDING_LIGHT, pikmin_landing_light_init);
    agent.status(Exit, *FIGHTER_STATUS_KIND_LANDING_LIGHT, pikmin_landing_light_exit);

    agent.status(Init, *FIGHTER_STATUS_KIND_LANDING, pikmin_landing_init);
    agent.status(Exec, *FIGHTER_STATUS_KIND_LANDING, pikmin_landing_exec);
    agent.status(Exit, *FIGHTER_STATUS_KIND_LANDING, pikmin_landing_exit);

    agent.status(Exit, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, pikmin_landing_attack_air_exit);

    agent.status(Exit, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, pikmin_landing_fall_special_exit);
}