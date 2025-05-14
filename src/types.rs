use serde::{Deserialize, Serialize};
use serde_json;

/// Json形式文字列への変換、もしくは文字列から構造体を取得する関数を提供するトレイト
trait JsonParse
where
    Self: Serialize + for<'de> Deserialize<'de> + Sized + Clone,
{
    /// Json形式の文字列に変換する
    fn create_packet(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Json形式の文字列を構造体に変換する
    fn from_packet(packet: &str) -> Self {
        serde_json::from_str(packet).unwrap()
    }
}

/// スティックと十字キーに使う
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Axis
{
    pub x: f32,
    pub y: f32,
}
impl JsonParse for Axis {}

/// ボタン郡
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Buttons
{
    pub circle : i8,
    pub triangle : i8,
    pub cube : i8,
    pub cross : i8,
    pub r1 : i8,
    pub r2 : i8,
    pub l1 : i8,
    pub l2 : i8
}
impl JsonParse for Buttons {}

/// 取得したコントローラーデータを格納する
/// * `left_stick` - 左スティックの縦横データ(float)
/// * `right_stick` - 右スティックの縦横データ(float)
/// * `dpad` - 十字キーの縦横データ(float)
/// * `btns` - 各ボタンのデータ。(int)
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ControllerData
{
    pub left_stick : Axis,
    pub right_stick : Axis,
    pub dpad : Axis,
    pub btns : Buttons
}
impl JsonParse for ControllerData {}