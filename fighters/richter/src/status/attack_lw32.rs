use super::*;

unsafe extern "C" fn richter_attack_lw32_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("attack_lw32"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    fighter.sub_shift_status_main(L2CValue::Ptr(richter_attack_lw32_main_loop as *const () as _))
}

unsafe extern "C" fn richter_attack_lw32_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32_LANDING
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
        return 1.into();
    }

    let no_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("attack_lw32_no_landing_frame"));
    let frame = MotionModule::frame(fighter.module_accessor);
    if frame >= no_landing_frame as f32 {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32_LANDING.into(), false.into());
            return 1.into();
        }
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_LW32_WORK_ID_FLAG_LANDING_AIR) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_LW32_WORK_ID_FLAG_LANDING_AIR);
        let speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("attack_lw32_motion_speed_mul"));
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            ENERGY_MOTION_RESET_TYPE_AIR_TRANS,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            set_speed_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            speed_mul
        );
    }

    0.into()
}

unsafe extern "C" fn attack_lw32_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_HIT)
    && !fighter.global_table[IS_STOP].get_bool() {
        fighter.change_status(vars::richter::status::SLIDE_BOUNCE.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn attack_lw32_check_attack(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash_rs::lib::L2CTable;
    let kind = MiscModule::get_table_value(table, "kind_").try_integer().unwrap() as i32;

    if ![
        *COLLISION_KIND_SHIELD,
    ].contains(&kind) {
        VarModule::on_flag(fighter.module_accessor, vars::richter::status::flag::SLIDE_BOUNCE_IS_HIT);
    }

    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_HIT);

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32, richter_attack_lw32_main);
    agent.status(Exec, *FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32, attack_lw32_exec);
    agent.status(CheckAttack, *FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32, attack_lw32_check_attack);
}