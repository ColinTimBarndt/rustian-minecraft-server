//! See this [Minecraft Forum post](https://www.minecraftforum.net/forums/minecraft-java-edition/redstone-discussion-and/351959-1-12-json-text-component-for-tellraw-title-books)
extern crate json;
use json::JsonValue;

/// Represents a chat component object
#[derive(Clone, Debug)]
pub struct ChatComponent {
    pub component_type: ChatComponentType,
    pub color: Option<ChatColor>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    pub insertion: Option<String>,
    pub click_event: Option<ClickEvent>,
    pub hover_event: Option<HoverEvent>,
    pub extra: Vec<ChatComponent>,
}

/// Represents the attributes of specific chat component types
#[derive(Clone, Debug)]
pub enum ChatComponentType {
    Text(String),
    Translate {
        key: String,
        with: Vec<ChatComponent>,
    },
    Score {
        name: String,
        objective: String,
        value: Option<String>,
    },
    Selector(String),
    Keybind(String),
}
impl ChatComponentType {
    pub fn make_json(&self) -> JsonValue {
        use ChatComponentType::*;
        let mut obj = JsonValue::new_object();
        match self {
            Text(text) => obj.insert("text", JsonValue::String(text.clone())).unwrap(),
            Translate { key, with } => {
                obj.insert("translate", JsonValue::String(key.clone()))
                    .unwrap();
                if with.len() > 0 {
                    obj.insert(
                        "with",
                        JsonValue::Array(with.iter().map(|cmp| cmp.make_json()).collect()),
                    );
                }
            }
            Score {
                name,
                objective,
                value,
            } => {
                let mut score = JsonValue::new_object();
                score.insert("name", name.clone()).unwrap();
                score.insert("objective", objective.clone()).unwrap();
                if let Some(value) = value {
                    score.insert("value", value.clone()).unwrap();
                }
                obj.insert("score", score).unwrap();
            }
            Selector(sel) => obj
                .insert("selector", JsonValue::String(sel.clone()))
                .unwrap(),
            Keybind(action) => obj
                .insert("keybind", JsonValue::String(action.clone()))
                .unwrap(),
        }
        obj
    }
}

pub mod keybinds {
    macro_rules! keybind {
        ($var:ident $val:literal) => {
            pub const $var: &str = $val;
        };
    }
    keybind!(FORWARD "key.forward");
    keybind!(LEFT "key.left");
    keybind!(BACK "key.back");
    keybind!(RIGHT "key.right");
    keybind!(JUMP "key.jump");
    keybind!(SNEAK "key.sneak");
    keybind!(SPRINT "key.sprint");
    keybind!(INVENTORY "key.inventory");
    keybind!(SWAP_HANDS "key.swapHands");
    keybind!(DROP "key.drop");
    keybind!(USE "key.use");
    keybind!(ATTACK "key.attack");
    keybind!(PICK_ITEM "key.pickItem");
    keybind!(CHAT "key.chat");
    keybind!(PLAYERLIST "key.playerlist");
    keybind!(COMMAND "key.command");
    keybind!(SCREENSHOT "key.screenshot");
    keybind!(TOGGLE_PERSPECTIVE "key.togglePerspective");
    keybind!(SMOOTH_CAMERA "key.smoothCamera");
    keybind!(FULLSCREEN "key.fullscreen");
    keybind!(SPECTATOR_OUTLINES "key.spectatorOutlines");
    keybind!(HOTBAR_1 "key.hotbar.1");
    keybind!(HOTBAR_2 "key.hotbar.2");
    keybind!(HOTBAR_3 "key.hotbar.3");
    keybind!(HOTBAR_4 "key.hotbar.4");
    keybind!(HOTBAR_5 "key.hotbar.5");
    keybind!(HOTBAR_6 "key.hotbar.6");
    keybind!(HOTBAR_7 "key.hotbar.7");
    keybind!(HOTBAR_8 "key.hotbar.8");
    keybind!(HOTBAR_9 "key.hotbar.9");
    keybind!(SAVE_TOOLBAR_ACTIVATOR "key.saveToolbarActivator");
    keybind!(LOAD_TOOLBAR_ACTIVATOR "key.loadToolbarActivator");
}

/// See the [Minecraft Wiki](https://minecraft.gamepedia.com/Formatting_codes#Color_codes)
#[derive(Debug, Clone, Copy)]
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
    White = 0xf,
}
impl From<ChatColor> for String {
    fn from(color: ChatColor) -> Self {
        use ChatColor::*;
        match color {
            Black => "black",
            DarkBlue => "dark_blue",
            DarkGreen => "dark_green",
            DarkAqua => "dark_aqua",
            DarkRed => "dark_red",
            DarkPurple => "dark_purple",
            Gold => "gold",
            Gray => "gray",
            DarkGray => "dark_gray",
            Blue => "blue",
            Green => "green",
            Aqua => "aqua",
            Red => "red",
            LightPurple => "light_purple",
            Yellow => "yellow",
            White => "white",
        }
        .to_string()
    }
}

