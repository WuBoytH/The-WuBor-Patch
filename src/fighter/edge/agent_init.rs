use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*}
};

pub unsafe extern "C" fn edge_status_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR
    || status == *FIGHTER_STATUS_KIND_REBIRTH {
        if ![
            *FIGHTER_STATUS_KIND_SPECIAL_HI,
            *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH,
            *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING,
            *FIGHTER_STATUS_KIND_JUMP_SQUAT
        ].contains(&status) {
            VarModule::set_int(fighter.battle_object, edge::instance::int::SPECIAL_HI_CANCEL_COUNT, 0);
        }
    }
    0.into()
}

#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_EDGE {
            return;
        }
        fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(edge_status_end_control as *const () as _));
    }
}

pub fn install() {
    install_agent_resets!(
        agent_reset
    );
}
