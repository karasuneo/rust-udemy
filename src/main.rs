// 定数
// letは関数の中でしか使えない
const MAX: u32 = 100_000;

// 関数
fn say_hello() {
    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("Hello, world!");

    // 変数を束縛する
    let a: i32 = 1;

    // 再代入可能な変数を定義するときはmutをつける
    let mut b = 2;
    b = 3;

    let c = 4;
    // シャドーイング
    // 同じ名前の変数を再定義することができる
    let c = c + 1;

    // 定数
    const MAX_POINTS: u32 = 100_000;

    // 数値型
    // i8, i16, i32, i64, i128, isize
    // u8, u16, u32, u64, u128, usize
    // f32, f64
    let x: i32 = 1;
    let y: f64 = 1.0;

    // 論理型
    // bool
    let t = true;

    // タプル
    // 複数の異なる型をまとめる
    // タプルの要素にアクセスするには、.の後にインデックスを指定する
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // ユニット型
    // ()
    // 何も返さない関数の戻り値として使われる
    let z = ();

    // 配列
    // 同じ型の要素を並べたもの
    // 配列の要素にアクセスするには、[]の中にインデックスを指定する
    let a = [1, 2, 3, 4, 5];
    let a1 = [0; 1000]; // 1000個の要素を0で初期化
    let first = a[0];

    // スライス
    // 配列の一部を参照する
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // 2, 3

    // ベクター
    // サイズが可変の配列
    let mut v = vec![1, 2, 3, 4, 5];
    let v1 = vec![0; 1000]; // 1000個の要素を0で初期化
    v.push(6);

    // Option型
    // 値が存在しない場合を表現する
    // 素材する場合はSome、存在しない場合はNone
    let pop = v.pop();
    print!("{:?}", pop);

    let v = vec![1, 2, 3, 4, 5];
    let z = v.get(1000);
    println!("{:?}", z);

    // 文字型
    let c1 = 'a';
    let c2 = 'あ';

    // 文字列型
    // 文字列リテラルは不変
    let s1 = "Hello, world!";
    // 文字列型は可変
    let s2 = String::from("Hello, world!");

    // 文字列の連結
    // 順序は逆にしたらエラーになる
    println!("{}", s2 + ", gold!");

    // フォーマット文字列
    // let s = format!("{}{}", s1, s2);

    say_hello();
    println!("{}", add(1, 2));

    // 式と文
}
