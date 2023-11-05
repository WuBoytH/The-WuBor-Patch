use crate::imports::status_imports::*;
use super::helper::*;

#[status_script(agent = "gaogaen", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn gaogaen_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.module_accessor, gaogaen::status::flag::REVENGE_AUTO);
    let mot;
    let correct;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP;
        mot = hash40("special_lw_start");
    }
    else {
        correct = *GROUND_CORRECT_KIND_AIR;
        mot = hash40("special_air_lw_start");
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(mot),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    gaogaen_special_lw_kinetic_helper(fighter, true.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(gaogaen_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn gaogaen_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if VarModule::is_flag(fighter.module_accessor, gaogaen::status::flag::REVENGE_AUTO) {
        fighter.change_status(FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE_START) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE_START);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE);
        let special_lw_hit_damage_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_hit_damage_mul"));
        let special_lw_hit_stop_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_hit_stop_mul"));
        DamageModule::set_damage_mul_2nd(fighter.module_accessor, special_lw_hit_damage_mul);
        HitModule::set_hit_stop_mul(fighter.module_accessor, special_lw_hit_stop_mul, HitStopMulTarget{_address: 3}, 0.0);
        DamageModule::set_no_reaction_mode_status(fighter.module_accessor, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_ALWAYS as u8}, -1.0, -1.0, -1);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE_END) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE_END);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE);
        DamageModule::set_no_reaction_mode_status(fighter.module_accessor, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8}, -1.0, -1.0, -1);
        DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
        HitModule::set_hit_stop_mul(fighter.module_accessor, 1.0, HitStopMulTarget{_address: 3}, 0.0);
    }
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if situation == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
        return 0.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        let mot;
        let correct;
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP;
            mot = hash40("special_lw_start");
        }
        else {
            correct = *GROUND_CORRECT_KIND_AIR;
            mot = hash40("special_air_lw_start");
        }
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
        gaogaen_special_lw_kinetic_helper(fighter, false.into());
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        gaogaen_special_lw_main
    );
}