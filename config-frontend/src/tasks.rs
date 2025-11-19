use messaging::client_wrapper::ClientWrapper;
use messaging::socket::Client;

pub fn connect_to_backend() -> ClientWrapper {
    loop {
        match Client::new() {
            Ok(client) => return ClientWrapper::new(client),
            Err(_) => continue,
        }
    }
}

pub fn select_image_blocking() -> Option<String> {
    let file = rfd::FileDialog::new()
        .add_filter("image", &["jpg"])
        .set_directory("/")
        .pick_file();

    if let Some(image) = file
        && let Ok(path_buf) = image.canonicalize()
        && let Some(absolute_path) = path_buf.to_str()
    {
        return Some(String::from(absolute_path));
    }

    None
}
