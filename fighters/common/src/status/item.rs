use super::*;

#[skyline::hook(replace = L2CFighterCommon_ItemThrowLightMotionDecision)]
unsafe extern "C" fn itemthrowlightmotiondecision(fighter: &mut L2CFighterCommon) {
    WorkModule::set_int64(fighter.module_accessor, hash40("item_light_throw_f") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("item_light_throw_air_f") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND_OPPOSITE);

    let cat3 = fighter.global_table[CMD_CAT3].get_i32();
    if cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_4 != 0 {
        if cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_FB4 != 0 {
            let fb4_kind = ControlModule::item_light_throw_fb4_kind(fighter.module_accessor) as i32;
            if fb4_kind == *FIGHTER_COMMAND_ITEM_THROW_KIND_F {
                WorkModule::set_int64(fighter.module_accessor, hash40("item_light_throw_f4") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND);
                WorkModule::set_int64(fighter.module_accessor, hash40("item_light_throw_air_f4") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND_OPPOSITE);
            }
            else {
                WorkModule::set_int64(fighter.module_accessor, hash40("item_light_throw_b4") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND);
                WorkModule::set_int64(fighter.module_accessor, hash40("item_light_throw_air_b4") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND_OPPOSITE);
                let lr = PostureModule::lr(fighter.module_accessor);
                WorkModule::set_float(fighter.module_accessor, -lr, *FIGHTER_STATUS_ITEM_THROW_WORK_FLOAT_LR);
            }
        }
        else {
            if cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_HI4 != 0 {
                WorkModule::set_int64(fighter.module_accessor, hash40("item_light_throw_hi4") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND);
                WorkModule::set_int64(fighter.module_accessor, hash40("item_light_throw_air_hi4") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND_OPPOSITE);
            }
            else {
                WorkModule::set_int64(fighter.module_accessor, hash40("item_light_throw_lw4") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND);
                WorkModule::set_int64(fighter.module_accessor, hash40("item_light_throw_air_lw4") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND_OPPOSITE);
            }
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ITEM_THROW_4);
    }
    else {
        if cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_HI != 0 {
            WorkModule::set_int64(fighter.module_accessor, hash40("item_light_throw_hi") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND);
            WorkModule::set_int64(fighter.module_accessor, hash40("item_light_throw_air_hi") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND_OPPOSITE);
        }
        else if cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_LW != 0 {
            WorkModule::set_int64(fighter.module_accessor, hash40("item_light_throw_lw") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND);
            WorkModule::set_int64(fighter.module_accessor, hash40("item_light_throw_air_lw") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND_OPPOSITE);
        }
        else if cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_DROP != 0 {
            // if ItemModule::is_have_item(fighter.module_accessor, 0)
            // && {
            //     fighter.clear_lua_stack();
            //     lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
            //     sv_module_access::item(fighter.lua_state_agent);
            //     fighter.pop_lua_stack(1).get_bool()
            // } {
                WorkModule::set_int64(fighter.module_accessor, hash40("item_light_throw_drop") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND);
                WorkModule::set_int64(fighter.module_accessor, hash40("item_light_throw_drop") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND_OPPOSITE);
            // }
        }
        else if cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_FB != 0 {
            let fb_kind = ControlModule::item_light_throw_fb_kind(fighter.module_accessor) as i32;
            if fb_kind == *FIGHTER_COMMAND_ITEM_THROW_KIND_F {
                WorkModule::set_int64(fighter.module_accessor, hash40("item_light_throw_f") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND);
                WorkModule::set_int64(fighter.module_accessor, hash40("item_light_throw_air_f") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND_OPPOSITE);
            }
            else {
                WorkModule::set_int64(fighter.module_accessor, hash40("item_light_throw_b") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND);
                WorkModule::set_int64(fighter.module_accessor, hash40("item_light_throw_air_b") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND_OPPOSITE);
                let lr = PostureModule::lr(fighter.module_accessor);
                WorkModule::set_float(fighter.module_accessor, -lr, *FIGHTER_STATUS_ITEM_THROW_WORK_FLOAT_LR);
            }
        }
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            itemthrowlightmotiondecision
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}