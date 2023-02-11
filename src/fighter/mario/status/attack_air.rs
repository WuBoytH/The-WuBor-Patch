use {
    smash::{
        lua2cpp::L2CFighterCommon,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    super::super::helper::*
};

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn mario_attackair_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    mario_remove_hammer(fighter);
    0.into()
}

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn mario_landingattackair_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_LandingAttackAir();
    mario_remove_hammer(fighter);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        mario_attackair_end,
        mario_landingattackair_end
    );
}