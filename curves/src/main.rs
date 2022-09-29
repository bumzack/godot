mod curve;

use crate::curve::{Curve, CurveCommon};
use plotters::prelude::*;
use raytracer_challenge_reference_impl::math::{Matrix, MatrixOps};
use raytracer_challenge_reference_impl::prelude::{Tuple, Tuple4D};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let trans = Matrix::new_identity_4x4();
    let trans_inv = Matrix::new_identity_4x4();
    let c = CurveCommon::new();
    let u_min = 0.0;
    let u_max = 1.0;
    let c = Curve::new(trans, trans_inv, c, false, 0.0, 1.0);
}

fn plotters_example() -> Result<(), Box<dyn std::error::Error>> {
    let root =
        BitMapBackend::new("/Users/bumzack/stoff/rust/godot/curves/src/plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
