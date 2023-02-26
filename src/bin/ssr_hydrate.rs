use blog_romira_dev::prelude::App;

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Error));
    yew::Renderer::<App>::new().hydrate();
}
