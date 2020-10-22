//! Naming convention: STATE _ SB/CB (server- or clientbound) _ NAME

macro_rules! define {
  ($($name:ident = $id:literal ;)*) => {
    $(
      pub const $name : u32 = $id ;
    )*
  };
}

define! {
  // Handshake
  HANDSHAKE_SB_HANDSHAKE = 0x00;
  // Status
  STATUS_CB_RESPONSE = 0x00;
  STATUS_CB_PONG = 0x01;
  STATUS_SB_REQUEST = 0x00;
  STATUS_SB_PING = 0x01;
  // Login
  LOGIN_CB_DISCONNECT = 0x00;
  LOGIN_CB_ENCRYPTION_REQUEST = 0x01;
  LOGIN_CB_LOGIN_SUCCESS = 0x02;
  LOGIN_CB_SET_COMPRESSION = 0x03;
  LOGIN_CB_LOGIN_PLUGIN_REQUEST = 0x04;
  LOGIN_SB_LOGIN_START = 0x00;
  LOGIN_SB_ENCRYPTION_RESPONSE = 0x01;
  LOGIN_SB_LOGIN_PLUGIN_RESPONSE = 0x02;
  // Play
  PLAY_CB_SPAWN_ENTITY = 0x00;
  PLAY_CB_SPAWN_EXPERIENCE_ORB = 0x01;
  PLAY_CB_SPAWN_WEATHER_ENTITY = 0x02;
  PLAY_CB_SPAWN_LIVING_ENTITY = 0x03;
  PLAY_CB_SPAWN_PAINTING = 0x04;
  PLAY_CB_SPAWN_PLAYER = 0x05;
  PLAY_CB_ENTITY_ANIMATION = 0x06;
  PLAY_CB_STATISTICS = 0x07;
  PLAY_CB_ACKNOWLEDGE_PLAYER_DIGGING = 0x08;
  PLAY_CB_BLOCK_BREAK_ANIMATION = 0x09;
  PLAY_CB_BLOCK_ENTITY_DATA = 0x0A;
  PLAY_CB_BLOCK_ACTION = 0x0B;
  PLAY_CB_BLOCK_CHANGE = 0x0C;
  PLAY_CB_BOSS_BAR = 0x0D;
  PLAY_CB_SERVER_DIFFICULTY = 0x0E;
  PLAY_CB_CHAT_MESSAGE = 0x0F;
  PLAY_CB_MULTI_BLOCK_CHANGE = 0x10;
  PLAY_CB_TAB_COMPLETE = 0x11;
  PLAY_CB_DECLARE_COMMANDS = 0x12;
  PLAY_CB_WINDOW_CONFIRMATION = 0x13;
  PLAY_CB_CLOSE_WINDOW = 0x14;
  PLAY_CB_WINDOW_ITEMS = 0x15;
  PLAY_CB_WINDOW_PROPERTY = 0x16;
  PLAY_CB_SET_SLOT = 0x17;
  PLAY_CB_SET_COOLDOWN = 0x18;
  PLAY_CB_PLUGIN_MESSAGE = 0x19;
  PLAY_CB_NAMED_SOUND_EFFECT = 0x1A;
  PLAY_CB_DISCONNECT = 0x1B;
  PLAY_CB_ENTITY_STATUS = 0x1C;
  PLAY_CB_EXPLOSION = 0x1D;
  PLAY_CB_UNLOAD_CHUNK = 0x1E;
  PLAY_CB_CHANGE_GAME_STATE = 0x1F;
  PLAY_CB_OPEN_HORSE_WINDOW = 0x20;
  PLAY_CB_KEEP_ALIVE = 0x21;
  PLAY_CB_CHUNK_DATA = 0x22;
  PLAY_CB_EFFECT = 0x23;
  PLAY_CB_PARTICLE = 0x24;
  PLAY_CB_UPDATE_LIGHT = 0x25;
  PLAY_CB_JOIN_GAME = 0x26;
  PLAY_CB_MAP_DATA = 0x27;
  PLAY_CB_TRADE_LIST = 0x28;
  PLAY_CB_ENTITY_POSITION = 0x29;
  PLAY_CB_ENTITY_POSITION_AND_ROTATION = 0x2A;
  PLAY_CB_ENTITY_ROTATION = 0x2B;
  PLAY_CB_ENTITY_MOVEMENT = 0x2C;
  PLAY_CB_VEHICLE_MOVE = 0x2D;
  PLAY_CB_OPEN_BOOK = 0x2E;
  PLAY_CB_OPEN_WINDOW = 0x2F;
  PLAY_CB_OPEN_SIGN_EDITOR = 0x30;
  PLAY_CB_CRAFT_RECIPE_RESPONSE = 0x31;
  PLAY_CB_PLAYER_ABILITIES = 0x32;
  PLAY_CB_COMBAT_EVENT = 0x33;
  PLAY_CB_PLAYER_INFO = 0x34;
  PLAY_CB_FACE_PLAYER = 0x35;
  PLAY_CB_PLAYER_POSITION_AND_LOOK = 0x36;
  PLAY_CB_UNLOCK_RECIPES = 0x37;
  PLAY_CB_DESTROY_ENTITIES = 0x38;
  PLAY_CB_REMOVE_ENTITY_EFFECT = 0x39;
  PLAY_CB_RESOURCE_PACK_SEND = 0x3A;
  PLAY_CB_RESPAWN = 0x3B;
  PLAY_CB_ENTITY_HEAD_LOOK = 0x3C;
  PLAY_CB_SELECT_ADVANCEMENT_TAB = 0x3D;
  PLAY_CB_WORLD_BORDER = 0x3E;
  PLAY_CB_CAMERA = 0x3F;
  PLAY_CB_HELD_ITEM_CHANGE = 0x40;
  PLAY_CB_UPDATE_VIEW_POSITION = 0x41;
  PLAY_CB_UPDATE_VIEW_DISTANCE = 0x42;
  PLAY_CB_DISPLAY_SCOREBOARD = 0x43;
  PLAY_CB_ENTITY_METADATA = 0x44;
  PLAY_CB_ATTACH_ENTITY = 0x45;
  PLAY_CB_ENTITY_VELOCITY = 0x46;
  PLAY_CB_ENTITY_EQUIPMENT = 0x47;
  PLAY_CB_SET_EXPERIENCE = 0x48;
  PLAY_CB_UPDATE_HEALTH = 0x49;
  PLAY_CB_SCOREBOARD_OBJECTIVE = 0x4A;
  PLAY_CB_SET_PASSENGERS = 0x4B;
  PLAY_CB_TEAMS = 0x4C;
  PLAY_CB_UPDATE_SCORE = 0x4D;
  PLAY_CB_SPAWN_POSITION = 0x4E;
  PLAY_CB_TIME_UPDATE = 0x4F;
  PLAY_CB_TITLE = 0x50;
  PLAY_CB_ENTITY_SOUND_EFFECT = 0x51;
  PLAY_CB_SOUND_EFFECT = 0x52;
  PLAY_CB_STOP_SOUND = 0x53;
  PLAY_CB_PLAYER_LIST_HEADER_AND_FOOTER = 0x54;
  PLAY_CB_NBT_QUERY_RESPONSE = 0x55;
  PLAY_CB_COLLECT_ITEM = 0x56;
  PLAY_CB_ENTITY_TELEPORT = 0x57;
  PLAY_CB_ADVANCEMENTS = 0x58;
  PLAY_CB_ENTITY_PROPERTIES = 0x59;
  PLAY_CB_ENTITY_EFFECT = 0x5A;
  PLAY_CB_DECLARE_RECIPES = 0x5B;
  PLAY_CB_TAGS = 0x5C;
  PLAY_SB_TELEPORT_CONFIRM = 0x00;
  PLAY_SB_QUERY_BLOCK_NBT = 0x01;
  PLAY_SB_SET_DIFFICULTY = 0x02;
  PLAY_SB_CHAT_MESSAGE = 0x03;
  PLAY_SB_CLIENT_STATUS = 0x04;
  PLAY_SB_CLIENT_SETTINGS = 0x05;
  PLAY_SB_TAB_COMPLETE = 0x06;
  PLAY_SB_WINDOW_CONFIRMATION = 0x07;
  PLAY_SB_CLICK_WINDOW_BUTTON = 0x08;
  PLAY_SB_CLICK_WINDOW = 0x09;
  PLAY_SB_CLOSE_WINDOW = 0x0A;
  PLAY_SB_PLUGIN_MESSAGE = 0x0B;
  PLAY_SB_EDIT_BOOK = 0x0C;
  PLAY_SB_ENTITY_NBT_REQUEST = 0x0D;
  PLAY_SB_INTERACT_ENTITY = 0x0E;
  PLAY_SB_KEEP_ALIVE = 0x0F;
  PLAY_SB_LOCK_DIFFICULTY = 0x10;
  PLAY_SB_PLAYER_POSITION = 0x11;
  PLAY_SB_PLAYER_POSITION_AND_ROTATION = 0x12;
  PLAY_SB_PLAYER_ROTATION = 0x13;
  PLAY_SB_PLAYER_MOVEMENT = 0x14;
  PLAY_SB_VEHICLE_MOVE = 0x15;
  PLAY_SB_STEER_BOAT = 0x16;
  PLAY_SB_PICK_ITEM = 0x17;
  PLAY_SB_CRAFT_RECIPE_REQUEST = 0x18;
  PLAY_SB_PLAYER_ABILITIES = 0x19;
  PLAY_SB_PLAYER_DIGGING = 0x1A;
  PLAY_SB_ENTITY_ACTION = 0x1B;
  PLAY_SB_STEER_VEHICLE = 0x1C;
  PLAY_SB_RECIPE_BOOK_DATA = 0x1D;
  PLAY_SB_NAME_ITEM = 0x1E;
  PLAY_SB_RECOURCE_PACK_STATUS = 0x1F;
  PLAY_SB_ADVANCEMENT_TAB = 0x20;
  PLAY_SB_SELECT_TRADE = 0x21;
  PLAY_SB_SET_BEACON_EFFECT = 0x22;
  PLAY_SB_HELD_ITEM_CHANGE = 0x23;
  PLAY_SB_UPDATE_COMMAND_BLOCK = 0x24;
  PLAY_SB_UPDATE_COMMAND_BLOCK_MINECART = 0x25;
  PLAY_SB_CREATIVE_INVENTORY_ACTION = 0x26;
  PLAY_SB_UPDATE_JIGSAW_BLOCK = 0x27;
  PLAY_SB_UPDATE_STRUCTURE_BLOCK = 0x28;
  PLAY_SB_UPDATE_SIGN = 0x29;
  PLAY_SB_ANIMATION = 0x2A;
  PLAY_SB_SPECTATE = 0x2B;
  PLAY_SB_PLAYER_BLOCK_PLACEMENT = 0x2C;
  PLAY_SB_USE_ITEM = 0x2D;
}