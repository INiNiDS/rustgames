use rustgames::core::app;

fn main() {

    println!("Запускаем движок...");

    if let Err(e) = app::run("Hello World", 2560.0, 1440.0) {
        eprintln!("Ошибка при запуске: {}", e);
    }
}