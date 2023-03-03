use crate::imports::status_imports::*;

#[status_script(agent = "edge", status = FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn edge_special_hi_rush_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        VarModule::on_flag(fighter.battle_object, edge::status::flag::SPECIAL_HI_CANCEL);
    }
    original!(fighter)
}

pub fn install() {
    install_status_scripts!(
        edge_special_hi_rush_end
    );
}