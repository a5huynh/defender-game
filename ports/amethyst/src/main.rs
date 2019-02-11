use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
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
use crate::defender::config::DefenderConfig;

fn main() -> amethyst::Result<()> {
    // Start the amethyst logger with a default config.
    amethyst::start_logger(Default::default());

    let resource_root = format!("{}/resources", application_root_dir());
    let path = format!("{}/display_config.ron", resource_root);
    let binding_path = format!("{}/bindings.ron", resource_root);
    let config_path = format!("{}/config.ron", resource_root);

    let config = DisplayConfig::load(&path);
    let game_config = DefenderConfig::load(&config_path);

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(binding_path)?;

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
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(defender::systems::EnemySystem, "enemy_system", &[])
        .with(defender::systems::PlayerSystem, "player_system", &["input_system"])
        .with(defender::systems::MoveBulletSystem, "move_bullet_system", &[]);

    let mut game = Application::build("./", Defender)?
        .with_resource(game_config.bullet)
        .with_resource(game_config.enemy)
        .with_resource(game_config.game)
        .with_resource(game_config.player)
        .build(game_data)?;

    game.run();

    Ok(())
}
