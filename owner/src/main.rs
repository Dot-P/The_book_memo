fn main() {
    {
        let s: &str = "hello";
        println!("{}", s);
    }
    // println!("{}", s); // error: undeclared variable `s`

    {
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{}", s);
    }
    // OSにメモリを返している
}
