fn main() {
    for n in 0..10 {
        // println!("{n}: Hello, Rust!");
        if n == 5{
            println!("{n}: Hello, Rust");
        }
        let a = "hellow";
        let mut b = "hellow";
        b = "not";
        const C: &str = "sss";
        println!("{a},{b},{C}")
        

        // NOTE: 文字列中で変数を展開する記法は、古いバージョンのRustだと動かない場合があります
        // その場合は以下に書き換えてください。
        // println!("{}: Hello, Rust!", n);
    }
}
