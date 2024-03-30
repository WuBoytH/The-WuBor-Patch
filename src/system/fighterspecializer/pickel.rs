use crate::imports::*;

#[skyline::hook( offset = 0xf10680 )]
pub unsafe fn is_mining_material_table_normal() -> bool {
    false
}

#[skyline::hook( offset = 0xf10710 )]
pub unsafe fn get_mining_material_table_result(fighter: &mut Fighter, table: i32, progress: i32) -> i32 {
    let ret = call_original!(fighter, table, progress);
    let module_accessor = fighter.battle_object.module_accessor;
    let weapon = WorkModule::get_int(module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_KIND);
    let material = WorkModule::get_int(module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    let upper_bound = if weapon != *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_NONE {
        match material {
            0 => 1,
            1 => 2,
            2 => 3,
            _ => 6
        }
    }
    else {
        1
    };
    ret.clamp(0, upper_bound)
}

pub fn install() {
    skyline::install_hooks!(
        is_mining_material_table_normal,
        get_mining_material_table_result
    );
    
}