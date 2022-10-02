extern crate sdl2;
mod vulkan;
mod utils;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use vulkan::vulkan_instance::VulkanApp;
use std::time::Duration;

const APP_VERSION: u32 = 
    ash::vk::make_api_version(0, 0 , 0, 1);

const VULKAN_API_VERSION: u32 = 
    ash::vk::make_api_version(0, 1, 3, 0);

pub fn main() {

    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .vulkan()
        .build()
        .unwrap();

    let vk_instance = VulkanApp::new(
        APP_VERSION,
        VULKAN_API_VERSION,
        &window.vulkan_instance_extensions().unwrap()
    );
    
    let mut canvas = window.into_canvas().build().unwrap();

    
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    vk_instance.destroy()
}