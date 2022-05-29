fn clo(){
    let a = "Hello";
    let b = "Tyr";

    let c = |msg: &str| {
        println!{"{},{}:{}",a,b,msg}
    };

    c("How are you ?");
}