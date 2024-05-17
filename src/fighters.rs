macro_rules! install_fighters {
    ($func:ident; $($name:ident = $feature:expr),*) => {{
        $(
            #[cfg(feature = $feature)]
            { $name::$func() }
        )*
    }}
}

pub fn install() {
    #[cfg(feature = "main_nro")]
    {
        common::install();
    }

    install_fighters! {
        install;
        bayonetta = "bayonetta",
        brave = "brave"
    //     buddy = "buddy",
    //     captain = "captain",
    //     chrom = "chrom",
    //     cloud = "cloud",
    //     daisy = "daisy",
    //     dedede = "dedede",
    //     demon = "demon",
    //     diddy = "diddy",
    //     dolly = "dolly",
    //     donkey = "donkey",
    //     duckhunt = "duckhunt",
    //     edge = "edge",
    //     eflame = "eflame",
    //     elight = "elight",
    //     falco = "falco",
    //     fox = "fox",
    //     gamewatch = "gamewatch",
    //     ganon = "ganon",
    //     gaogaen = "gaogaen",
    //     gekkouga = "gekkouga",
    //     iceclimber = "iceclimber",
    //     ike = "ike",
    //     inkling = "inkling",
    //     jack = "jack",
    //     kamui = "kamui",
    //     ken = "ken",
    //     kirby = "kirby",
    //     koopa = "koopa",
    //     koopajr = "koopajr",
    //     krool = "krool",
    //     link = "link",
    //     littlemac = "littlemac",
    //     lucario = "lucario",
    //     lucas = "lucas",
    //     lucina = "lucina",
    //     luigi = "luigi",
    //     mario = "mario",
    //     mariod = "mariod",
    //     marth = "marth",
    //     master = "master",
    //     metaknight = "metaknight",
    //     mewtwo = "mewtwo",
    //     miifighter = "miifighter",
    //     miigunner = "miigunner",
    //     miiswordsman = "miiswordsman",
    //     murabito = "murabito",
    //     ness = "ness",
    //     packun = "packun",
    //     pacman = "pacman",
    //     palutena = "palutena",
    //     peach = "peach",
    //     pfushigisou = "pfushigisou",
    //     pichu = "pichu",
    //     pickel = "pickel",
    //     pikachu = "pikachu",
    //     pikmin = "pikmin",
    //     pit = "pit",
    //     pitb = "pitb",
    //     plizardon = "plizardon",
    //     purin = "purin",
    //     pzenigame = "pzenigame",
    //     reflet = "reflet",
    //     richter = "richter",
    //     ridley = "ridley",
    //     robot = "robot",
    //     rockman = "rockman",
    //     rosetta = "rosetta",
    //     roy = "roy",
    //     ryu = "ryu",
    //     samus = "samus",
    //     samusd = "samusd",
    //     sheik = "sheik",
    //     shizue = "shizue",
    //     shulk = "shulk",
    //     simon = "simon",
    //     snake = "snake",
    //     sonic = "sonic",
    //     szerosuit = "szerosuit",
    //     tantan = "tantan",
    //     toonlink = "toonlink",
    //     trail = "trail",
    //     wario = "wario",
    //     wiifit = "wiifit",
    //     wolf = "wolf",
    //     yoshi = "yoshi",
    //     younglink = "younglink",
    //     zelda = "zelda"
    }
}
