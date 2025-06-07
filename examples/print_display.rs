use std::fmt; // `fmt`のインポート


// 比較のため、フィールドに名前をつけられる様な構造体を定義しましょう。
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}


impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display: {} + {}i", self.real, self.imag)
    }
}

fn main(){
    let point = Complex { real: 3.3, imag: 7.2 };

    println!("Display: {}", point);
    println!("Debug: {:?}", point);

}