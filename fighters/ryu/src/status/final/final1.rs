use super::*;

unsafe extern "C" fn ryu_final_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.module_accessor, vars::ryu::instance::flag::SKIP_FINAL_PROX_CHECK) {
        fighter.sub_status_pre_FinalCommon();
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new_raw(0x20e3e3c94b));
        sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
        if !fighter.pop_lua_stack(1).get_bool() {
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_RYU_STATUS_KIND_FINAL2);
            return 1.into();
        }
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_DISABLE,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_FINAL|
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        (
            *FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT |
            *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE |
            *FIGHTER_STATUS_ATTR_FINAL
        ) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_FINAL as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn ryu_final_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.module_accessor, vars::ryu::instance::flag::SKIP_FINAL_PROX_CHECK);

    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x201bc9217c));

    // HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    let mot = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        "final"
    }
    else {
        "final_air"
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new(mot),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    ItemModule::set_change_status_event(fighter.module_accessor, false);

    let main_loop = smashline::api::get_target_function("lua2cpp_ryu.nrs", 0x43f70).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_FINAL, ryu_final_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_FINAL, ryu_final_main);
}