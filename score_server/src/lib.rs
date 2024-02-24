fn sample_code_add(mut n: u64, mut m: u64) -> u64 {
    n + m
}

// テスト実行時のみコンパイルされる
#[cfg(test)]
// テスト名でsnake_case以外の名称を採用したい場合の処理
#[allow(non_snake_case)]
mod sample_mathテスト{
    // テストモジュール内でクレートを利用する
    use super::*;

    // 1つのテストとして実行
    #[test]
    fn sample_code_add関数は2つの引数の加算を行う() {
        assert_eq!(sample_code_add(1, 2), 3)
    }
}