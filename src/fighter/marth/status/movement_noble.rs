use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    wubor_utils::table_const::*,
    custom_status::*,
    super::{
        helper::*,
        super::{vars::*, vl}
    }
};

// Note to self: Implement PARRY_XLU when dash anims are done

// Used for both dashes

unsafe extern "C" fn marth_speciallw_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
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
        0,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_DASH_F

unsafe extern "C" fn marth_speciallw_dash_f_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::clear_command_one(
        fighter.module_accessor,
        *FIGHTER_PAD_COMMAND_CATEGORY1,
        *FIGHTER_PAD_CMD_CAT1_DASH
    );
    sv_kinetic_energy!(
        set_speed_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        0.5
    );
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("final_dash_end"),
        0.0,
        2.5,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_dash_main_loop as *const () as _))
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_DASH_B

unsafe extern "C" fn marth_speciallw_dash_b_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::clear_command_one(
        fighter.module_accessor,
        *FIGHTER_PAD_COMMAND_CATEGORY1,
        *FIGHTER_PAD_CMD_CAT1_TURN_DASH
    );
    sv_kinetic_energy!(
        set_speed_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        0.5
    );
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("escape_b"),
        0.0,
        2.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_dash_main_loop as *const () as _))
}

// Dash Main Loop

unsafe extern "C" fn marth_speciallw_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if fighter.global_table[MOTION_FRAME].get_f32() >= vl::param_stance::dash_attack_cancel_frame {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
        if marth_stance_cancel_helper(fighter).get_bool() {
            return 1.into();
        }
        if fighter.global_table[MOTION_FRAME].get_f32() >= vl::param_stance::dash_redash_frame {
            if marth_stance_dash_cancel_helper(fighter, true).get_bool() {
                return 1.into();
            }
        }
        marth_stance_mot_end_helper(fighter);
    }
    0.into()
}

// Dash End

unsafe extern "C" fn marth_speciallw_dash_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU);
    marth_stance_common_end(fighter);
    0.into()
}

pub fn install() {
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_DASH_F,
        StatusInfo::new()
            .with_pre(marth_speciallw_dash_pre)
            .with_main(marth_speciallw_dash_f_main)
            .with_end(marth_speciallw_dash_end)
    );
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_DASH_B,
        StatusInfo::new()
            .with_pre(marth_speciallw_dash_pre)
            .with_main(marth_speciallw_dash_b_main)
            .with_end(marth_speciallw_dash_end)
    );
}