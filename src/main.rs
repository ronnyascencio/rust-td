mod dos;
mod tres;
mod uno;

fn main() {
    uno::say_hello();
    let _ = dos::create_folders("rust_folder");
    let list_items = vec!["test", "01", "03"];
    let _list: () = tres::read_list(&list_items);
    println!("{}", uno::shot_name("test", "lighting", 3));
}



