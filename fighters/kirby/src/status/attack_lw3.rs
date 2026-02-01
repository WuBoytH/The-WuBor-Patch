use super::*;

unsafe extern "C" fn kirby_attacklw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackLw3_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_attacklw3_main_loop as *const () as _))
}

unsafe extern "C" fn kirby_attacklw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.global_table[IS_STOP].get_bool()
    && VarModule::is_flag(fighter.module_accessor, vars::kirby::status::flag::ATTACK_LW3_BOUNCE) {
        fighter.change_status(vars::kirby::status::ATTAK_LW3_BOUNCE.into(), false.into());
        return 1.into();
    }

    fighter.status_AttackLw3_Main()
}

unsafe fn get_table_value(table: *mut smash_rs::lib::L2CTable, key: &str) -> smash_rs::lib::L2CValue {
    let hash = if key.starts_with("0x") {
        smash_rs::phx::Hash40::from_hex_str(key).unwrap()
    } else {
        smash_rs::phx::hash40(key)
    };
    (*table).get_map(hash).unwrap().clone()
}

unsafe extern "C" fn kirby_attacklw3_check_attack(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash_rs::lib::L2CTable;
    let kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;

    if [
        *COLLISION_KIND_HIT,
        *COLLISION_KIND_SHIELD
    ].contains(&kind) {
        if !VarModule::is_flag(fighter.module_accessor, vars::kirby::status::flag::ATTACK_LW3_BOUNCE) {
            EffectModule::req_on_joint(
                fighter.module_accessor,
                Hash40::new("kirby_star"),
                Hash40::new("top"),
                &Vector3f{x: 0.0, y: 3.0, z: 6.0},
                &vars::ZERO_VECTOR,
                1.0,
                &vars::ZERO_VECTOR,
                &vars::ZERO_VECTOR,
                false,
                0,
                0,
                0
            );
        }
        if kind == *COLLISION_KIND_HIT {
            VarModule::on_flag(fighter.module_accessor, vars::kirby::status::flag::ATTACK_LW3_HIT);
        }
        VarModule::on_flag(fighter.module_accessor, vars::kirby::status::flag::ATTACK_LW3_BOUNCE);
    }

    fighter.FighterStatusUniqProcessAttackLw3_check_attack(param_2.clone(), param_3.clone())
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW3, kirby_attacklw3_main);
    agent.status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_LW3, kirby_attacklw3_check_attack);
}