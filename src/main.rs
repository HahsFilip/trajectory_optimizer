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
    //let acc = f(planet_masses, planet_location, [state.x, state.y],g);
    // let mut k1 = f(planet_masses.clone(), planet_location.clone(), [state.x, state.y],g);
    // let mut k2 = f(planet_masses.clone(), planet_location.clone(), [state.x+ dt*k1[0]/2.0, state.y + dt*k1[1]/2.0],g);
    // let mut k3 = f(planet_masses.clone(), planet_location.clone(), [state.x+ dt*k2[0]/2.0, state.y + dt*k2[1]/2.0],g);
    // let mut k4 = f(planet_masses.clone(), planet_location.clone(), [state.x+ dt*k3[0], state.y + dt*k3[1]],g);
    // let mut acc = [0.0;2];
    // acc = [(k1[0]+ 2.0*k2[0]+2.0*k3[0]+ k4[0])/6.0,(k1[1]+ 2.0*k2[1]+2.0*k3[1]+ k4[1])/6.0];
    // k1 = [state.vx, state.vy];
    // k2 =  [state.vx+ dt*k1[0]/2.0, state.vy + dt*k1[1]/2.0];
    // k3 =[state.vx+ dt*k2[0]/2.0, state.vy + dt*k2[1]/2.0];
    // k4 = [state.vx+ dt*k3[0], state.vy + dt*k3[1]];

    // let mut vel = [0.0; 2];
    
    // vel  = [(k1[0]+ 2.0*k2[0]+2.0*k3[0]+ k4[0])/6.0,(k1[1]+ 2.0*k2[1]+2.0*k3[1]+ k4[1])/6.0];
    let mut a = f(planet_masses.clone(), planet_location.clone(), [state.x, state.y],g);
    a[0] = dt*a[0]/2.0;
    a[1] = dt*a[1]/2.0;
    let beta = [dt*(state.vx+a[0]/2.0)/2.0, dt*(state.vy+a[1]/2.0)/2.0];
    let mut b = f(planet_masses.clone(), planet_location.clone(), [state.x+beta[0], state.y+beta[1]],g);
    b[0] = dt*b[0]/2.0;
    b[1] = dt*b[1]/2.0;
    let mut c = f(planet_masses.clone(), planet_location.clone(), [state.x+beta[0], state.y+beta[1]],g);
    c[0] = dt*c[0]/2.0;
    c[1] = dt*c[1]/2.0;
    let delta = [dt*(state.vx+c[0]),dt*(state.vy+c[1])];
    let mut d = f(planet_masses.clone(), planet_location.clone(), [state.x+delta[0], state.y+delta[1]],g);
    d[0] = dt*d[0]/2.0;
    d[1] = dt*d[1]/2.0;
    let k = [(a[0] + b[0] + c[0])/3.0,(a[1] + b[1] +c[1])/3.0];
    let j = [(a[0] + 2.0*b[0] + 2.0*c[0] + d[0])/3.0, (a[1] + 2.0*b[1] + 2.0*c[1] + d[1])/3.0];
    State {
        x: state.x + dt*(state.vx+k[0]),
        y: state.y + dt*(state.vy+k[1]),
        vx: state.vx + j[0],
        vy: state.vy + j[1],
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

        canvas.fill_rect(Rect::new((state.x+400.0) as i32,(state.y +300.0)as i32, 1, 1) );
        if i%1000 == 0{
            canvas.present();

        }

    }
    

    println!("Hello, world!");
    Ok(())
}
