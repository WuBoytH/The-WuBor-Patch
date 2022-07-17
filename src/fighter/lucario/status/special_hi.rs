use {
    smash::{
        lua2cpp::L2CFighterCommon,
        // hash40,
        // phx::*,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    // smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*/*, table_const::**/},
    // super::super::helper::*
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
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    let enhance = VarModule::get_int(fighter.battle_object, lucario::instance::int::AURA_LEVEL) > 0;
    if enhance {
        VarModule::on_flag(fighter.battle_object, lucario::status::flag::IS_AURA_ENHANCED);
        VarModule::dec_int(fighter.battle_object, lucario::instance::int::AURA_LEVEL);
    }
    0.into()
}

// #[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// unsafe fn lucario_special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_CONTINUE);
//     WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_GRAVITY_ONOFF);
//     GroundModule::set_passable_check(fighter.module_accessor, true);
//     let lr = PostureModule::lr(fighter.module_accessor);
//     let mot;
//     let air_mot;
//     if lr != -1.0 {
//         mot = hash40("special_hi_r");
//         air_mot = hash40("special_air_hi_r");
//     }
//     else {
//         mot = hash40("special_hi_l");
//         air_mot = hash40("special_air_hi_l");
//     }
//     WorkModule::set_int64(fighter.module_accessor, mot as i64, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_MOTION_KIND);
//     WorkModule::set_int64(fighter.module_accessor, air_mot as i64, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_MOTION_KIND_AIR);
//     lucario_special_hi_set_kinetic_mot(fighter);
//     fighter.sub_shift_status_main(L2CValue::Ptr(lucario_special_hi_main_loop as *const () as _))
// }

// unsafe extern "C" fn lucario_special_hi_set_kinetic_mot(fighter: &mut L2CFighterCommon) {
//     let mot;
//     if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
//         GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
//         KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
//         mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_MOTION_KIND_AIR);
//     }
//     else {
//         GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
//         KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
//         mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_MOTION_KIND);
//     }
//     if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_CONTINUE) {
//         MotionModule::change_motion_inherit_frame(
//             fighter.module_accessor,
//             Hash40::new_raw(mot),
//             -1.0,
//             1.0,
//             0.0,
//             false,
//             false
//         );
//     }
//     else {
//         MotionModule::change_motion(
//             fighter.module_accessor,
//             Hash40::new_raw(mot),
//             0.0,
//             1.0,
//             false,
//             0.0,
//             false,
//             false
//         );
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_CONTINUE);
//     }
// }

// unsafe extern "C" fn lucario_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
//     if !StatusModule::is_changing(fighter.module_accessor)
//     && StatusModule::is_situation_changed(fighter.module_accessor) {
//         lucario_special_hi_set_kinetic_mot(fighter);
//     }
//     if fighter.sub_transition_group_check_air_cliff().get_bool() {
//         return 1.into();
//     }
//     if CancelModule::is_enable_cancel(fighter.module_accessor) {
//         if fighter.sub_wait_ground_check_common(false.into()).get_bool()
//         || fighter.sub_air_check_fall_common().get_bool() {
//             return 0.into();
//         }
//     }
//     if !MotionModule::is_end(fighter.module_accessor) {

//     }
//     else {
//         fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH.into(), false.into());
//     }
//     0.into()
// }

// unsafe extern "C" fn lucario_special_hi_rush_dir(fighter: &mut L2CFighterCommon) {
//     if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_RUSH_DIR) {
//         return;
//     }
//     let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
//     let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
//     let angle = 90.0f32.to_radians();
//     let rush_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rush_speed"));
//     let rush_speed_add = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), 0x189cd804c5);
//     let curr_aura = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
//     let rush_speed = rush_speed_add * curr_aura + rush_speed;
//     let rush_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rush_stick"));
//     let lr = PostureModule::lr(fighter.module_accessor);
//     if lr != -1.0 {
//         PostureModule::set_lr(fighter.module_accessor, 1.0);
//         PostureModule::update_rot_y_lr(fighter.module_accessor);
//         WorkModule::set_int64(fighter.module_accessor, hash40("special_hi_r") as i64, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_MOTION_KIND);
//         WorkModule::set_int64(fighter.module_accessor, hash40("special_air_hi_r") as i64, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_INT_MOTION_KIND_AIR);
//         let mot = if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
//             hash40("special_air_hi_r")
//         }
//         else {
//             hash40("special_hi_r")
//         };
//         MotionModule::change_motion_inherit_frame(
//             fighter.module_accessor,
//             Hash40::new_raw(mot),
//             -1.0,
//             1.0,
//             0.0,
//             false,
//             false
//         );
//     }
//     if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
//         let touch = GroundModule::get_touch_normal_consider_gravity(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);

//     }
// }

pub fn install() {
    install_status_scripts!(
        lucario_special_lw_pre/*, lucario_special_hi_main*/
    );
}