use super::*;
use super::super::vl;

unsafe extern "C" fn kirby_attackdash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy!(
        set_speed_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        1.32
    );
    fighter.status_AttackDash()
}

unsafe extern "C" fn kirby_attacklw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackLw3_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_attacklw3_main_loop as *const () as _))
}

unsafe extern "C" fn kirby_attacklw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.global_table[IS_STOP].get_bool()
    && VarModule::is_flag(fighter.module_accessor, vars::kirby::status::flag::ATTACK_LW3_BOUNCE) {
        fighter.change_status(vars::kirby::status::ATTAK_LW3_BOUNCE.into(), false.into());
        return 1.into();
    }

    fighter.status_AttackLw3_Main()
}

unsafe fn get_table_value(table: *mut smash_rs::lib::L2CTable, key: &str) -> smash_rs::lib::L2CValue {
    let hash = if key.starts_with("0x") {
        smash_rs::phx::Hash40::from_hex_str(key).unwrap()
    } else {
        smash_rs::phx::hash40(key)
    };
    (*table).get_map(hash).unwrap().clone()
}

unsafe extern "C" fn kirby_attacklw3_check_attack(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash_rs::lib::L2CTable;
    let kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;

    if [
        *COLLISION_KIND_HIT,
        *COLLISION_KIND_SHIELD
    ].contains(&kind) {
        if !VarModule::is_flag(fighter.module_accessor, vars::kirby::status::flag::ATTACK_LW3_BOUNCE) {
            EffectModule::req_on_joint(
                fighter.module_accessor,
                Hash40::new("kirby_star"),
                Hash40::new("top"),
                &Vector3f{x: 0.0, y: 3.0, z: 6.0},
                &vars::ZERO_VECTOR,
                1.0,
                &vars::ZERO_VECTOR,
                &vars::ZERO_VECTOR,
                false,
                0,
                0,
                0
            );
        }
        if kind == *COLLISION_KIND_HIT {
            VarModule::on_flag(fighter.module_accessor, vars::kirby::status::flag::ATTACK_LW3_HIT);
        }
        VarModule::on_flag(fighter.module_accessor, vars::kirby::status::flag::ATTACK_LW3_BOUNCE);
    }

    fighter.FighterStatusUniqProcessAttackLw3_check_attack(param_2.clone(), param_3.clone())
}

pub unsafe extern "C" fn kirby_attacklw3_bounce_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_3 as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn kirby_attacklw3_bounce_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);

    let rate = if VarModule::is_flag(fighter.module_accessor, vars::kirby::status::flag::ATTACK_LW3_HIT) {
        31.0 / 20.0
    }
    else {
        1.0
    };
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("jump_b"), 22.0, rate, false, 0.0, false, false);

    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    let lr = PostureModule::lr(fighter.module_accessor);
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        -0.5 * lr,
        0.0
    );

    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        1.5
    );

    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_attacklw3_bounce_main_loop as *const () as _))
}

unsafe extern "C" fn kirby_attacklw3_bounce_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    else if MotionModule::frame(fighter.module_accessor) >= vl::param_special_lw::slide_bounce_cancel_frame
    && VarModule::is_flag(fighter.module_accessor, vars::kirby::status::flag::ATTACK_LW3_HIT) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_DASH, kirby_attackdash_main);

    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW3, kirby_attacklw3_main);
    agent.status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_LW3, kirby_attacklw3_check_attack);

    agent.status(Pre, vars::kirby::status::ATTAK_LW3_BOUNCE, kirby_attacklw3_bounce_pre);
    agent.status(Main, vars::kirby::status::ATTAK_LW3_BOUNCE, kirby_attacklw3_bounce_main);
}