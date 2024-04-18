use std::rc::Rc;

struct WindowConfig {
    title: String,
    width: u32,
    height: u32,
}

fn configure_window(config: &Rc<WindowConfig>) {
    println!("Configuring window:");
    println!("Title: {}", config.title);
    println!("Size: {}x{}", config.width, config.height);
}

fn resize_window(config: &Rc<WindowConfig>, new_width: u32, new_height: u32) {
    println!("Resizing window from {}x{} to {}x{}", config.width, config.height, new_width, new_height);
}

pub fn hello_rc1() {
    let window_config = Rc::new(WindowConfig {
        title: "My App Window".to_string(),
        width: 800,
        height: 600,
    });

    configure_window(&window_config);
    resize_window(&window_config, 1024, 768);
}
