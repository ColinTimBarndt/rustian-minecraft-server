pub struct ChatComponent {
    pub type: ChatComponentType,
    pub color: Option<ChatColor>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    pub insertion: Option<String>,
    pub click_event: Option<ClickEvent>,
    pub hover_event: Option<HoverEvent>,
    pub extra: Vec<ChatComponent>
}

pub enum ChatComponentType {
    Text(pub String),
    Translate {pub key: String, pub with: Vec<String>},
    Score {pub name: String, pub objective: String, pub value: String},
    Selector(pub String),
    Keybind(pub String)
}

/// See the [Minecraft Wiki](https://minecraft.gamepedia.com/Formatting_codes#Color_codes)
pub enum ChatColor {
    Black = 0x0,
    DarkBlue = 0x1,
    DarkGreen = 0x2,
    DarkAqua = 0x3,
    DarkRed = 0x4,
    DarkPurple = 0x5,
    Gold = 0x6,
    Gray = 0x7,
    DarkGray = 0x8,
    Blue = 0x9,
    Green = 0xa,
    Aqua = 0xb,
    Red = 0xc,
    LightPurple = 0xd,
    Yellow = 0xe,
    White = 0xf
}

impl ChatComponent {
    pub fn new(type: ChatComponentType) {
        ChatComponent {
            type,
            ..Default::default()
        }
    }
}

impl Default for ChatComponent {
    fn default() -> Self {
        ChatComponent {
            type: ChatComponentType::Text(String::from("")),
            color: None,
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            insertion: None,
            click_event: None,
            hover_event: None,
            extra: Vec::new()
        }
    }
}