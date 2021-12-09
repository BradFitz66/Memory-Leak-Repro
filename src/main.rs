use bevy::{app::AppExit, prelude::*, window::WindowResizeConstraints,diagnostic::{Diagnostic, DiagnosticId, Diagnostics, LogDiagnosticsPlugin,FrameTimeDiagnosticsPlugin},};
use bevy_pixels::prelude::*;

//Constants
const WIDTH: u32 = 200;
const HEIGHT: u32 = 200;
const PIXEL_SIZE:u32 = 4;
const WINDOW_WIDTH: u32 = WIDTH * PIXEL_SIZE;
const WINDOW_HEIGHT: u32 = HEIGHT * PIXEL_SIZE;




fn main() {
    App::build()
    .insert_resource(WindowDescriptor{
        title:"I am a window!".to_string(),
        width:(WINDOW_WIDTH as f32),
        height:(WINDOW_HEIGHT as f32),
        vsync:false,
        resizable:false,
        decorations:true,
        ..Default::default()
    })
    .insert_resource(PixelsOptions {
        width: WIDTH,
        height: HEIGHT,
    })
    .add_plugin(PixelsPlugin)
    .add_plugins(DefaultPlugins)
    .run();
    
}
