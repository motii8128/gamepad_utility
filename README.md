# gamepad_utility

|test|status|
|:--:|:--:|
|Rust|[![Rust](https://github.com/motii8128/gamepad_utility/actions/workflows/rust.yml/badge.svg)](https://github.com/motii8128/gamepad_utility/actions/workflows/rust.yml)|

# 使用方法
まずCargo.tomlの`dependencies`の下に以下のように追加してください。
```toml
[dependencies]
gamepad_utility = {git = "https://github.com/motii8128/gamepad_utility.git"}
```
# 例１
１台のコントローラーの値を取得するコードの例を以下に示します。
```rs
// ライブラリをもってくる
use gamepad_utility::GamePadsDriver;

fn main() {
    // ドライバーの初期化
    let mut driver = GamePadsDriver::new();

    // ここからループ
    loop {
        // ドライバーを毎回アップデートする必要がある。
        //　値を取得する前に絶対実行する
        driver.update();

        // コントローラーが接続されているかをチェックする
        // 接続されていると「true」が返ってくるのでif文の中に入る
        if driver.is_connected()
        {
            // １台めの値を取得する
            let data = driver.get(1);

            // 左スティックの横方向の値(-1.0 ~ 1.0)を表示する。
            println!("LeftStick x axis : {}", data.left_stick.x);
        }
    }
}
```

# 例２
複数のコントローラーを扱う例を以下に示します。
```rs
// ライブラリをもってくる
use gamepad_utility::GamePadsDriver;

fn main() {
    // ドライバーの初期化
    let mut driver = GamePadsDriver::new();

    // ここからループ
    loop {
        // ドライバーを毎回アップデートする必要がある。
        //　値を取得する前に絶対実行する
        driver.update();

        // コントローラーの接続台数を取得する
        if driver.controller_num() == 1
        {
            let controller1 = driver.get(1);

            // 丸ボタンの値を取得。ボタンは押されていたら「１」、押されていなかったら「０」
            println!("1 controller is connected. Circle Button: {}", controller1.btns.circle);
        }
        else if driver.controller_num() == 2
        {
            // １台めの値を取得
            let controller1 = driver.get(1);
            // ２台めの値を取得
            let controller2 = driver.get(2);

            println!("2 controller is connected. CircleButton1:{}, CircleButton2:{}", controller1.btns.circle, controller2.btns.circle);
        }
    }
}
```
