use super::*;

pub unsafe extern "C" fn ken_final_set_area(fighter: &mut L2CFighterCommon, enable: L2CValue) {
    for x in 0..*FIGHTER_AREA_KIND_NUM {
        if x != *FIGHTER_AREA_KIND_WIND && x != *FIGHTER_AREA_KIND_WIND_RAD {
            AreaModule::enable_area(fighter.module_accessor, x, enable.get_bool(), -1);
        }
    }
}

mod final1;
mod final1_jump;
mod final1_hit;

mod final2;
mod final2_fall;
mod final2_landing;
mod final2_air_end;

pub fn install(agent: &mut Agent) {
    final1::install(agent);
    final1_jump::install(agent);
    final1_hit::install(agent);

    final2::install(agent);
    final2_fall::install(agent);
    final2_landing::install(agent);
    final2_air_end::install(agent);
}