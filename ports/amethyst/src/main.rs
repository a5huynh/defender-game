extern crate amethyst;
use amethyst::core::transform::TransformBundle;
use amethyst::prelude::*;
use amethyst::renderer::{
    DisplayConfig,
    DrawFlat,
    DrawFlat2D,
    Pipeline,
    PosTex,
    RenderBundle,
    Stage,
};
use amethyst::utils::application_root_dir;

mod defender;
use crate::defender::Defender;

fn main() -> amethyst::Result<()> {
    // Start the amethyst logger with a default config.
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir();

    let path = format!(
        "{}/resources/display_config.ron",
        app_root
    );

    let config = DisplayConfig::load(&path);

    // Setup the rendering pipeline
    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                .with_pass(DrawFlat2D::new())
                .with_pass(DrawFlat::<PosTex>::new())
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
        )?
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::new("./", Defender, game_data)?;
    game.run();

    Ok(())
}
