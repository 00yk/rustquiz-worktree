mod quiz1;
mod quiz21;
mod quiz13;
fn main() {
    // let a = f();
    // println!("{:?}", a);
    let x = || { (return) || true; };
    println!("{:?}", x());
}
