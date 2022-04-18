use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    wubor_utils::table_const::*,
    custom_status::*,
    super::{
        helper::*,
        super::vars::*
    }
};

#[status_script(agent = "marth", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn marth_speciallw_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU);
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_hit_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_hit_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            marth_speciallw_hit_mot_helper(fighter);
        }
    }
    let frame = fighter.global_table[MOTION_FRAME].get_f32();
    if frame > 1.0
    && !CancelModule::is_enable_cancel(fighter.module_accessor) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    if frame > 10.0
    && WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU);
    }
    if marth_stance_cancel_helper(fighter).get_bool()
    || marth_stance_dash_cancel_helper(fighter, true).get_bool() {
        return 1.into();
    }
    marth_stance_mot_end_helper(fighter);
    0.into()
}

unsafe extern "C" fn marth_speciallw_hit_mot_helper(fighter: &mut L2CFighterCommon) {
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    let correct;
    let mot;
    if sit != *SITUATION_KIND_GROUND {
        correct = *GROUND_CORRECT_KIND_AIR;
        mot = Hash40::new("special_air_lw");
    }
    else {
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP;
        mot = Hash40::new("special_lw");
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
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

#[status_script(agent = "marth", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn marth_speciallw_hit_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU)
    && ![
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_DASH_F),
        CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_DASH_B),
        CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK),
        CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK_LW3),
    ].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    marth_speciallw_common_end(fighter);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        marth_speciallw_hit_main,
        marth_speciallw_hit_end
    );
}