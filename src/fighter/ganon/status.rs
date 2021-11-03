use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    crate::table_const::*
};

#[status_script(agent = "ganon", status = FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ganon_sspecialaircatch_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    MotionModule::set_rate(fighter.module_accessor, 0.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_specialairscatch_mainloop as *const () as _))
}

unsafe extern "C" fn ganon_specialairscatch_mainloop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut val = 0;
    if fighter.global_table[CURRENT_FRAME].get_f32() == 1.0 {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_catch"), 1.0, 1.0, false, 0.0, false, false);
        fighter.set_situation(L2CValue::I32(*SITUATION_KIND_AIR));
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.clear_lua_stack();
        smash_script::lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0x8cdc1683 as u64, 0.0);
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        fighter.clear_lua_stack();
    }
    if fighter.global_table[CURRENT_FRAME].get_f32() >= 1.0 {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END.into(), false.into());
        }
        val = 1;
    }
    L2CValue::I32(val)
}

#[status_script(agent = "ganon", status = FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn ganon_sspecialairend_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 1);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, true, false, *WEAPON_MARIO_PUMP_WATER_STATUS_KIND_REGULAR as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    L2CValue::I32(0)
}

#[status_script(agent = "ganon", status = FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ganon_sspecialairend_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_specialairsend_mainloop as *const () as _))
}

unsafe extern "C" fn ganon_specialairsend_mainloop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
        }
    }
    L2CValue::I32(1)
}

#[common_status_script(status = FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn common_status_catchedairganon_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_share = WorkModule::get_param_int(fighter.module_accessor, 0xad2ee25e as u64, 0x7d88ea0 as u64);
    if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
        FighterMotionModuleImpl::add_body_type_hash(fighter.module_accessor, Hash40::new("catched_ganon"), *BODY_TYPE_MOTION_DX);
    }
    else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
        FighterMotionModuleImpl::add_body_type_hash(fighter.module_accessor, Hash40::new("catched_ganon"), *BODY_TYPE_MOTION_GIRL);
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("catched_ganon"), 1.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(common_status_catchedairganon_mainloop as *const () as _))
}

unsafe extern "C" fn common_status_catchedairganon_mainloop(_fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CValue::I32(0)
}

#[common_status_script(status = FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn common_status_catchedairendganon_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_share = WorkModule::get_param_int(fighter.module_accessor, 0xad2ee25e as u64, 0x7d88ea0 as u64);
    let throw_motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROWN_WORK_INT_MOTION_KIND);
    if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
        FighterMotionModuleImpl::add_body_type_hash(fighter.module_accessor, Hash40::new_raw(throw_motion), *BODY_TYPE_MOTION_DX);
    }
    else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
        FighterMotionModuleImpl::add_body_type_hash(fighter.module_accessor, Hash40::new_raw(throw_motion), *BODY_TYPE_MOTION_GIRL);
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(throw_motion), 1.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(common_status_catchedairendganon_mainloop as *const () as _))
}

unsafe extern "C" fn common_status_catchedairendganon_mainloop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_FALL.into(), false.into());
    }
    L2CValue::I32(0)
}

pub fn install() {
    install_status_scripts!(
        ganon_sspecialaircatch_main,
        ganon_sspecialairend_pre,
        ganon_sspecialairend_main,
        common_status_catchedairganon_main,
        common_status_catchedairendganon_main
    );
}