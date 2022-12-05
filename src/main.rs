use crate::cli::FakerApp;

mod cli;

// as cli
fn main() -> std::io::Result<()> {
    FakerApp::new().run()
}
