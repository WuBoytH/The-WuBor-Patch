use crate::imports::status_imports::*;

#[skyline::hook(replace = L2CFighterCommon_status_CliffJump2)]
unsafe extern "C" fn status_cliffjump2(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, cliff::flag::CLIFF_JUMP_MINI) {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            speed_y * 0.6
        );
    }

    fighter.sub_air_check_fall_common_pre();

    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_STOP_CEIL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LASSO);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("cliff_jump_quick2"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_cliff_jump2_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_cliff_jump2_uniq as *const () as _));

    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CliffJump2_Main as *const () as _))
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_cliffjump2
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}