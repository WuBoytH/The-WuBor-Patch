use crate::imports::status_imports::*;

#[status_script(agent = "bayonetta", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn bayonetta_attackair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air();
    let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    if mot == hash40("attack_air_lw") {
        if ItemModule::is_have_item(fighter.module_accessor, 0) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("gun_hand") as i64, 0xf62011258);
            ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_KEEP_AIR);
    }
    if mot != hash40("attack_air_n")
    && mot != hash40("attack_air_hi") {
        fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_AttackAir_Main as *const () as _))
    }
    else {
        let loop_mot;
        let param;
        if mot == hash40("attack_air_n") {
            loop_mot = hash40("attack_air_n_hold");
            param = hash40("attack_air_n_loop_max");
        }
        else {
            loop_mot = hash40("attack_air_hi_hold");
            param = hash40("attack_air_hi_loop_max");
        }
        WorkModule::set_int64(fighter.module_accessor, mot as i64, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_INT_MOTION_KIND);
        WorkModule::set_int64(fighter.module_accessor, loop_mot as i64, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_INT_LOOP_MOTION_KIND);
        let loop_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), param);
        WorkModule::set_int(fighter.module_accessor, loop_max, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_INT_LOOP_COUNT);
        fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_attackair_main_loop as *const () as _))
    }
}

unsafe extern "C" fn bayonetta_attackair_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
        VarModule::off_flag(fighter.module_accessor, attack_air::flag::WHIFF);
    }
    else if !VarModule::is_flag(fighter.module_accessor, attack_air::flag::WHIFF) {
        let part_size = AttackModule::part_size(fighter.module_accessor) as i32;
        for id in 0..part_size {
            if AttackModule::is_attack(fighter.module_accessor, id, false) {
                VarModule::on_flag(fighter.module_accessor, attack_air::flag::WHIFF);
                break;
            }
        }
    }
    let step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_INT_STEP);
    let mut check_end = true;
    if step != *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_STEP_START {
        if step == *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_STEP_LOOP_START {
            if !StopModule::is_stop(fighter.module_accessor) {
                bayonetta_attackair_loop_helper(fighter, false.into());
            }
            fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(bayonetta_attackair_loop_helper as *const () as _));
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_STEP_LOOP, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_INT_STEP);
        }
        let step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_INT_STEP);
        if step == *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_STEP_LOOP {
            check_end = false;
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_FLAG_CONTINUE) {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_FLAG_LOOPED) {
                    let loop_mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_INT_LOOP_MOTION_KIND);
                    MotionModule::change_motion(
                        fighter.module_accessor,
                        Hash40::new_raw(loop_mot),
                        0.0,
                        1.0,
                        false,
                        0.0,
                        false,
                        false
                    );
                    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_FLAG_LOOPED);
                }
            }
            else {
                let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_INT_MOTION_KIND);
                let restart = WorkModule::get_float(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_FLOAT_MOTION_RESTART_FRAME);
                MotionModule::change_motion_force_inherit_frame(
                    fighter.module_accessor,
                    Hash40::new_raw(mot),
                    restart,
                    1.0,
                    0.0
                );
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_STEP_END, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_INT_STEP);
            }
        }
    }
    let landing = if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_KEEP_AIR) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.sub_transition_group_check_air_landing().get_bool()
        }
        else {
            let status = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING) {
                FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR
            }
            else {
                FIGHTER_STATUS_KIND_LANDING
            };
            fighter.change_status(status.into(), false.into());
            true
        }
    }
    else {
        false
    };
    if !landing {
        if CancelModule::is_enable_cancel(fighter.module_accessor)
        && fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
        if CustomCancelManager::execute_cancel(fighter) {
            return 0.into();
        }
        if check_end && MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
    }
    if landing {
        fighter.sub_air_check_superleaf_fall_slowly();
        if !fighter.global_table[IS_STOP].get_bool() {
            fighter.sub_attack_air_uniq_process_exec_fix_pos();
        }
    }
    0.into()
}

unsafe extern "C" fn bayonetta_attackair_loop_helper(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    fighter.attack_air_uniq(param_1.clone());
    if !param_1.get_bool() {
        if MotionModule::is_end(fighter.module_accessor) {
            let step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP);
            if step != *FIGHTER_BAYONETTA_SHOOTING_STEP_SHOOTING {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_FLAG_CONTINUE);
            }
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_INT_LOOP_COUNT);
            let loops = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_INT_LOOP_COUNT);
            if loops <= 0 {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_FLAG_CONTINUE);
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_FLAG_CONTINUE) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_FLAG_LOOPED);
            }
        }
    }
    0.into()
}

#[status_script(agent = "bayonetta", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn bayonetta_attackair_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F
    && status != *FIGHTER_STATUS_KIND_ATTACK_AIR {
        FGCModule::reset_used_aerials(fighter);
    }
    0.into()
}

#[status_script(agent = "bayonetta", status = FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn bayonetta_attackairf_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_uniq_process_init()
}

#[status_script(agent = "bayonetta", status = FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn bayonetta_attackairf_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_STATUS_KIND_ATTACK_AIR {
        FGCModule::reset_used_aerials(fighter);
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, bayonetta_attackair_main);
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_ATTACK_AIR, bayonetta_attackair_end);

    agent.status(smashline::Init, *FIGHTER_STATUS_KIND_ATTACK_AIR, bayonetta_attackairf_init);
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_ATTACK_AIR, bayonetta_attackairf_end);
}