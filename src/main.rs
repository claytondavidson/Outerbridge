use amethyst::utils::application_root_dir;
use amethyst::{GameDataBuilder, Application};
use amethyst::core::TransformBundle;
use amethyst::renderer::{
    plugins::{RenderToWindow, RenderFlat2D},
    types::DefaultBackend,
    RenderingBundle
};
use amethyst::tiles::{RenderTiles2D};

mod states;
mod resources;
mod entities;
mod systems;
mod utils;

use crate::entities::Terrain;
use crate::systems::{MovementSystem};
use crate::systems::CameraFollowSystem;
use crate::systems::MovementBindingTypes;
use amethyst::input::{InputBundle};
use amethyst::core::frame_limiter::FrameRateLimitStrategy;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let resources = app_root.join("resources");
    let config = app_root.join("config");
    let display_config_path = config.join("display_config.ron");
    let input_config_path = config.join("input_config.ron");

    let input_bundle =
        InputBundle::<MovementBindingTypes>::new()
            .with_bindings_from_file(input_config_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(input_bundle)?
        .with(MovementSystem, "movement_system", &["input_system"])
        .with(CameraFollowSystem, "camera_follow_system", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_config_path)?.with_clear([0.0, 0.0, 0.0, 1.0]))
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<Terrain>::default())
        )?;

    let mut game = Application::build(resources, states::GameState)?.with_frame_limit(FrameRateLimitStrategy::Yield, 144).build(game_data)?;

    game.run();
    Ok(())
}
