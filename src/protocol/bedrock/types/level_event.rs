pub struct LevelEvent {}

impl LevelEvent {
    pub const SOUND_CLICK: i32 = 1000;
    pub const SOUND_CLICK_FAIL: i32 = 1001;
    pub const SOUND_SHOOT: i32 = 1002;
    pub const SOUND_DOOR: i32 = 1003;
    pub const SOUND_FIZZ: i32 = 1004;
    pub const SOUND_IGNITE: i32 = 1005;
    pub const SOUND_PLAY_RECORDING: i32 = 1006;
    pub const SOUND_GHAST: i32 = 1007;
    pub const SOUND_GHAST_SHOOT: i32 = 1008;
    pub const SOUND_BLAZE_SHOOT: i32 = 1009;
    pub const SOUND_DOOR_BUMP: i32 = 1010;

    pub const SOUND_DOOR_CRASH: i32 = 1012;

    pub const SOUND_ZOMBIE_INFECTED: i32 = 1016;
    pub const SOUND_ZOMBIE_CONVERT: i32 = 1017;
    pub const SOUND_ENDERMAN_TELEPORT: i32 = 1018;

    pub const SOUND_ANVIL_BREAK: i32 = 1020;
    pub const SOUND_ANVIL_USE: i32 = 1021;
    pub const SOUND_ANVIL_FALL: i32 = 1022;

    pub const SOUND_POP: i32 = 1030;

    pub const SOUND_PORTAL: i32 = 1032;

    pub const SOUND_ITEMFRAME_ADD_ITEM: i32 = 1040;
    pub const SOUND_ITEMFRAME_REMOVE: i32 = 1041;
    pub const SOUND_ITEMFRAME_PLACE: i32 = 1042;
    pub const SOUND_ITEMFRAME_REMOVE_ITEM: i32 = 1043;
    pub const SOUND_ITEMFRAME_ROTATE_ITEM: i32 = 1044;

    pub const SOUND_CAMERA: i32 = 1050;
    pub const SOUND_ORB: i32 = 1051;
    pub const SOUND_TOTEM: i32 = 1052;

    pub const SOUND_ARMOR_STAND_BREAK: i32 = 1060;
    pub const SOUND_ARMOR_STAND_HIT: i32 = 1061;
    pub const SOUND_ARMOR_STAND_FALL: i32 = 1062;
    pub const SOUND_ARMOR_STAND_PLACE: i32 = 1063;
    pub const SOUND_POINTED_DRIPSTONE_FALL: i32 = 1064;
    pub const SOUND_DYE_USED: i32 = 1065;
    pub const SOUND_INK_SAC_USED: i32 = 1066;

    pub const PARTICLE_SHOOT: i32 = 2000;
    pub const PARTICLE_DESTROY: i32 = 2001; //sound + particles
    pub const PARTICLE_SPLASH: i32 = 2002;
    pub const PARTICLE_EYE_DESPAWN: i32 = 2003;
    pub const PARTICLE_SPAWN: i32 = 2004;
    pub const BONE_MEAL_USE: i32 = 2005; //sound + green particles
    pub const GUARDIAN_CURSE: i32 = 2006;
    pub const PARTICLE_DEATH_SMOKE: i32 = 2007;
    pub const PARTICLE_BLOCK_FORCE_FIELD: i32 = 2008;
    pub const PARTICLE_PROJECTILE_HIT: i32 = 2009;
    pub const PARTICLE_DRAGON_EGG_TELEPORT: i32 = 2010;
    pub const PARTICLE_CROP_EATEN: i32 = 2011;
    pub const PARTICLE_CRITICAL_HIT: i32 = 2012;
    pub const PARTICLE_ENDERMAN_TELEPORT: i32 = 2013;
    pub const PARTICLE_PUNCH_BLOCK: i32 = 2014;
    pub const PARTICLE_BUBBLE: i32 = 2015;
    pub const PARTICLE_EVAPORATE: i32 = 2016;
    pub const PARTICLE_ARMOR_STAND_DESTROY: i32 = 2017;
    pub const PARTICLE_EGG_PUNCH: i32 = 2018;
    pub const PARTICLE_EGG_BREAK: i32 = 2019;
    pub const PARTICLE_ICE_EVAPORATE: i32 = 2020;
    pub const PARTICLE_DESTROY_NO_SOUND: i32 = 2021;
    pub const PARTICLE_KNOCKBACK_ROAR: i32 = 2022; //spews out tons of white particles
    pub const PARTICLE_TELEPORT_TRAIL: i32 = 2023;
    pub const PARTICLE_POINT_CLOUD: i32 = 2024;
    pub const PARTICLE_EXPLODE: i32 = 2025; //data >= 2: i32 = huge explode seed, otherwise huge explode
    pub const PARTICLE_BLOCK_EXPLODE: i32 = 2026;
    pub const PARTICLE_VIBRATION_SIGNAL: i32 = 2027;
    pub const PARTICLE_DRIPSTONE_DRIP: i32 = 2028;
    pub const PARTICLE_FIZZ: i32 = 2029;
    pub const COPPER_WAX_ON: i32 = 2030; //sound + particles
    pub const COPPER_WAX_OFF: i32 = 2031; //sound + particles
    pub const COPPER_SCRAPE: i32 = 2032; //sound + particles
    pub const PARTICLE_ELECTRIC_SPARK: i32 = 2033; //lightning rod
    pub const PARTICLE_TURTLE_EGG_GROW: i32 = 2034;
    pub const PARTICLE_SCULK_SHRIEK: i32 = 2035;
    pub const PARTICLE_SCULK_CATALYST_BLOOM: i32 = 2036;

