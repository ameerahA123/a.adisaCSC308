fn main() {
    //let name = String::from("George");
    
    //let greet = ||println!("Hello, {}", name);
    //greet();
    //println!("Still usable:{}", name);
    //let owned = move||println!("Hello again, {}!", name);
    //owned();
    //println!("{}", name); //won't compile: moved

    let handle = thread::spawn(||{
        for i in 1..5{
            println!("hi number {} 
}