impl ChatComponent {
    pub fn new(component_type: ChatComponentType) -> Self {
        ChatComponent {
            component_type,
            ..Default::default()
        }
    }
    pub fn make_json(&self) -> JsonValue {
        let mut obj = self.component_type.make_json();
        let mut is_blank = if let ChatComponentType::Text(txt) = &self.component_type {
            Some(txt)
        } else {
            None
        };
        if let Some(color) = self.color {
            obj.insert("color", JsonValue::String(color.into()))
                .unwrap();
            is_blank = None;
        }
        if let Some(bold) = self.bold {
            obj.insert("bold", JsonValue::Boolean(bold)).unwrap();
            is_blank = None;
        }
        if let Some(italic) = self.italic {
            obj.insert("italic", JsonValue::Boolean(italic)).unwrap();
            is_blank = None;
        }
        if let Some(underlined) = self.underlined {
            obj.insert("underlined", JsonValue::Boolean(underlined))
                .unwrap();
            is_blank = None;
        }
        if let Some(strikethrough) = self.strikethrough {
            obj.insert("strikethrough", JsonValue::Boolean(strikethrough))
                .unwrap();
            is_blank = None;
        }
        if let Some(obfuscated) = self.obfuscated {
            obj.insert("obfuscated", JsonValue::Boolean(obfuscated))
                .unwrap();
            is_blank = None;
        }
        if let Some(insertion) = &self.insertion {
            obj.insert("insertion", JsonValue::String(insertion.clone()))
                .unwrap();
            is_blank = None;
        }
        if let Some(hover) = &self.hover_event {
            obj.insert("hover_event", hover.make_json()).unwrap();
            is_blank = None;
        }
        if let Some(click) = &self.click_event {
            obj.insert("click_event", click.make_json()).unwrap();
            is_blank = None;
        }
        if let Some(txt) = is_blank {
            JsonValue::String(txt.clone())
        } else {
            obj
        }
    }
    pub fn set_color(mut self, color: ChatColor) -> Self {
        self.color = Some(color);
        self
    }
    pub fn set_bold(mut self, bold: bool) -> Self {
        self.bold = Some(bold);
        self
    }
    pub fn set_italic(mut self, italic: bool) -> Self {
        self.italic = Some(italic);
        self
    }
    pub fn set_underlined(mut self, underlined: bool) -> Self {
        self.underlined = Some(underlined);
        self
    }
    pub fn set_strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = Some(strikethrough);
        self
    }
    pub fn set_obfuscated(mut self, obfuscated: bool) -> Self {
        self.obfuscated = Some(obfuscated);
        self
    }
    pub fn set_insertion(mut self, insertion: String) -> Self {
        self.insertion = Some(insertion);
        self
    }
    pub fn set_hover_event(mut self, hover_event: HoverEvent) -> Self {
        self.hover_event = Some(hover_event);
        self
    }
    pub fn set_click_event(mut self, click_event: ClickEvent) -> Self {
        self.click_event = Some(click_event);
        self
    }
    pub fn add_extra(&mut self, extra: ChatComponent) {
        self.extra.push(extra);
    }
}

impl Default for ChatComponent {
    fn default() -> Self {
        ChatComponent {
            component_type: ChatComponentType::Text(String::from("")),
            color: None,
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            insertion: None,
            click_event: None,
            hover_event: None,
            extra: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ClickEvent {
    OpenUrl(String),
    RunCommand(String),
    SuggestCommand(String),
    ChangePage(u32),
}

impl ClickEvent {
    pub fn make_json(&self) -> JsonValue {
        use ClickEvent::*;
        let mut obj = JsonValue::new_object();
        let res: (&str, JsonValue) = match self {
            OpenUrl(url) => ("open_url", JsonValue::String(url.clone())),
            RunCommand(cmd) => ("run_command", JsonValue::String(cmd.clone())),
            SuggestCommand(cmd) => ("suggest_command", JsonValue::String(cmd.clone())),
            ChangePage(page) => ("change_page", JsonValue::String(page.to_string())),
        };
        obj.insert("action", JsonValue::String(res.0.to_string()))
            .unwrap();
        obj.insert("value", res.1).unwrap();
        obj
    }
}

#[derive(Clone, Debug)]
pub enum HoverEvent {
    ShowText(String),
    ShowComponentText(Vec<ChatComponent>),
}

impl HoverEvent {
    pub fn make_json(&self) -> JsonValue {
        use HoverEvent::*;
        let mut obj = JsonValue::new_object();
        let res: (&str, JsonValue) = match self {
            ShowText(text) => ("show_text", JsonValue::String(text.clone())),
            ShowComponentText(cmps) => (
                "show_text",
                JsonValue::Array(cmps.iter().map(|cmp| cmp.make_json()).collect()),
            ),
        };
        obj.insert("action", JsonValue::String(res.0.to_string()))
            .unwrap();
        obj.insert("value", res.1).unwrap();
        obj
    }
}
