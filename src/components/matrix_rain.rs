use gloo_timers::callback::Timeout;
use rand::prelude::*;
use web_sys::window;
use yew::prelude::*;
use yew_hooks::prelude::*;

const VALID_CHARS: &str = "abcdefghijklmnopqrstuvwxyz0123456789$+-*/=%'#&_(),.;:?!\\|{}<>[]^~";

const MIN_STREAM_SIZE: u32 = 10;
const MAX_STREAM_SIZE: u32 = 20;

const STREAM_MUTATION_RATE: f32 = 0.2;

const MIN_INTERVAL_DELAY: u32 = 70;
const MAX_INTERVAL_DELAY: u32 = 90;

const MIN_DELAY_BETWEEN_STREAMS: u32 = 0;
const MAX_DELAY_BETWEEN_STREAMS: u32 = 8000;

const FONT_SIZE: i32 = 35;

pub fn get_rand_char() -> char {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..VALID_CHARS.len());
    VALID_CHARS.chars().nth(idx).unwrap()
}

pub fn get_rand_in_range(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn get_random_stream() -> Vec<char> {
    let stream_size = get_rand_in_range(MIN_STREAM_SIZE, MAX_STREAM_SIZE);
    let mut stream = Vec::with_capacity(stream_size as usize);
    for _ in 0..stream_size {
        stream.push(get_rand_char());
    }
    stream
}

pub fn get_mutated_stream(stream: &Vec<char>) -> Vec<char> {
    let mut rng = rand::thread_rng();
    let mut stream = stream.clone();
    stream.remove(0);
    for i in 0..stream.len() {
        if rng.gen_range(0.0..1.0) < STREAM_MUTATION_RATE {
            stream[i] = get_rand_char();
        }
    }
    stream.push(get_rand_char());
    stream
}

#[function_component]
fn RainStream() -> Html {
    let stream = use_state(|| get_random_stream());
    let top_padding = use_state(|| stream.len() as i32 * -FONT_SIZE);
    let interval_delay = use_state(|| 0);

    {
        let interval_delay = interval_delay.clone();
        use_effect_with_deps(
            move |_| {
                let interval_delay = interval_delay.clone();
                Timeout::new(
                    get_rand_in_range(MIN_DELAY_BETWEEN_STREAMS, MAX_DELAY_BETWEEN_STREAMS),
                    move || {
                        interval_delay
                            .set(get_rand_in_range(MIN_INTERVAL_DELAY, MAX_INTERVAL_DELAY));
                    },
                )
                .forget();
            },
            (),
        );
    }

    {
        let interval_delay_clone = interval_delay.clone();

        let top_padding = top_padding.clone();
        let height = window()
            .unwrap()
            .inner_height()
            .unwrap()
            .as_f64()
            .unwrap()
            .clone();
        let stream = stream.clone();

        use_interval(
            move || {
                if *top_padding > height as i32 {
                    stream.set(get_random_stream());
                    top_padding.set((|| stream.len() as i32 * -FONT_SIZE)());
                    interval_delay.set(0);
                    let interval_delay = interval_delay.clone();
                    Timeout::new(
                        get_rand_in_range(MIN_DELAY_BETWEEN_STREAMS, MAX_DELAY_BETWEEN_STREAMS),
                        move || {
                            interval_delay
                                .set(get_rand_in_range(MIN_INTERVAL_DELAY, MAX_INTERVAL_DELAY));
                        },
                    )
                    .forget();
                } else {
                    top_padding.set(*top_padding + 44);
                }
                stream.set(get_mutated_stream(&stream));
            },
            *interval_delay_clone,
        );
    }

    html! {
       <div style={format!(
                    "font-family: matrixFont; margin-top: {}px; color: #20c20e; writing-mode: vertical-rl;
                    text-orientation: upright; font-size: {}px; white-space: nowrap;
                    user-select: none; text-shadow: 0px 0px 8px rgba(32, 194, 14, 0.8);",
                    *top_padding, FONT_SIZE
                )}>
            {
            stream.iter().enumerate().map(|(i, c)| {
                html!{<span key={i}
                style={format!(
                    "{} opacity: {}; {}; margin-top: -12;",
                    if i == stream.len() - 1 { "color: #fff;" } else { "" },
                    if i < 6 { 0.1 + i as f32 * 0.15} else {1.0},
                    if i == stream.len() - 1 { "textShadow: 0px 0px 20px rgba(255, 255, 255, 1);" } else { "" },
                )}>
                { c }</span>}
            }).collect::<Html>()
        }
       </div>
    }
}

#[function_component]
pub fn MatrixRain() -> Html {
    let width: i32 = window()
        .unwrap()
        .inner_height()
        .unwrap()
        .as_f64()
        .unwrap()
        .clone() as i32;

    let stream_count = (width / 17) as usize;
    let streams = vec![0; stream_count];
    html! {
        <div
        style="background: black; position: fixed; top: 0;
        left: 0; bottom: 0; right: 0; display: flex;">
        {streams.iter().map(|_| {
                html!{<RainStream />}
            }).collect::<Html>()
            }
        </div>

    }
}
