use std::collections::HashMap;

#[derive(Debug)]
pub enum JsonValue {
  Int32(i32),
  Int64(i64),
  Float32(f32),
  Float64(f64),
  String(JsonString),
  Object(Vec<NamedValue>),
  Array(Vec<Eval<JsonValue>>),
}

#[derive(Debug)]
pub enum JsonString {
  Owned(String),
  Slice(&'static str),
}

#[derive(Debug)]
pub enum Eval<T> {
  Value(T),
  Variable(u16),
}

#[derive(Debug)]
pub enum Evaluated {
  Text(String),
  Variable(u16),
}

#[derive(Debug)]
pub struct NamedValue {
  pub name: Eval<JsonString>,
  pub value: Eval<JsonValue>,
}

impl JsonString {
  pub fn into_literal(self) -> String {
    let escaped: Vec<String> = self.as_ref().chars().map(escape).collect();
    format!("\"{}\"", escaped.join(""))
  }
}

impl AsRef<str> for JsonString {
  fn as_ref(&self) -> &str {
    match self {
      Self::Owned(ow) => ow.as_ref(),
      Self::Slice(s) => s,
    }
  }
}

impl NamedValue {
  pub fn new_const(name: &'static str, value: JsonValue) -> Self {
    Self {
      name: Eval::Value(JsonString::Slice(name)),
      value: Eval::Value(value),
    }
  }
  pub fn new_var(name: &'static str, var: u16) -> Self {
    Self {
      name: Eval::Value(JsonString::Slice(name)),
      value: Eval::Variable(var),
    }
  }
}

impl<T> Eval<T> {
  pub fn unwrap_variable(self) -> u16 {
    match self {
      Self::Variable(v) => v,
      _ => panic!("Could not unwrap computed value"),
    }
  }
  pub fn unwrap_value(self) -> T {
    match self {
      Self::Value(v) => v,
      _ => panic!("Could not unwrap inner value"),
    }
  }
  pub fn is_value(&self) -> bool {
    match self {
      Self::Value(_) => true,
      _ => false,
    }
  }
}

trait CompileJson: Sized {
  fn compile(self, vec: &mut Vec<Evaluated>);
}

impl CompileJson for JsonValue {
  fn compile(self, vec: &mut Vec<Evaluated>) {
    match self {
      Self::Int32(n) => vec.push(Evaluated::Text(format!("{}", n))),
      Self::Int64(n) => vec.push(Evaluated::Text(format!("{}", n))),
      Self::Float32(n) => vec.push(Evaluated::Text(format!("{}", n))),
      Self::Float64(n) => vec.push(Evaluated::Text(format!("{}", n))),
      Self::String(s) => vec.push(Evaluated::Text(s.into_literal())),
      Self::Object(obj) => obj.compile(vec),
      Self::Array(arr) => arr.compile(vec),
    }
  }
}

impl CompileJson for NamedValue {
  fn compile(self, vec: &mut Vec<Evaluated>) {
    vec.push(match self.name {
      Eval::Variable(v) => Evaluated::Variable(v),
      Eval::Value(val) => Evaluated::Text(val.into_literal()),
    });
    vec.push(Evaluated::Text(":".to_owned()));
    match self.value {
      Eval::Variable(v) => vec.push(Evaluated::Variable(v)),
      Eval::Value(val) => val.compile(vec),
    }
  }
}

impl CompileJson for Vec<NamedValue> {
  fn compile(self, vec: &mut Vec<Evaluated>) {
    if self.len() == 0 {
      vec.push(Evaluated::Text("{}".to_owned()));
      return;
    }
    vec.push(Evaluated::Text("{".to_owned()));
    let mut first = true;
    for nv in self {
      if first {
        first = false;
      } else {
        vec.push(Evaluated::Text(",".to_owned()));
      }
      nv.compile(vec);
    }
    vec.push(Evaluated::Text("}".to_owned()));
  }
}

impl CompileJson for Vec<Eval<JsonValue>> {
  fn compile(self, vec: &mut Vec<Evaluated>) {
    vec.push(Evaluated::Text("[".to_owned()));
    let mut first = true;
    for entry in self {
      if first {
        first = false;
      } else {
        vec.push(Evaluated::Text(",".to_owned()));
      }
      match entry {
        Eval::Value(v) => v.compile(vec),
        Eval::Variable(v) => vec.push(Evaluated::Variable(v)),
      }
    }
    vec.push(Evaluated::Text("]".to_owned()));
  }
}

fn escape(ch: char) -> String {
  match ch {
    '\n' => "\\n".to_owned(),
    '\r' => "\\r".to_owned(),
    '\\' => "\\\\".to_owned(),
    '"' => "\\\"".to_owned(),
    ch => {
      if !ch.is_ascii() {
        let mut buf = Vec::with_capacity(ch.len_utf16());
        ch.encode_utf16(&mut buf);
        buf
          .into_iter()
          .map(|code| format!("\\u{:04X}", code))
          .collect::<Vec<_>>()
          .join("")
      } else {
        ch.to_string()
      }
    }
  }
}

pub fn simplify(vec: Vec<Evaluated>) -> Vec<Evaluated> {
  use std::mem::replace;
  let mut temp = "".to_owned();
  let temp_ref = &mut temp;
  let mut simplified = Vec::new();
  for ev in vec {
    if let Evaluated::Text(text) = ev {
      *temp_ref += &text;
    } else {
      if temp_ref.len() > 0 {
        simplified.push(Evaluated::Text(replace(temp_ref, "".to_owned())));
      }
      simplified.push(ev);
    }
  }
  drop(temp_ref);
  if temp.len() > 0 {
    simplified.push(Evaluated::Text(temp));
  }
  simplified
}

pub fn apply(vec: &Vec<Evaluated>, vars: &HashMap<u16, String>) -> String {
  let mut s = String::with_capacity(
    vec
      .iter()
      .map(|ev| match ev {
        Evaluated::Text(text) => text.len(),
        _ => 10,
      })
      .sum(),
  );
  let null = "null".to_owned();
  for ev in vec {
    match ev {
      Evaluated::Text(text) => s += text,
      Evaluated::Variable(var) => s += vars.get(&var).unwrap_or(&null),
    }
  }
  s
}

/*
#[macro_export]
macro_rules! object {
  {} => ($crate::helpers::fast::json::JsonValue::Object(Vec::new()));
  {$($key:ident : $val:expr),*} => {
    {
      $crate::helpers::fast::json::JsonValue::Object(vec![
        $(
          NamedValue {
            name: Eval::Value(JsonString::Slice(stringify!($key))),
            value: object!(@VAL $val)
          }
        ),*
      ])
    }
  };
  {@VAL $v:literal} => {
    Eval::Value(JsonValue::String(JsonString::Slice($v)))
  }
}
*/

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_obj() {
    let obj = JsonValue::Object(vec![
      NamedValue::new_const("foo", JsonValue::Int32(10)),
      NamedValue::new_var("bar", 1),
      NamedValue::new_const(
        "array",
        JsonValue::Array(vec![Eval::Value(JsonValue::String(JsonString::Slice(
          "baz",
        )))]),
      ),
    ]);
    println!("{:#?}", obj);
    let mut buf = Vec::new();
    obj.compile(&mut buf);
    buf = simplify(buf);
    println!("{:#?}", buf);
    let mut map = HashMap::with_capacity(1);
    map.insert(1, JsonString::Slice("ABC").into_literal());

    assert_eq!(
      apply(&buf, &map),
      r#"{"foo":10,"bar":"ABC","array":["baz"]}"#
    );
  }
}
