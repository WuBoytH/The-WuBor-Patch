use crate::imports::status_imports::*;

#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_LANDING_LIGHT, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn pikmin_landinglight_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    pikmin_landing_init_inner(fighter)
}

#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_LANDING_LIGHT, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn pikmin_landinglight_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    pikmin_landing_exit_inner(fighter)
}

#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn pikmin_landing_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    pikmin_landing_init_inner(fighter)
}

#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
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

#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn pikmin_landing_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    pikmin_landing_exit_inner(fighter)
}

#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn pikmin_landingattackair_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    pikmin_landing_exit_inner(fighter)
}

#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
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
    install_status_scripts!(
        pikmin_landinglight_init,
        pikmin_landinglight_exit,

        pikmin_landing_init,
        pikmin_landing_exec,
        pikmin_landing_exit,

        pikmin_landingattackair_exit,

        pikmin_landingfallspecial_exit
    );
}