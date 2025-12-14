pub struct ActorEvent {}

impl ActorEvent {
    pub const JUMP: i32 = 1;
    pub const HURT_ANIMATION: i32 = 2;
    pub const DEATH_ANIMATION: i32 = 3;
    pub const ARM_SWING: i32 = 4;
    pub const STOP_ATTACK: i32 = 5;
    pub const TAME_FAIL: i32 = 6;
    pub const TAME_SUCCESS: i32 = 7;
    pub const SHAKE_WET: i32 = 8;
    pub const USE_ITEM: i32 = 9;
    pub const EAT_GRASS_ANIMATION: i32 = 10;
    pub const FISH_HOOK_BUBBLE: i32 = 11;
    pub const FISH_HOOK_POSITION: i32 = 12;
    pub const FISH_HOOK_HOOK: i32 = 13;
    pub const FISH_HOOK_TEASE: i32 = 14;
    pub const SQUID_INK_CLOUD: i32 = 15;
    pub const ZOMBIE_VILLAGER_CURE: i32 = 16;
    pub const PLAY_AMBIENT_SOUND: i32 = 17;
    pub const RESPAWN: i32 = 18;
    pub const IRON_GOLEM_OFFER_FLOWER: i32 = 19;
    pub const IRON_GOLEM_WITHDRAW_FLOWER: i32 = 20;
    pub const LOVE_PARTICLES: i32 = 21; //breeding
    pub const VILLAGER_ANGRY: i32 = 22;
    pub const VILLAGER_HAPPY: i32 = 23;
    pub const WITCH_SPELL_PARTICLES: i32 = 24;
    pub const FIREWORK_PARTICLES: i32 = 25;
    pub const IN_LOVE_PARTICLES: i32 = 26;
    pub const SILVERFISH_SPAWN_ANIMATION: i32 = 27;
    pub const GUARDIAN_ATTACK: i32 = 28;
    pub const WITCH_DRINK_POTION: i32 = 29;
    pub const WITCH_THROW_POTION: i32 = 30;
    pub const MINECART_TNT_PRIME_FUSE: i32 = 31;
    pub const CREEPER_PRIME_FUSE: i32 = 32;
    pub const AIR_SUPPLY_EXPIRED: i32 = 33;
    pub const PLAYER_ADD_XP_LEVELS: i32 = 34;
    pub const ELDER_GUARDIAN_CURSE: i32 = 35;
    pub const AGENT_ARM_SWING: i32 = 36;
    pub const ENDER_DRAGON_DEATH: i32 = 37;
    pub const DUST_PARTICLES: i32 = 38; //not sure what this is
    pub const ARROW_SHAKE: i32 = 39;

    pub const EATING_ITEM: i32 = 57;

    pub const BABY_ANIMAL_FEED: i32 = 60; //green particles, like bonemeal on crops
    pub const DEATH_SMOKE_CLOUD: i32 = 61;
    pub const COMPLETE_TRADE: i32 = 62;
    pub const REMOVE_LEASH: i32 = 63; //data 1: i32 = cut leash
    pub const CARAVAN_UPDATED: i32 = 64;
    pub const CONSUME_TOTEM: i32 = 65;
    pub const DEPRECATED_UPDATE_STRUCTURE_FEATURE: i32 = 66; //mojang...
    pub const ENTITY_SPAWN: i32 = 67; //used for MinecraftEventing stuff, not needed
    pub const DRAGON_PUKE: i32 = 68; //they call this puke particles
    pub const ITEM_ENTITY_MERGE: i32 = 69;
    pub const START_SWIM: i32 = 70;
    pub const BALLOON_POP: i32 = 71;
    pub const TREASURE_HUNT: i32 = 72;
    pub const AGENT_SUMMON: i32 = 73;
    pub const CHARGED_ITEM: i32 = 74;
    pub const FALL: i32 = 75;
    pub const GROW_UP: i32 = 76;
    pub const VIBRATION_DETECTED: i32 = 77;
    pub const DRINK_MILK: i32 = 78;
    pub const SHAKE_WETNESS_STOP: i32 = 79;
    pub const KINETIC_DAMAGE_DEALT: i32 = 80;
}
