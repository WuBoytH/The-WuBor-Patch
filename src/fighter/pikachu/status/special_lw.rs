use crate::imports::status_imports::*;
use super::super::vl;

#[status_script(agent = "pikachu", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn pikachu_special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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

#[status_script(agent = "pikachu", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn pikachu_special_lw_init(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[status_script(agent = "pikachu", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn pikachu_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.module_accessor, pikachu::status::flag::SPECIAL_LW_START);
    VarModule::on_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_LW);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_jump"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_AIR,
            speed_x * vl::special_lw::START_SPEED_X_MUL,
            0.0,
            0.0,
            0.0,
            0.0
        );
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_GROUND,
            speed_x * vl::special_lw::START_SPEED_X_MUL,
            0.0,
            0.0,
            0.0,
            0.0
        );
    }
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    fighter.sub_shift_status_main(L2CValue::Ptr(pikachu_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn pikachu_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, pikachu::status::flag::SPECIAL_LW_START) {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_lw_strike"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            VarModule::off_flag(fighter.module_accessor, pikachu::status::flag::SPECIAL_LW_START);
        }
    }
    else {
        if VarModule::is_flag(fighter.module_accessor, pikachu::status::flag::SPECIAL_LW_ENABLE_GRAVITY) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            VarModule::off_flag(fighter.module_accessor, pikachu::status::flag::SPECIAL_LW_ENABLE_GRAVITY);
        }
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool()
            || fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
        if VarModule::is_flag(fighter.module_accessor, pikachu::status::flag::SPECIAL_LW_ENABLE_LANDING)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            WorkModule::set_float(fighter.module_accessor, 12.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        pikachu_special_lw_pre,
        pikachu_special_lw_init,
        pikachu_special_lw_main
    );
}