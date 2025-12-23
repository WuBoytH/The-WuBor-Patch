use super::*;

unsafe extern "C" fn kirby_attackdash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy!(
        set_speed_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        1.32
    );
    fighter.status_AttackDash()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_DASH, kirby_attackdash_main);
}