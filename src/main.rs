use druid::{AppLauncher, WindowDesc, Widget, PlatformError, Color};
use druid::widget::{Label, Flex, Padding, Container};

fn build_ui() -> impl Widget<()> {
    Container::new(
        Padding::new(
            10.0,
            Flex::row()
                .with_flex_child(
                    Flex::column()
                        .with_flex_child(Label::new("top left"), 1.0)
                        .with_flex_child(Label::new("bottom left"), 1.0),
                    1.0)
                .with_flex_child(
                    Flex::column()
                        .with_flex_child(Label::new("top right"), 1.0)
                        .with_flex_child(Label::new("bottom right"), 1.0),
                    1.0),
        )
    ).background(Color::BLACK)
}

fn main() -> Result<(), PlatformError> {
    let root = || {
        build_ui()
    };
    let window = WindowDesc::new(root)
        .window_size((720., 600.))
        .with_min_size((620., 300.))
        .title("Quick Paste");
    AppLauncher::with_window(
        window
    ).launch(())?;
    Ok(())
}
