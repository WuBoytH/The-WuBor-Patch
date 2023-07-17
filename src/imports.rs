pub mod acmd_imports {
    pub use {
        smash::{
            lua2cpp::*,
            hash40,
            phx::*,
            app::{lua_bind::*, sv_animcmd::*, *},
            lib::lua_const::*
        },
        smash_script::*,
        smashline::*,
        custom_var::*,
        wubor_utils::{wua_bind::*, vars::*}
    };
}

pub mod status_imports {
    pub use {
        smash::{
            lua2cpp::*,
            hash40,
            phx::*,
            app::{lua_bind::*, sv_animcmd::*, *},
            lib::{lua_const::*, L2CValue, L2CAgent}
        },
        smash_script::*,
        smashline::*,
        custom_var::*,
        custom_status::*,
        custom_cancel::*,
        wubor_utils::{cancels::*, wua_bind::*, vars::*, table_const::*}
    };
}