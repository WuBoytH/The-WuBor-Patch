use crate::imports::status_imports::*;

unsafe extern "C" fn lucario_auraball_start_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if !VarModule::is_flag(weapon.module_accessor, weapon::instance::flag::FROM_POCKET) {
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        if sv_battle_object::category(owner_id) == *BATTLE_OBJECT_CATEGORY_FIGHTER
        && sv_battle_object::kind(owner_id) == *FIGHTER_KIND_LUCARIO {
            let owner_object = MiscModule::get_battle_object_from_id(owner_id);
            if VarModule::is_flag((*owner_object).module_accessor, lucario::status::flag::SPECIAL_N_SPIRIT_BOMB) {
                VarModule::on_flag(weapon.module_accessor, lucario_auraball::instance::flag::SPIRIT_BOMB);
                let charge_max = WorkModule::get_param_float((*owner_object).module_accessor, hash40("param_special_n"), hash40("max_charge_frame"));
                WorkModule::set_int(weapon.module_accessor, charge_max as i32, *WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_INT_PARAM_MAX_CHARGE_FRAME);
                if StatusModule::situation_kind((*owner_object).module_accessor) == *SITUATION_KIND_GROUND {
                    WorkModule::set_customize_no(weapon.module_accessor, 1, 0);
                }
                else {
                    WorkModule::set_customize_no(weapon.module_accessor, 2, 0);
                }
            }
        }
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        lucario_auraball_start_end
    );
}