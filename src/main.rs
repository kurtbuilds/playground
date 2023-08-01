use anyhow::anyhow;

fn foo() -> anyhow::Result<i64> {
    return Err(anyhow!("foo"));
}

fn main() {
    let r = foo();
    let Err(r) = r else {
        panic!("Expected Err");
    };
    let s = r.backtrace().to_string();
    println!("{}", s);
}