use vizia::prelude::*;

struct RightAlignment;

impl RightAlignment {
    fn new<F, U: Into<Units>>(ctx: &mut Context, right: impl Res<U>, content: F) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        Self {}
            .build(ctx, |ctx| content(ctx))
            .role(Role::GenericContainer)
            // .left(Stretch(1.0))
            .right(right)
    }
}

impl View for RightAlignment {
    fn element(&self) -> Option<&'static str> {
        Some("right_alignment")
    }
}

pub fn run() {
    Application::new(|ctx| {
        VStack::new(ctx, |ctx| {
            HStack::new(ctx, |ctx| {
                Button::new(ctx, |_| {}, |ctx| Label::new(ctx, "RGBcraft"));
                RightAlignment::new(ctx, Pixels(0.0), |ctx| {
                    HStack::new(ctx, |ctx| {
                        Label::new(ctx, "Enn3DevPlayer");
                        Button::new(ctx, |_| {}, |ctx| Label::new(ctx, "Skin"));
                    })
                    .col_between(Pixels(10.0));
                })
                .width(Percentage(100.0));
            })
            .width(Percentage(100.0))
            .child_space(Pixels(10.0))
            .col_between(Pixels(20.0));
        })
        .width(Percentage(100.0));
    })
    .title("RGBcraft")
    .run();
}
