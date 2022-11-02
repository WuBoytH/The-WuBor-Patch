use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    custom_var::*,
    wubor_utils::vars::*
};

unsafe fn pickel_sync_mining_pattern(fighter: &mut L2CFighterCommon) {
    let patterns = [
        *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_SPECIAL_N1_MINING_GRADE_1_TABLE_PROGRESS,
        *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_SPECIAL_N1_MINING_WOOD_TABLE_PROGRESS,
        *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_SPECIAL_N1_MINING_STONE_TABLE_PROGRESS,
        *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_SPECIAL_N1_MINING_IRON_TABLE_PROGRESS,
        *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_SPECIAL_N1_MINING_NOT_NORMAL_STAGE_TABLE_PROGRESS
    ];
    let init_val = VarModule::get_int(fighter.battle_object, pickel::instance::int::MINING_PROGRESS_REAL);
    for x in patterns.iter() {
        let count = WorkModule::get_int(fighter.module_accessor, *x);
        if init_val < count || init_val != count && count == 1 {
            VarModule::set_int(fighter.battle_object, pickel::instance::int::MINING_PROGRESS_REAL, count);
            for y in patterns.iter() {
                WorkModule::set_int(fighter.module_accessor, count, *y);
            }
            break;
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_PICKEL )]
fn pickel_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        pickel_sync_mining_pattern(fighter);
    }
}

pub fn install() {
    install_agent_frames!(
        pickel_frame
    );
}