fn main() {
    let mut v = vec![1, 2, 3];
    //Avoid using raw pointers
    v[0] = 10;
    println!( "{:?}", v);
}