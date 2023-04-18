use rand::Rng;
extern crate sdl2;
use colors_transform;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
// use colors_transform::Color;
// use colors_transform::{Hsl, Color};
use colors_transform::Color as OtherColor;
use std::{thread, time};
use std::fs::File;
use std::io::prelude::*;

fn calculate_accelerations(planet_masses: Vec<f32>, planet_location: Vec<[f32;2]>, ship_location:[f32;2], g: f32) -> [f32;2]{
    let mut result:[f32;2] = [0.0;2];
    for i in 0..planet_masses.len(){
        let r = (planet_location[i][0]-ship_location[0]).powf(2.0)+(planet_location[i][1]-ship_location[1]).powf(2.0);
        let acc = planet_masses[i]*g/r;
        let x = planet_location[i][0]-ship_location[0];
        let y = planet_location[i][1]-ship_location[1];
        let cos_pos = x/r.sqrt();
        let sin_pos = y/r.sqrt();
        result[0] += acc*cos_pos;
        result[1] += acc*sin_pos;
    }
    result
}


fn integrate<F: Fn( Vec<f32>, Vec<[f32;2]>,[f32;2], f32) ->[f32;2] >(
    f: F,
    state: State,
   
    dt: f32,
    planet_location: Vec<[f32;2]>,
    planet_masses: Vec<f32>,
    g:f32
) -> State {
    let acc = f(planet_masses, planet_location, [state.x, state.y],g);
    State {
        x: state.x + dt*state.vx,
        y: state.y + dt*state.vy,
        vx: state.vx + dt*acc[0],
        vy: state.vy + dt*acc[1],
    }
}

struct State {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

impl Copy for State { }

impl Clone for State {
    fn clone(&self) -> State {
        *self
    }
}

fn main() -> Result<(), String> {
    let planet_masses = vec![10.0;1];
    let planet_location = vec![[0.0, 0.0];1];
    let g_const = 1000.0;

    
    let n_time_steps = 100000;
    let dt = 0.001;
    let mut state = State{x:100.0, y: 0.0, vx:0.0, vy:5.0};
    let mut velocities = vec![[0.0,0.0];n_time_steps];
    let mut positions = vec![[0.0,0.0];n_time_steps];
    
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
    .window("rust-sdl2 demo: Video", 800, 600)
    .position_centered()
    .opengl()
    .build()
    .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.present();
    canvas.set_draw_color(Color::RGB(100, 100, 100));

    for i in 0..n_time_steps{
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        state = integrate(calculate_accelerations, state, dt, planet_location.clone(), planet_masses.clone(), g_const);
        println!("{} {} {}",(state.x*10.0), (state.y*10.0) as i32, state.x.powf(2.0)+state.y.powf(2.0));
        canvas.set_draw_color(Color::RGB(100, 100, 100));

        canvas.fill_rect(Rect::new((state.x+400.0) as i32,(state.y +300.0)as i32, 10, 10) );
        canvas.present();

    }
    

    println!("Hello, world!");
    Ok(())
}
