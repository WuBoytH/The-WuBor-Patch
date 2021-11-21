use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    crate::{
        vars::*,
        table_const::*,
        fighter::ganon::helper::*
    },
    super::vl::*
};

#[status_script(agent = "kirby", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn kirby_attacklw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackLw3_common();
    if !StopModule::is_stop(fighter.module_accessor) {
        kirby_attacklw3_main_stop(fighter);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(kirby_attacklw3_main_stop as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_attacklw3_main_loop as *const () as _))
}

unsafe extern "C" fn kirby_attacklw3_main_stop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_KIRBY_STATUS_ATTACK_LW3_FLAG_BOUNCE) {
        if !StopModule::is_stop(fighter.module_accessor) {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                macros::EFFECT(fighter, Hash40::new("kirby_star"), Hash40::new("top"), 0, 3, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
                GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                // macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("jump_b"), 22.0, 31.0 / 20.0, false, 0.0, false, false);
                macros::SET_SPEED_EX(fighter, -1.0, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_KIRBY_STATUS_ATTACK_LW3_FLAG_BOUNCE);
            }
        }
    }
    else if MotionModule::frame(fighter.module_accessor) >= slide_bounce_cancel_frame {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    return 0.into();
}

unsafe extern "C" fn kirby_attacklw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_KIRBY_STATUS_ATTACK_LW3_FLAG_BOUNCE) {
        return fighter.status_AttackLw3_Main();
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
        else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}

#[status_script(agent = "kirby", status = FIGHTER_KIRBY_STATUS_KIND_GANON_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn kirby_ganon_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        FighterMotionModuleImpl::change_motion_kirby_copy(
            fighter.module_accessor,
            Hash40::new("special_n"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        FighterMotionModuleImpl::change_motion_kirby_copy(
            fighter.module_accessor,
            Hash40::new("special_air_n"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_ganon_specialn_main_loop as *const () as _))
}

unsafe extern "C" fn kirby_ganon_specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_GANON_STATUS_WORK_ID_INT_TELEPORT_STEP) == FIGHTER_GANON_TELEPORT_STEP_INIT {
        deception_init(fighter);
    }
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_GANON_STATUS_WORK_ID_INT_TELEPORT_STEP) == FIGHTER_GANON_TELEPORT_STEP_CHECK_FEINT {
        deception_feint_handler(fighter);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GANON_STATUS_WORK_ID_FLAG_TELEPORT_STOP) {
        KineticModule::unable_energy_all(fighter.module_accessor);
    }
    let curr_sit = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_sit = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if curr_sit != prev_sit {
        if curr_sit == *SITUATION_KIND_GROUND {
            FighterMotionModuleImpl::change_motion_inherit_frame_kirby_copy(
                fighter.module_accessor,
                Hash40::new("special_n"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
        else {
            FighterMotionModuleImpl::change_motion_inherit_frame_kirby_copy(
                fighter.module_accessor,
                Hash40::new("special_air_n"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
    }
    if MotionModule::frame(fighter.module_accessor) >= 65.0 {
        if curr_sit == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        kirby_attacklw3_main,
        kirby_ganon_specialn_main
    );
}