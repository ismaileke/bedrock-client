pub struct PlayerAuthInputFlags {}

/**
 * These flags are used in PlayerAuthInputPacket's inputFlags field.
 * The flags should be written as
 * `flags |= (1 << flag)`
 * and read as
 * `(flags & (1 << flag)) !== 0`
 *
 * See PlayerAuthInput
 */
impl PlayerAuthInputFlags {
    /** Pressing the "fly up" key when using touch. */
    pub const ASCEND: usize = 0;
    /** Pressing the "fly down" key when using touch. */
    pub const DESCEND: usize = 1;
    /** Pressing (and optionally holding) the jump key (while not flying). */
    pub const NORTH_JUMP: usize = 2;
    /** Pressing (and optionally holding) the jump key (including while flying). */
    pub const JUMP_DOWN: usize = 3;
    /** Pressing (and optionally holding) the sprint key (typically the CTRL key). Does not include double-pressing the forward key. */
    pub const SPRINT_DOWN: usize = 4;
    /** Pressing (and optionally holding) the fly button ONCE when in flight mode when using touch. This has no obvious use. */
    pub const CHANGE_HEIGHT: usize = 5;
    /** Pressing (and optionally holding) the jump key (including while flying), and also auto-jumping. */
    pub const JUMPING: usize = 6;
    /** Auto-swimming upwards while pressing forwards with auto-jump enabled. */
    pub const AUTO_JUMPING_IN_WATER: usize = 7;
    /** Sneaking, and pressing the "fly down" key or "sneak" key (including while flying). */
    pub const SNEAKING: usize = 8;
    /** Pressing (and optionally holding) the sneak key (including while flying). This includes when the sneak button is toggled ON with touch controls. */
    pub const SNEAK_DOWN: usize = 9;
    /** Pressing the forward key (typically W on keyboard). */
    pub const UP: usize = 10;
    /** Pressing the backward key (typically S on keyboard). */
    pub const DOWN: usize = 11;
    /** Pressing the left key (typically A on keyboard). */
    pub const LEFT: usize = 12;
    /** Pressing the right key (typically D on keyboard). */
    pub const RIGHT: usize = 13;
    /** Pressing the ↖ key on touch. */
    pub const UP_LEFT: usize = 14;
    /** Pressing the ↗ key on touch. */
    pub const UP_RIGHT: usize = 15;
    /** Client wants to go upwards. Sent when Ascend or Jump is pressed, irrespective of whether flight is enabled. */
    pub const WANT_UP: usize = 16;
    /** Client wants to go downwards. Sent when Descend or Sneak is pressed, irrespective of whether flight is enabled. */
    pub const WANT_DOWN: usize = 17;
    /** Same as "want up" but slow. Only usable with controllers at the time of writing. Triggered by pressing the right joystick by default. */
    pub const WANT_DOWN_SLOW: usize = 18;
    /** Same as "want down" but slow. Only usable with controllers at the time of writing. Not bound to any control by default. */
    pub const WANT_UP_SLOW: usize = 19;
    /** Unclear usage, during testing it was only seen in conjunction with SPRINT_DOWN. NOT sent while actually sprinting. */
    pub const SPRINTING: usize = 20;
    /** Ascending scaffolding. Note that this is NOT sent when climbing ladders. */
    pub const ASCEND_BLOCK: usize = 21;
    /** Descending scaffolding. */
    pub const DESCEND_BLOCK: usize = 22;
    /** Toggling the sneak button on touch when the button enters the "enabled" state. */
    pub const SNEAK_TOGGLE_DOWN: usize = 23;
    /** Unclear use. Sent continually on touch controls, irrespective of whether the player is actually sneaking or not. */
    pub const PERSIST_SNEAK: usize = 24;
    pub const START_SPRINTING: usize = 25;
    pub const STOP_SPRINTING: usize = 26;
    pub const START_SNEAKING: usize = 27;
    pub const STOP_SNEAKING: usize = 28;
    pub const START_SWIMMING: usize = 29;
    pub const STOP_SWIMMING: usize = 30;
    /** Initiating a new jump. Sent every time the client leaves the ground due to jumping, including auto jumps. */
    pub const START_JUMPING: usize = 31;
    pub const START_GLIDING: usize = 32;
    pub const STOP_GLIDING: usize = 33;
    pub const PERFORM_ITEM_INTERACTION: usize = 34;
    pub const PERFORM_BLOCK_ACTIONS: usize = 35;
    pub const PERFORM_ITEM_STACK_REQUEST: usize = 36;
    pub const HANDLED_TELEPORT: usize = 37;
    pub const EMOTING: usize = 38;
    /** Left-clicking the air. In vanilla, this generates an ATTACK_NODAMAGE sound and does nothing else. */
    pub const MISSED_SWING: usize = 39;
    pub const START_CRAWLING: usize = 40;
    pub const STOP_CRAWLING: usize = 41;
    pub const START_FLYING: usize = 42;
    pub const STOP_FLYING: usize = 43;
    pub const ACK_ACTOR_DATA: usize = 44;
    pub const IN_CLIENT_PREDICTED_VEHICLE: usize = 45;
    pub const PADDLING_LEFT: usize = 46;
    pub const PADDLING_RIGHT: usize = 47;
    pub const BLOCK_BREAKING_DELAY_ENABLED: usize = 48;
    pub const HORIZONTAL_COLLISION: usize = 49;
    pub const VERTICAL_COLLISION: usize = 50;
    pub const DOWN_LEFT: usize = 51;
    pub const DOWN_RIGHT: usize = 52;
    pub const START_USING_ITEM: usize = 53;
    pub const IS_CAMERA_RELATIVE_MOVEMENT_ENABLED: usize = 54;
    pub const IS_ROT_CONTROLLED_BY_MOVE_DIRECTION: usize = 55;
    pub const START_SPIN_ATTACK: usize = 56;
    pub const STOP_SPIN_ATTACK: usize = 57;
    pub const IS_HOTBAR_ONLY_TOUCH: usize = 58;
    pub const JUMP_RELEASED_RAW: usize = 59;
    pub const JUMP_PRESSED_RAW: usize = 60;
    pub const JUMP_CURRENT_RAW: usize = 61;
    pub const SNEAK_RELEASED_RAW: usize = 62;
    pub const SNEAK_PRESSED_RAW: usize = 63;
    pub const SNEAK_CURRENT_RAW: usize = 64;

    pub const NUMBER_OF_FLAGS: usize = 65;
}
