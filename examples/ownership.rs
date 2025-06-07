fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // 所有権が s1 から s2 にムーブされる

    // println!("{}", s1); // これはエラー！s1 はもはや使えない

    println!("s2: {}", s2);

    let s3 = s2.clone(); // 所有権を複製（ディープコピー）
    println!("s2: {}, s3: {}", s2, s3);

    takes_ownership(s3); // s3 の所有権が関数にムーブされる
    // println!("{}", s3); // これもエラー！s3 は move された

    let x = 5;
    makes_copy(x); // i32 は Copy トレイトを持っているため使える

    println!("x is still usable: {}", x);
}

fn takes_ownership(s: String) {
    println!("Took ownership of: {}", s);
}

fn makes_copy(n: i32) {
    println!("Copied value: {}", n);
}
