#![feature(std_misc)]
#![feature(io)]
extern crate clock_ticks;

use clock_ticks as tick;
use std::cmp;
use std::old_io::timer::sleep;
use std::time::duration::Duration;

#[allow(dead_code)]
const BILLION: u64 = 1_000_000_000 as u64;

pub enum LoopState {
    Break,
    Continue,
}

#[allow(dead_code)]
pub struct EventLoop {
    ups: u32,
    fps: u32,
}

pub trait HasEventLoop {
    fn render(&mut self, dt: f32);
    fn update(&mut self, dt: f32);
    fn poll(&mut self) -> Option<LoopState>;
}

#[allow(dead_code)]
impl EventLoop {
    pub fn new(ups: u32, fps: u32) -> EventLoop {
        EventLoop {
            ups: ups,
            fps: fps,
        }
    }
    pub fn run<T: HasEventLoop>(&self, obj: &mut T) {
        let max_fps = (BILLION / self.fps as u64) as u64;
        let max_ups = (BILLION / self.ups as u64) as u64;

        let mut last_frame = 0.0 as u64;
        let mut last_update = 0.0 as u64;
        let start_time = tick::precise_time_ns();
        'main: loop {
            let current_time = tick::precise_time_ns() - start_time;
            let next_frame = last_frame + max_fps;
            let next_update = last_update + max_ups;
            let next_time = cmp::min(next_frame, next_update);
            if next_time > current_time {
                let diff = (next_time - current_time) as i64;
                sleep(Duration::nanoseconds(diff as i64));
            } else if next_time == next_frame {
                let diff = (current_time - last_frame) as i64;
                last_frame = current_time;
                let render_dt = (diff as f64 / BILLION as f64) as f32;
                obj.render(render_dt);
            } else {
                match obj.poll() {
                    Some(state) => match state {
                        LoopState::Break => break 'main,
                        LoopState::Continue => continue 'main,
                    },
                    None => {
                        last_update = current_time + max_ups as u64;
                        obj.update(1.0 / self.ups as f32);
                    }
                }
            }
        }
    }
}
