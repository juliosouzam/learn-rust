const MAX_SPEED: i32 = 8000;

fn clamp_speed(speed: i32) -> i32 {
    if speed > MAX_SPEED {
        MAX_SPEED
    }

    speed
}

fn main() {
    //
}
