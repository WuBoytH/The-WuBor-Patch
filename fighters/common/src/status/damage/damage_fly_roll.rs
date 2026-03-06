use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_DamageFlyRoll)]
unsafe extern "C" fn status_pre_damageflyroll(fighter: &mut L2CFighterCommon) -> L2CValue {
    super::damage_fly::damagefly_pre_inner(fighter)
}

#[skyline::hook(replace = L2CFighterCommon_status_DamageFlyRoll_Common)]
unsafe extern "C" fn status_damageflyroll_common(fighter: &mut L2CFighterCommon) {
    MotionAnimcmdModule::call_script_single(
        fighter.module_accessor,
        *FIGHTER_ANIMCMD_EXPRESSION,
        Hash40::new_raw(0x1b19dc3bd1),
        -1
    );
    fighter.sub_DamageFly_setup_strans();
    fighter.sub_DamageFlyCommon_check_dead();
    fighter.sub_DamageFlyCommon_init();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.damage_air_chk_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_damage_fly_chk_uniq as *const () as _));
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_damageflyroll,
            status_damageflyroll_common
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}