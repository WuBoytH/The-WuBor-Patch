use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::*,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*
};

#[status_script(agent = "simon", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn simon_attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    let new_mot = if motion == hash40("attack_air_b") { hash40("attack_air_f") }
    else if motion == hash40("attack_air_b_hi") { hash40("attack_air_f_hi") }
    else if motion == hash40("attack_air_b_lw") { hash40("attack_air_f_lw") }
    else { motion };
    if new_mot != motion {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new_raw(new_mot),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::set_int64(fighter.module_accessor, new_mot as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    }
    ret
}

#[status_script(agent = "simon", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn simon_attack_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_uniq_process_exec()
}

#[status_script(agent = "simon", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_CHECK_ATTACK)]
unsafe fn simon_attack_air_check_attack(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    install_status_scripts!(
        simon_attack_air_main, simon_attack_air_exec, simon_attack_air_check_attack
    );
}