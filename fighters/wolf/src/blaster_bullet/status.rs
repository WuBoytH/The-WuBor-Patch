use super::*;

unsafe extern "C" fn stay_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    effect!(
        weapon,
        MA_MSC_CMD_EFFECT_EFFECT,
        Hash40::new("fox_blaster_hit"),
        Hash40::new("top"),
        8.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        false
    );
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *WEAPON_FOX_BLASTER_BULLET_STATUS_KIND_STAY, stay_end);
}