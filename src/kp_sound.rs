use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

const PI: f64 = std::f64::consts::PI;

fn create_noise(n: u32) -> Vec<(f64, f64)> {
    let mut noise = Vec::new();
    let phase: f64 = 3.0 * PI / 17.0;
    for i in 0..n {
        noise.push((i as f64, f64::sin(i as f64 * phase)));
    }
    noise
}

fn compute_samples(nb_samples: u32, noise: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    // https://crypto.stanford.edu/~blynn/sound/karplusstrong.html

    let mut samples: Vec<(f64, f64)> = Vec::new();
    let n = noise.len();
    for i in 0..nb_samples as usize {
        if i < n {
            samples.push((i as f64, noise[i].1));
        } else if i > n {
            samples.push((i as f64, (samples[i - n].1 + samples[i - n - 1].1) / 2.0));
        } else {
            samples.push((i as f64, 0.0));
        }
    }
    samples
}

fn main() {
    let v = create_noise(128);

    let base0: u32 = 2;
    let s = compute_samples(base0.pow(15), v);

    let s1: Plot = Plot::new(s).point_style(
        PointStyle::new()
            .marker(PointMarker::Cross) // setting the marker to be a square
            .colour("#DD3355"),
    );

    // The 'view' describes what set of data is drawn
    let base: f64 = 2.;
    let view = ContinuousView::new()
        .add(s1)
        .x_range(0., base.powf(15.))
        .y_range(-2., 2.)
        .x_label("Some varying variable")
        .y_label("The response of something");

    // A page with a single view is then saved to an SVG file
    Page::single(&view).save("scatter.svg").unwrap();
}
