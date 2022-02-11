use {
    smash::{
        app::*,
        lib::lua_const::*
    }
};

#[repr(C)]
pub struct WorkModule {
  vtable: *const u64,
  owner: *mut BattleObjectModuleAccessor,
  // ...
}

#[macro_export]
macro_rules! dump_trace {
    () => {{
        let text = ::skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        dump_trace!(text)
    }};
    ($base:expr) => {{
        const MAXIMUM_BT_LEN: usize = 0x20;
        let text = $base;
        println!("Current text address: {:#x}", text);

        let mut lr: *const u64;
        unsafe {
            asm!("mov {}, x30", out(reg) lr);
        }

        let mut fp: *const u64;
        unsafe {
            asm!("mov {}, x29", out(reg) fp);
        }

        println!("Current LR:\t\t{:#x} ({:#x})", (lr as u64) - text, (lr as u64));

        let mut counter = 0usize;
        while !fp.is_null() && counter < MAXIMUM_BT_LEN {
            lr = *fp.offset(1) as *const u64;
            if !lr.is_null() {
                println!("[{}]: {:#x} ({:#x})", counter, (lr as u64), (lr as u64) - text);
                counter += 1;
            }
            fp = *fp as *const u64;
        }
    }}
}

#[skyline::hook(offset = 0x4e4600)]
pub unsafe fn set_int_debug(module: *mut WorkModule, val: i32, what: i32) {
    let boma = &mut *(*module).owner;
    let fighter_kind = utility::get_kind(boma);
    let category = utility::get_category(boma);
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if fighter_kind == *FIGHTER_KIND_SHULK {
            if what == *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE {
                println!("Art Type has been set!");
                dump_trace!();
            }
        }
    }
    call_original!(module, val, what)
}

#[skyline::hook(offset = 0x4e4910)]
unsafe fn on_flag_debug(module: *mut WorkModule, what: i32) -> bool {
    let module_accessor = (*module).owner;
    if utility::get_kind(&mut *module_accessor) == *FIGHTER_KIND_SHULK && what == *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE {
        println!("Art activated!");
        dump_trace!();
    }
    call_original!(module, what)
}

#[skyline::hook(offset = 0x4e4a10)]
pub unsafe fn set_flag_debug(module: *mut WorkModule, val: bool, what: i32) {
    let boma = &mut *(*module).owner;
    let fighter_kind = utility::get_kind(boma);
    let category = utility::get_category(boma);
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if fighter_kind == *FIGHTER_KIND_SHULK {
            if what == *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE {
                println!("Art activated! (in set_flag)");
                dump_trace!();
            }
        }
    }
    call_original!(module, val, what)
}

#[skyline::hook(offset = 0x4e4420)]
pub unsafe fn set_float_debug(module: *mut WorkModule, val: f32, what: i32) {
    let boma = &mut *(*module).owner;
    let fighter_kind = utility::get_kind(boma);
    let category = utility::get_category(boma);
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if fighter_kind == *FIGHTER_KIND_SHULK {
            if what == *FIGHTER_SHULK_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_SHIELD_ENDURE_MUL {
                println!("Shield Endure Mul Set!");
                dump_trace!();
            }
            if what == *FIGHTER_SHULK_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_SMASH_BLOWN_MUL {
                println!("Smash Blown Mul Set!");
                dump_trace!();
            }
        }
    }
    call_original!(module, val, what)
}

#[skyline::hook(offset = 0x1158f10, inline)]
pub unsafe fn thefuckisthis(ctx: &mut skyline::hooks::InlineCtx) {
    println!("the fuck is this?");
    dump_trace!();
}

pub fn install() {
    skyline::install_hooks!(
        set_int_debug,
        on_flag_debug,
        set_flag_debug,
        set_float_debug,
        thefuckisthis
    );
}