use super::*;

unsafe extern "C" fn simon_attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_HI);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_HI_BACK_CLIFF);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FIGHTER_SEARCH_HIT);
    let kind = fighter.sub_attack_air_kind_set_log_info();

    let angle_check : fn(ret: &mut L2CValue, fighter: &mut L2CFighterCommon, kind: L2CValue) = std::mem::transmute(
        smashline::api::get_target_function("lua2cpp_simon.nrs", 0xb230).unwrap()
    );
    let ret = &mut L2CValue::U64(0);
    angle_check(ret, fighter, kind);
    let mut new_kind = ret.get_u64();
    match new_kind {
        0xc33f869bc => {
            PostureModule::reverse_lr(fighter.module_accessor);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
            new_kind = hash40("attack_air_f");
        },
        0xfcaf6254b => {
            PostureModule::reverse_lr(fighter.module_accessor);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
            new_kind = hash40("attack_air_f_hi");
        },
        0xf5495dd2c => {
            PostureModule::reverse_lr(fighter.module_accessor);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
            new_kind = hash40("attack_air_f_lw");
        },
        _ => {}
    };

    if new_kind == hash40("attack_air_hi") {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_HI);
    }

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(new_kind),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    WorkModule::set_int64(fighter.module_accessor, new_kind as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);

    fighter.sub_attack_air_common(false.into());

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_DAMAGE);

    let simon_attack_air_main_loop : fn(fighter: &mut L2CFighterCommon) -> L2CValue = std::mem::transmute(
        smashline::api::get_target_function("lua2cpp_simon.nrs", 0xb940).unwrap()
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(simon_attack_air_main_loop as *const () as _))
}

unsafe extern "C" fn simon_attack_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_uniq_process_exec()
}

unsafe extern "C" fn simon_attack_air_check_attack(_fighter: &mut L2CFighterCommon, _param_1: &L2CValue, _param_2: &L2CValue) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, simon_attack_air_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_ATTACK_AIR, simon_attack_air_exec);
    agent.status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_AIR, simon_attack_air_check_attack);
}