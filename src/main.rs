extern crate ron;
#[macro_use] extern crate serde;
extern crate sdl2;
extern crate gl;

mod config;
mod graphics;
//mod input;


fn main() {
    //Load Configuration from file
    let conf = config::Config::from_file(std::path::Path::new("res/config.ron")).unwrap();

    //Initialize SDL
    let sdl = sdl2::init().unwrap();
    
    //Initialize graphics
    let mut graphics_manager = graphics::GraphicsManager::new(&conf, &sdl);
    graphics_manager.load_shaders(
        std::path::Path::new("res/shaders/triangle.vert"),
        std::path::Path::new("res/shaders/triangle.frag")
    );
     
    //Initialize events
    let mut event_pump = sdl.event_pump().unwrap();


    //Main loop
    'main: loop {
        //Handle events
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        //Game Logic
        
        //Draw
        graphics_manager.render();
    }
}