    pub const PARTICLE_DUST_PLUME: i32 = 2040;

    pub const START_RAIN: i32 = 3001;
    pub const START_THUNDER: i32 = 3002;
    pub const STOP_RAIN: i32 = 3003;
    pub const STOP_THUNDER: i32 = 3004;
    pub const PAUSE_GAME: i32 = 3005; //data: 1 to pause, 0 to resume
    pub const PAUSE_GAME_NO_SCREEN: i32 = 3006; //data: 1 to pause, 0 to resume - same effect as normal pause but without screen
    pub const SET_GAME_SPEED: i32 = 3007; //x coordinate of pos: i32 = scale factor (default 1.0)

    pub const REDSTONE_TRIGGER: i32 = 3500;
    pub const CAULDRON_EXPLODE: i32 = 3501;
    pub const CAULDRON_DYE_ARMOR: i32 = 3502;
    pub const CAULDRON_CLEAN_ARMOR: i32 = 3503;
    pub const CAULDRON_FILL_POTION: i32 = 3504;
    pub const CAULDRON_TAKE_POTION: i32 = 3505;
    pub const CAULDRON_FILL_WATER: i32 = 3506;
    pub const CAULDRON_TAKE_WATER: i32 = 3507;
    pub const CAULDRON_ADD_DYE: i32 = 3508;
    pub const CAULDRON_CLEAN_BANNER: i32 = 3509; //particle + sound
    pub const PARTICLE_CAULDRON_FLUSH: i32 = 3510;
    pub const PARTICLE_AGENT_SPAWN: i32 = 3511;
    pub const SOUND_CAULDRON_FILL_LAVA: i32 = 3512;
    pub const SOUND_CAULDRON_TAKE_LAVA: i32 = 3513;
    pub const SOUND_CAULDRON_FILL_POWDER_SNOW: i32 = 3514;
    pub const SOUND_CAULDRON_TAKE_POWDER_SNOW: i32 = 3515;

    pub const BLOCK_START_BREAK: i32 = 3600;
    pub const BLOCK_STOP_BREAK: i32 = 3601;
    pub const BLOCK_BREAK_SPEED: i32 = 3602;
    pub const PARTICLE_PUNCH_BLOCK_DOWN: i32 = 3603;
    pub const PARTICLE_PUNCH_BLOCK_UP: i32 = 3604;
    pub const PARTICLE_PUNCH_BLOCK_NORTH: i32 = 3605;
    pub const PARTICLE_PUNCH_BLOCK_SOUTH: i32 = 3606;
    pub const PARTICLE_PUNCH_BLOCK_WEST: i32 = 3607;
    pub const PARTICLE_PUNCH_BLOCK_EAST: i32 = 3608;
    pub const PARTICLE_SHOOT_WHITE_SMOKE: i32 = 3609;
    pub const PARTICLE_BREEZE_WIND_EXPLOSION: i32 = 3610;
    pub const PARTICLE_TRIAL_SPAWNER_DETECTION: i32 = 3611;
    pub const PARTICLE_TRIAL_SPAWNER_SPAWNING: i32 = 3612;
    pub const PARTICLE_TRIAL_SPAWNER_EJECTING: i32 = 3613;
    pub const PARTICLE_WIND_EXPLOSION: i32 = 3614;

    pub const SET_DATA: i32 = 4000;

    pub const PLAYERS_SLEEPING: i32 = 9800;
    pub const NUMBER_OF_SLEEPING_PLAYERS: i32 = 9801;

    pub const JUMP_PREVENTED: i32 = 9810;
    pub const ANIMATION_VAULT_ACTIVATE: i32 = 9811;
    pub const ANIMATION_VAULT_DEACTIVATE: i32 = 9812;
    pub const ANIMATION_VAULT_EJECT_ITEM: i32 = 9813;

    pub const ADD_PARTICLE_MASK: i32 = 0x4000;
}
