use std::io;
use std::io::stdout;
use std::io::Write;
use std::thread;
use std::time::Duration;
use terminal_size::{terminal_size, Height, Width};

const PAPER_MAX: usize = 50000;

struct TerminalEnv {
    width: u16,
    height: u16,
    sqr: u16,
    coefic: f32,
}
impl Copy for TerminalEnv {}
impl Clone for TerminalEnv {
    fn clone(&self) -> TerminalEnv {
        *self
    }
}

fn start_env_manager_thread() {
    let env_manager_thread = thread::spawn(move || {
        fn get_terminal_size() -> (u16, u16) {
            if let Some((Width(w), Height(h))) = terminal_size() {
                println!("Your terminal is {} cols wide and {} lines tall", w, h);
                //TODO add if state if > 16...
                return (w, h);
            } else {
                println!("Unable to get terminal size");
                return (0, 0);
            }
        }
        loop {
            thread::sleep(Duration::from_secs(10000))
        }
    });
}

fn start_event_listener_thread() {
    let event_listener_thread = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(10000))
    });
}

fn update_env_var() -> TerminalEnv {
    let a: u16 = 24;
    let b: u16 = 80;
    return TerminalEnv {
        width: a,
        height: b,
        sqr: a * b,
        coefic: (a as f32 / b as f32),
    };
}

fn render_math_patern<F>(
    env: TerminalEnv,
    time: f64,
    paper: &mut [char; PAPER_MAX],
    patern: F,
    from_x: i16,
    to_x: i16,
) -> &mut [char; PAPER_MAX]
where
    F: Fn(u16) -> u16,
{
    for i in 0..(env.width as usize) {
        paper[i] = '_';
        //FIXME
    }
    return paper;
}

fn printer(env: TerminalEnv, paper: &mut [char; PAPER_MAX]) -> () {
    for i in 0..(env.sqr as usize) {
        print!("{}", paper[i]);
    }
    //FIXME
}

fn main() {
    start_env_manager_thread();
    start_event_listener_thread();

    println!("hello)");

    let mut time_sec: f64 = 0.0;
    loop {
        let env: TerminalEnv = update_env_var();
        time_sec += 0.500;
        thread::sleep(Duration::from_millis(500));
        // /////////////////////////////////////////////////

        let mut paper: [char; PAPER_MAX] = ['0'; PAPER_MAX];

        printer(
            env,
            render_math_patern(env, time_sec, &mut paper, |x| x, -4, 4),
        );
        //FIXME
        stdout().flush().ok();
    }
}
