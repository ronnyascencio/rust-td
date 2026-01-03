mod dos;
mod uno;

fn main() {
    uno::say_hello();
    let _ = dos::create_folders("rust_folder");
    println!("{}", uno::shot_name("test", "lighting", 3))
}
