use druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use druid::widget::Label;

fn build_ui() -> impl Widget<()> {
    Label::new("Hello world")
}

fn main() -> Result<(), PlatformError> {
    let root = || {
        build_ui()
    };
    AppLauncher::with_window(
        WindowDesc::new(root)
    ).launch(())?;
    Ok(())
}
