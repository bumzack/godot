struct MyStruct {
    a: f64,
}

fn main() {
    let a: Vec<MyStruct> = Vec::new();

    a.push(MyStruct { a: 2.0 });
    a.push(MyStruct { a: 3.0 });

    a.sort()
}
