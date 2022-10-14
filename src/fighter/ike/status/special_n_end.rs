use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::*,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*},
    super::super::vl
};

#[status_script(agent = "ike", status = FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn ike_special_n_end_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    ike_special_n_end_init_inner(fighter)
}

pub unsafe extern "C" fn ike_special_n_end_init_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    let charge_count = WorkModule::get_float(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT);
    let special_n_charge_count_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_charge_count_max"));
    let max_count = special_n_charge_count_max as f32 * 30.0;
    let eruption_count = if max_count <= charge_count {
        vl::special_n::eruption_count_max
    }
    else {
        let ratio = charge_count / max_count;
        (ratio * vl::special_n::eruption_count_max as f32) as i32
    };
    VarModule::set_int(fighter.battle_object, ike::status::int::ERUPTION_COUNT, eruption_count);
    0.into()
}

#[status_script(agent = "ike", status = FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ike_special_n_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
    ike_special_n_end_mot_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(ike_special_n_end_main_loop as *const () as _))
}

unsafe extern "C" fn ike_special_n_end_mot_helper(fighter: &mut L2CFighterCommon) {
    let mot;
    let kinetic;
    let correct;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        mot = Hash40::new("special_air_n_end");
        kinetic = *FIGHTER_KINETIC_TYPE_AIR_STOP;
        correct = *GROUND_CORRECT_KIND_AIR;
    }
    else {
        mot = Hash40::new("special_n_end");
        kinetic = *FIGHTER_KINETIC_TYPE_GROUND_STOP;
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP;
    }
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT) {
        MotionModule::change_motion(
            fighter.module_accessor,
            mot,
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
    }
    else {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            mot,
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
}

unsafe extern "C" fn ike_special_n_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        ike_special_n_end_mot_helper(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        ike_special_n_end_init, ike_special_n_end_main
    );
}