fn main() {
    let flag = 
        if Ok("release".to_owned()) == std::env::var("PROFILE") {
            "RELEASE"
        } else {
            "DEBUG"
        };

    println!("cargo:rustc-link-search=native=./Core/build/{}/.", flag);
}