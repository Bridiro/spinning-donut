use rand::Rng;
use std::f64::consts::PI;
use std::thread::sleep;
use std::time::Duration;
use term_size;

struct SpinningDonut {
    a: f64,
    b: f64,
    k1: f64,
    k2: f64,
    r1: f64,
    r2: f64,
}

fn main() {
    let (width, height) = if let Some((w, h)) = term_size::dimensions() {
        (w as usize, h as usize)
    } else {
        (80, 24)
    };

    let aspect_ratio_correction = width as f64 / height as f64 / 4.0;

    let mut donut = SpinningDonut {
        a: 0.0,
        b: 0.0,
        k1: 40.0,
        k2: 5.0,
        r1: 1.2,
        r2: 2.0,
    };

    let mut rng = rand::thread_rng();

    loop {
        let mut zbuffer = vec![0.0; width * height];
        let mut color_output = vec!["\x1b[0m ".to_string(); width * height];

        for theta in (0..360).step_by(5) {
            let theta = theta as f64 * PI / 180.0;
            for phi in (0..360).step_by(3) {
                let phi = phi as f64 * PI / 180.0;

                let cos_a = donut.a.cos();
                let sin_a = donut.a.sin();
                let cos_b = donut.b.cos();
                let sin_b = donut.b.sin();
                let cos_theta = theta.cos();
                let sin_theta = theta.sin();
                let cos_phi = phi.cos();
                let sin_phi = phi.sin();

                let circle_x = donut.r2 + donut.r1 * cos_theta;
                let circle_y = donut.r1 * sin_theta;

                let x = circle_x * (cos_b * cos_phi + sin_a * sin_b * sin_phi)
                    - circle_y * cos_a * sin_b;
                let y = circle_x * (sin_b * cos_phi - sin_a * cos_b * sin_phi)
                    + circle_y * cos_a * cos_b;
                let y = y * aspect_ratio_correction;
                let z = donut.k2 + cos_a * circle_x * sin_phi + circle_y * sin_a;
                let ooz = 1.0 / z;

                let xp = (width as f64 / 2.0 + donut.k1 * ooz * x) as usize;
                let yp = (height as f64 / 2.0 - donut.k1 * ooz * y) as usize;

                let luminance_index = (((cos_phi * cos_theta * sin_b
                    - cos_a * cos_theta * sin_phi
                    - sin_a * sin_theta
                    + cos_b * (cos_a * sin_theta - cos_theta * sin_a * sin_phi))
                    * 10.0)
                    + 10.0) as usize;

                let chars = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];
                let ch = chars
                    .get(luminance_index.max(0).min(chars.len() - 1))
                    .unwrap_or(&' ');

                let idx = xp + yp * width;

                let (color, priority) = if theta < PI {
                    if rng.gen_range(0..100) < 5 {
                        let sprinkle_colors = [
                            "\x1b[38;5;226m",
                            "\x1b[38;5;202m",
                            "\x1b[38;5;51m",
                            "\x1b[38;5;129m",
                        ];
                        let sprinkle_color =
                            sprinkle_colors[rng.gen_range(0..sprinkle_colors.len())];
                        (sprinkle_color, 2)
                    } else {
                        ("\x1b[38;5;218m", 1)
                    }
                } else {
                    ("\x1b[38;5;94m", 0)
                };

                if idx < width * height && (ooz > zbuffer[idx] || priority > 0) {
                    zbuffer[idx] = ooz;
                    color_output[idx] = format!("{}{}", color, ch);
                }
            }
        }

        print!("\x1B[H");
        for line in color_output.chunks(width) {
            println!("{}", line.join(""));
        }

        donut.a += 0.04;
        donut.b += 0.02;

        sleep(Duration::from_millis(30));
    }
}
