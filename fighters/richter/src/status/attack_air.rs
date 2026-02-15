use super::*;

unsafe extern "C" fn richter_attack_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_uniq_process_exec();
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw")
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_HIT)
    && !fighter.global_table[IS_STOP].get_bool() {
        fighter.change_status(vars::richter::status::SLIDE_BOUNCE.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn attack_air_check_attack(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    if [
        hash40("attack_air_lw"),
        hash40("fall_leaning_c")
    ].contains(&motion) {
        let table = param_3.get_table() as *mut smash_rs::lib::L2CTable;
        let kind = MiscModule::get_table_value(table, "kind_").try_integer().unwrap() as i32;
    
        if ![
            *COLLISION_KIND_SHIELD,
        ].contains(&kind) {
            VarModule::on_flag(fighter.module_accessor, vars::richter::status::flag::SLIDE_BOUNCE_IS_HIT);
        }

        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_HIT);
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_ATTACK_AIR, richter_attack_air_exec);
    agent.status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_check_attack);
}