use crate::imports::*;
use crate::offsets;

#[skyline::hook( offset = offsets::fighter::pickel::IS_MINING_MATERIAL_TABLE_NORMAL )]
pub unsafe fn is_mining_material_table_normal() -> bool {
    false
}

#[skyline::hook( offset = offsets::fighter::pickel::GET_MINING_MATERIAL_TABLE_RESULT )]
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
    unsafe {
        // Forces the common mining pattern
        skyline::patching::Patch::in_text(offsets::fighter::pickel::FORCE_COMMON_MINING_PATTERN_PATCH).data(0x14000012u32);

        // Skips the check for what tool to mine with
        skyline::patching::Patch::in_text(offsets::fighter::pickel::SKIP_MINING_TOOL_CHECK_PATCH).data(0x14000023u32);

        // Forces the mining tool used to be Pickaxe
        skyline::patching::Patch::in_text(offsets::fighter::pickel::FORCE_PICKAXE_PATCH).data(0x321F03E1u32);

        skyline::install_hooks!(
            is_mining_material_table_normal,
            get_mining_material_table_result
        );
    
    }
}