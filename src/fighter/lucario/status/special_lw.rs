use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::{Vector3f, Vector2f},
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*},
    super::super::helper::*
};

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn lucario_special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
        *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn lucario_special_lw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let start_x_spd_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("start_x_spd_mul"));
    let speed_x = speed_x * start_x_spd_mul;
    lua_bind::KineticEnergy::reset_energy(
        stop_energy as *mut smash::app::KineticEnergy,
        *ENERGY_STOP_RESET_TYPE_AIR,
        &Vector2f{x: speed_x, y: 0.0},
        &Vector3f{x: 0.0, y: 0.0, z: 0.0},
        fighter.module_accessor
    );
    lua_bind::KineticEnergy::enable(stop_energy as *mut smash::app::KineticEnergy);
    let gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    let speed_y = if !VarModule::is_flag(fighter.battle_object, lucario::instance::flag::USED_AURA_CHARGE_AIR) {
        0.0
    }
    else {
        KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
    };
    lua_bind::FighterKineticEnergyGravity::set_speed(
        gravity as *mut smash::app::FighterKineticEnergyGravity,
        speed_y
    );
    let fall_acc_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("fall_acc_y"));
    lua_bind::FighterKineticEnergyGravity::set_accel(
        gravity as *mut smash::app::FighterKineticEnergyGravity,
        -fall_acc_y
    );
    let fall_spd_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("fall_spd_max"));
    lua_bind::FighterKineticEnergyGravity::set_limit_speed(
        gravity as *mut smash::app::FighterKineticEnergyGravity,
        fall_spd_max
    );
    lua_bind::KineticEnergy::enable(gravity as *mut smash::app::KineticEnergy);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
    0.into()
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucario_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_lw") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    lucario_special_lw_set_kinetic(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn lucario_special_lw_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        lucario_special_set_air(fighter);
        lucario_special_air_mot_helper(fighter);
    }
    else {
        lucario_special_set_ground(fighter);
        lucario_special_ground_mot_helper(fighter);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RUN_STOP);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            ENERGY_MOTION_RESET_TYPE_GROUND_TRANS,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
}

unsafe extern "C" fn lucario_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, lucario::status::flag::SPECIAL_LW_ENABLE_CANCEL) {
        let cat1 = fighter.global_table[CMD_CAT1].get_i32();
        let cat2 = fighter.global_table[CMD_CAT2].get_i32();
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE != 0
        || cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD != 0 {
            ControlModule::clear_command(fighter.module_accessor, true);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_cancel") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_cancel") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
            lucario_special_lw_set_kinetic(fighter);
            VarModule::off_flag(fighter.battle_object, lucario::status::flag::SPECIAL_LW_ENABLE_CANCEL);
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        lucario_special_lw_set_kinetic(fighter);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_FALL
        }
        else {
            FIGHTER_STATUS_KIND_WAIT
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        lucario_special_lw_pre, lucario_special_lw_init, lucario_special_lw_main
    );
}