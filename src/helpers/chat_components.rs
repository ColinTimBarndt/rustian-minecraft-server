//! See this [Minecraft Forum post](https://www.minecraftforum.net/forums/minecraft-java-edition/redstone-discussion-and/351959-1-12-json-text-component-for-tellraw-title-books)

use super::fast::json::escape_write;
use serde_json::Map;
use serde_json::Value as JsonValue;
use std::borrow::Cow;

type CowStr = Cow<'static, str>;

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
    pub insertion: Option<CowStr>,
    pub click_event: Option<ClickEvent>,
    pub hover_event: Option<HoverEvent>,
    pub extra: Vec<ChatComponent>,
}

/// Represents the attributes of specific chat component types
#[derive(Clone, Debug)]
pub enum ChatComponentType {
    Text(CowStr),
    Translate {
        key: CowStr,
        with: Vec<ChatComponent>,
    },
    Score {
        name: CowStr,
        objective: CowStr,
        value: Option<CowStr>,
    },
    Selector(CowStr),
    Keybind(CowStr),
}

impl ChatComponentType {
    fn make_json(&self) -> Map<String, JsonValue> {
        use ChatComponentType::*;
        let mut obj = Map::with_capacity(10);
        match self {
            Text(text) => {
                obj.insert("text".to_owned(), JsonValue::String(text.clone().into()));
            }
            Translate { key, with } => {
                obj.insert(
                    "translate".to_owned(),
                    JsonValue::String(key.clone().into()),
                );
                if with.len() > 0 {
                    obj.insert(
                        "with".to_owned(),
                        JsonValue::Array(with.into_iter().map(|cmp| cmp.make_json()).collect()),
                    );
                }
            }
            Score {
                name,
                objective,
                value,
            } => {
                let mut score = Map::with_capacity(2);
                score.insert("name".to_owned(), JsonValue::String(name.clone().into()));
                score.insert(
                    "objective".to_owned(),
                    JsonValue::String(objective.clone().into()),
                );
                if let Some(value) = value {
                    score.insert("value".to_owned(), JsonValue::String(value.clone().into()));
                }
                obj.insert("score".to_owned(), JsonValue::Object(score));
            }
            Selector(sel) => {
                obj.insert("selector".to_owned(), JsonValue::String(sel.clone().into()));
            }
            Keybind(action) => {
                obj.insert(
                    "keybind".to_owned(),
                    JsonValue::String(action.clone().into()),
                );
            }
        }
        obj
    }
    #[inline]
    fn serialize_json(&self, buffer: &mut Vec<u8>) {
        use ChatComponentType::*;
        match self {
            Text(ref text) => {
                buffer.extend_from_slice(br#"{"text":""#);
                escape_write(text, buffer);
                buffer.push(b'"');
            }
            Translate { ref key, ref with } => {
                {
                    let len = key.len();
                    buffer.reserve(25 + len + (len >> 3));
                }
                buffer.extend_from_slice(br#"{"translate":""#);
                escape_write(key, buffer);
                buffer.extend_from_slice(br#"","with":["#);
                {
                    let mut iter = with.iter();
                    if let Some(i) = iter.next() {
                        i.serialize_json(buffer);
                        for i in iter {
                            buffer.push(b',');
                            i.serialize_json(buffer);
                        }
                    }
                }
                buffer.push(b']');
            }
            Score {
                ref name,
                ref objective,
                value: ref value_opt,
            } => {
                buffer.extend_from_slice(br#"{"score":{"name":""#);
                escape_write(name, buffer);
                buffer.extend_from_slice(br#"","objective":""#);
                escape_write(objective, buffer);
                if let Some(value) = value_opt {
                    buffer.extend_from_slice(br#"","value":""#);
                    buffer.extend(value.to_string().as_bytes());
                }
                buffer.extend_from_slice(br#""}"#);
            }
            Selector(sel) => {
                buffer.extend_from_slice(br#"{"selector":""#);
                escape_write(sel, buffer);
                buffer.push(b'"');
            }
            Keybind(action) => {
                buffer.extend_from_slice(br#"{"keybind":""#);
                escape_write(action, buffer);
                buffer.push(b'"');
            }
        }
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
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
    /// Implemented for Minecraft 1.16 and above
    Custom(super::Color),
}
impl ChatColor {
    pub fn code(&self) -> &'static str {
        use ChatColor::*;
        match self {
            Black => "§0",
            DarkBlue => "§1",
            DarkGreen => "§2",
            DarkAqua => "§3",
            DarkRed => "§4",
            DarkPurple => "§5",
            Gold => "§6",
            Gray => "§7",
            DarkGray => "§8",
            Blue => "§9",
            Green => "§a",
            Aqua => "§b",
            Red => "§c",
            LightPurple => "§d",
            Yellow => "§e",
            White => "§f",
            Custom(_) => "",
        }
    }
}
impl From<&ChatColor> for CowStr {
    fn from(color: &ChatColor) -> Self {
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
            Custom(color) => return color.to_hex_code().into(),
        }
        .into()
    }
}

impl ChatComponent {
    pub fn new(component_type: ChatComponentType) -> Self {
        ChatComponent {
            component_type,
            ..Default::default()
        }
    }
    pub fn text(text: impl Into<CowStr>) -> Self {
        ChatComponent::new(ChatComponentType::Text(text.into()))
    }
    pub fn translate(key: impl Into<CowStr>, with: Vec<ChatComponent>) -> Self {
        ChatComponent::new(ChatComponentType::Translate {
            key: key.into(),
            with,
        })
    }
    pub fn make_json(&self) -> JsonValue {
        if self.color.is_none()
            && self.bold.is_none()
            && self.italic.is_none()
            && self.underlined.is_none()
            && self.underlined.is_none()
            && self.strikethrough.is_none()
            && self.obfuscated.is_none()
            && self.insertion.is_none()
            && self.hover_event.is_none()
            && self.click_event.is_none()
        {
            if let ChatComponentType::Text(txt) = &self.component_type {
                return JsonValue::String(txt.clone().into());
            }
        }
        let mut obj = self.component_type.make_json();
        if let Some(color) = self.color.as_ref() {
            obj.insert(
                "color".to_owned(),
                JsonValue::String(CowStr::from(color).into()),
            );
        }
        if let Some(bold) = self.bold {
            obj.insert("bold".to_owned(), JsonValue::Bool(bold));
        }
        if let Some(italic) = self.italic {
            obj.insert("italic".to_owned(), JsonValue::Bool(italic));
        }
        if let Some(underlined) = self.underlined {
            obj.insert("underlined".to_owned(), JsonValue::Bool(underlined));
        }
        if let Some(strikethrough) = self.strikethrough {
            obj.insert("strikethrough".to_owned(), JsonValue::Bool(strikethrough));
        }
        if let Some(obfuscated) = self.obfuscated {
            obj.insert("obfuscated".to_owned(), JsonValue::Bool(obfuscated));
        }
        if let Some(insertion) = &self.insertion {
            obj.insert(
                "insertion".to_owned(),
                JsonValue::String(insertion.clone().into()),
            );
        }
        if let Some(hover) = &self.hover_event {
            obj.insert(
                "hover_event".to_owned(),
                JsonValue::Object(hover.make_json()),
            );
        }
        if let Some(click) = &self.click_event {
            obj.insert(
                "click_event".to_owned(),
                JsonValue::Object(click.make_json()),
            );
        }
        JsonValue::Object(obj)
    }
    /// Function optimized for serialiting a chat component to
    /// a buffer by minimizing memory allocation.
    pub fn serialize_json(&self, buffer: &mut Vec<u8>) {
        if self.color.is_none()
            && self.bold.is_none()
            && self.italic.is_none()
            && self.underlined.is_none()
            && self.underlined.is_none()
            && self.strikethrough.is_none()
            && self.obfuscated.is_none()
            && self.insertion.is_none()
            && self.hover_event.is_none()
            && self.click_event.is_none()
        {
            if let ChatComponentType::Text(txt) = &self.component_type {
                buffer.push(b'"');
                escape_write(txt, buffer);
                buffer.push(b'"');
                return;
            }
        }
        self.component_type.serialize_json(buffer);
        let bools = [&b"false"[..], &b"true"[..]];
        if let Some(color) = self.color {
            buffer.extend_from_slice(br#","color":""#);
            buffer.extend_from_slice(CowStr::from(&color).as_bytes());
            buffer.push(b'"');
        }
        if let Some(bold) = self.bold {
            buffer.extend_from_slice(br#","bold":"#);
            buffer.extend_from_slice(bools[bold as usize]);
        }
        if let Some(italic) = self.italic {
            buffer.extend_from_slice(br#","italic":"#);
            buffer.extend_from_slice(bools[italic as usize]);
        }
        if let Some(underlined) = self.underlined {
            buffer.extend_from_slice(br#","underlined":"#);
            buffer.extend_from_slice(bools[underlined as usize]);
        }
        if let Some(strikethrough) = self.strikethrough {
            buffer.extend_from_slice(br#","strikethrough":"#);
            buffer.extend_from_slice(bools[strikethrough as usize]);
        }
        if let Some(obfuscated) = self.obfuscated {
            buffer.extend_from_slice(br#","obfuscated":"#);
            buffer.extend_from_slice(bools[obfuscated as usize]);
        }
        if let Some(ref insertion) = self.insertion {
            buffer.extend_from_slice(br#","insertion":""#);
            escape_write(insertion, buffer);
            buffer.push(b'"');
        }
        if let Some(ref hover) = self.hover_event {
            buffer.extend_from_slice(br#","hover_event":{"#);
            hover.serialize_json(buffer);
            buffer.push(b'}');
        }
        if let Some(ref click) = self.click_event {
            buffer.extend_from_slice(br#","click_event":{"#);
            click.serialize_json(buffer);
            buffer.push(b'}');
        }
        buffer.push(b'}');
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
        self.insertion = Some(insertion.into());
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
            component_type: ChatComponentType::Text("".into()),
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
    pub fn make_json(&self) -> Map<String, JsonValue> {
        use ClickEvent::*;
        let mut obj = Map::with_capacity(2);
        let res: (&str, JsonValue) = match self {
            OpenUrl(url) => ("open_url", JsonValue::String(url.clone())),
            RunCommand(cmd) => ("run_command", JsonValue::String(cmd.clone())),
            SuggestCommand(cmd) => ("suggest_command", JsonValue::String(cmd.clone())),
            ChangePage(page) => ("change_page", JsonValue::String(page.to_string())),
        };
        obj.insert("action".to_owned(), JsonValue::String(res.0.to_owned()));
        obj.insert("value".to_owned(), res.1);
        obj
    }
    #[inline]
    fn serialize_json(&self, buffer: &mut Vec<u8>) {
        use ClickEvent::*;
        match self {
            OpenUrl(ref url) => {
                buffer.extend_from_slice(br#""action":"open_url","value":""#);
                escape_write(url, buffer);
            }
            RunCommand(ref cmd) => {
                buffer.extend_from_slice(br#""action":"run_command","value":""#);
                escape_write(cmd, buffer);
            }
            SuggestCommand(ref cmd) => {
                buffer.extend_from_slice(br#""action":"suggest_command","value":""#);
                escape_write(cmd, buffer);
            }
            ChangePage(ref page) => {
                buffer.extend_from_slice(br#""action":"change_page","value":""#);
                buffer.extend_from_slice(page.to_string().as_bytes());
            }
        }
        buffer.push(b'"');
    }
}

#[derive(Clone, Debug)]
pub enum HoverEvent {
    ShowText(String),
    ShowComponentText(Vec<ChatComponent>),
}

impl HoverEvent {
    pub fn make_json(&self) -> Map<String, JsonValue> {
        use HoverEvent::*;
        let mut obj = Map::with_capacity(2);
        let res: (&str, JsonValue) = match self {
            ShowText(text) => ("show_text", JsonValue::String(text.clone())),
            ShowComponentText(cmps) => (
                "show_text",
                JsonValue::Array(cmps.into_iter().map(|cmp| cmp.make_json()).collect()),
            ),
        };
        obj.insert("action".to_owned(), JsonValue::String(res.0.to_owned()));
        obj.insert("value".to_owned(), res.1);
        obj
    }
    #[inline]
    fn serialize_json(&self, buffer: &mut Vec<u8>) {
        use HoverEvent::*;
        match self {
            ShowText(ref text) => {
                buffer.extend_from_slice(br#""action":"show_text","value":""#);
                escape_write(text, buffer);
                buffer.push(b'"');
            }
            ShowComponentText(ref cmps) => {
                buffer.extend_from_slice(br#""action":"show_text","value":["#);
                let mut iter = cmps.iter();
                if let Some(cmp) = iter.next() {
                    cmp.serialize_json(buffer);
                    for cmp in iter {
                        buffer.push(b',');
                        cmp.serialize_json(buffer);
                    }
                }
                buffer.push(b']');
            }
        }
    }
}
