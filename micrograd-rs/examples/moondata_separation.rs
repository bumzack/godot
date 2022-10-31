use std::f64::consts::PI;
use std::process::exit;
use std::time::Instant;

use plotters::prelude::*;
use rand::Rng;

use micrograd_rs::prelude::calc_loss_mse;
use micrograd_rs::prelude::{draw_graph, print_predictions, ValueRefV2, MLP};

fn main() {
    // config
    let cnt_samples = 100;
    let epochs = 500;

    // x1, y1 red points
    // x1, y1 blue points

    let (x1, y1, x2, y2) = create_moondata(cnt_samples, 0.1);

    let y_red = vec![1.0; cnt_samples];
    let y_blue = vec![-1_f64; cnt_samples];

    let model = MLP::new(2, vec![16, 16, 1]); // 2-layer neural network

    model.print_params();

    let mut x: Vec<Vec<f64>> = vec![];
    let mut y: Vec<f64> = vec![];

    // take care, that the x1,y1  resp x2,y2 pairs match the resulting color in y
    for i in 0..x1.len() {
        let inp = vec![x1[i], y1[i]];
        x.push(inp);
        y.push(y_red[i]);
        let inp = vec![x2[i], y2[i]];
        x.push(inp);
        y.push(y_blue[i]);
    }
    println!("number of parameters {}", model.parameters().len());
    println!("x.len()  {}", x.len());
    println!("y.len()  {}", y.len());

    let start = Instant::now();
    // desired targets
    let mut y_pred = vec![];

    for i in 0..epochs {
        // forward pass
        y_pred = model.forward(&x);

        // calculate loss
        let (mut loss, accuracy) = calc_loss_max_margin(&y, &y_pred, model.parameters());

        // print_params(&mlp);
        // backward pass consists of 2 steps
        model.reset_grades();
        loss.backward();

        // update parameters
        model.update2(i, epochs);

        // keep track of loss improvement
        println!(
            "iteration {}   loss {}, accuracy {:.4}%",
            i + 1,
            loss.get_data(),
            accuracy * 100.0
        );
    }
    println!("y_pred.len()  {}", y_pred.len());

    print_predictions(y_pred, &y);
    //  model. print_params();

    let duration = start.elapsed();
    println!("training took {:?}", duration);

    plot_result(&model, &x, &y);
    println!("DONE");
}

fn plot_result(model: &MLP, x: &Vec<Vec<f64>>, y: &Vec<f64>) -> Result<(), Box<dyn std::error::Error>> {
    // dont know how to draw a contour plot using plotters -> so just evaluate the model for a lot of (x,y) coordinate pairs and draw a lot og point
    let x_min = 0.0_f64;
    let x_max = 2.0_f64;

    let y_min = 0.0_f64;
    let y_max = 2.0_f64;

    let steps = 75;

    let delta_x = (x_max - x_min) / steps as f64;
    let delta_y = (x_max - x_min) / steps as f64;

    let mut res = vec![];

    for ix in 0..steps {
        for iy in 0..steps {
            let x = x_min + ix as f64 * delta_x;
            let y = y_min + iy as f64 * delta_y;
            let input = vec![vec![x, y]];
            let prediction = model.forward(&input);

            prediction.iter().for_each(|p| {
                let color = if p.get_data() > 0.0 { RED } else { BLUE };
                res.push((x, y, color))
            });
        }
    }

    let filename = "xprediction_fake_contour.png".to_string();

    let root = BitMapBackend::new(&filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..2f32, 0.0f32..2.0f32)?;

    chart.configure_mesh().draw()?;

    let delta = 0.005;

    chart.draw_series(res.iter().map(|(xx, yy, color)| {
        Rectangle::new(
            [
                (*xx as f32 - delta, *yy as f32 - delta),
                (*xx as f32 + delta, *yy as f32 + delta),
            ],
            color,
        )
    }))?;

    let mut points = vec![];
    for i in 0..x.len() {
        let p = &x[i];
        let x_coord = p[0];
        let y_coord = p[1];
        let color = if y[i] > 0.0 { RED } else { BLUE };

        points.push((x_coord, y_coord, color));
    }

    chart.draw_series(
        points
            .iter()
            .map(|(xx, yy, color)| Circle::new((*xx as f32, *yy as f32), 3, color.filled())),
    )?;

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    println!("wrote file {:?}", &filename);

    Ok(())
}

pub fn calc_loss_max_margin(
    y_ground_truth: &Vec<f64>,
    y_pred: &Vec<ValueRefV2>,
    parameters: Vec<ValueRefV2>,
) -> (ValueRefV2, f64) {
    let loss_vec: Vec<ValueRefV2> = y_pred
        .iter()
        .zip(y_ground_truth.iter())
        .into_iter()
        .map(|(ypred, ygr)| (1.0_f64 + &(&(-ypred) * *ygr)).relu())
        .collect();
    //loss_vec.iter().for_each(|y| println!("loss_vec = {}", y.get_data()));
    let mut loss = ValueRefV2::new_value(0.0, "loss".to_string());
    for l in loss_vec.iter() {
        // println!("loss {} += l {} ", loss, l.get_data());
        loss = &loss + l;
    }
    let data_loss = loss / loss_vec.len() as f64;
    let alpha = 0.0001_f64;
    // let sum_parameters = parameters.iter().map(|p| p * p).collect();
    let mut reg_loss = ValueRefV2::new_value(0.0, "reg_loss".to_string());
    parameters.iter().for_each(|p| reg_loss = &reg_loss + p);
    reg_loss = &reg_loss * alpha;
    let total_loss = reg_loss + data_loss;

    // accuracy
    let accuracies: Vec<bool> = y_pred
        .iter()
        .zip(y_ground_truth.iter())
        .map(|(y_pred_i, y_ground_truth_i)| (y_pred_i.get_data() > 0.0_f64) == (*y_ground_truth_i > 0.0_f64))
        .collect();
    let success = accuracies.iter().filter(|&a| *a).count() as f64;
    let accuracy = success / accuracies.len() as f64;

    (total_loss, accuracy)
}

fn create_moondata(n: usize, noise: f64) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let center_x = 0.75;
    let center_y = 0.75;

    let (x1, y1) = create_half_cirle(center_x, center_y, n, true, noise);
    let (x2, y2) = create_half_cirle(center_x + 0.5, center_y + 0.25, n, false, noise);

    draw_chart("moondata.png".to_string(), &x1, &y1, RED, &x2, &y2, BLUE);
    (x1, y1, x2, y2)
}

fn create_half_cirle(center_x: f64, center_y: f64, n: usize, upper_half: bool, noise: f64) -> (Vec<f64>, Vec<f64>) {
    let mut rnd = rand::thread_rng();
    let theta_delta = PI / n as f64;

    let (start_theta, _end_theta) = if upper_half { (0.0, PI) } else { (PI, 0.0) };

    let r = 0.5;
    let mut x = vec![];
    let mut y = vec![];
    let mut theta = start_theta;
    for _i in 0..n {
        let x_noise = rnd.gen::<f64>() * 2.0 * noise - noise;
        let y_noise = rnd.gen::<f64>() * 2.0 * noise - noise;
        let x_tmp = center_x + r * theta.cos() + x_noise;
        let y_tmp = center_y + r * theta.sin() + y_noise;
        x.push(x_tmp);
        y.push(y_tmp);
        theta += theta_delta;
    }

    (x, y)
}

fn draw_chart(
    filename: String,
    x1: &Vec<f64>,
    y1: &Vec<f64>,
    color1: RGBColor,
    x2: &Vec<f64>,
    y2: &Vec<f64>,
    color2: RGBColor,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(&filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..2f32, 0.0f32..2.0f32)?;

    chart.configure_mesh().draw()?;

    let x1_y1 = x1.iter().zip(y1.iter()).map(|(xx, yy)| (*xx as f32, *yy as f32));
    let x2_y2 = x2.iter().zip(y2.iter()).map(|(xx, yy)| (*xx as f32, *yy as f32));

    chart.draw_series(x1_y1.map(|(xx, yy)| Circle::new((xx, yy), 1, color1.filled())))?;
    chart.draw_series(x2_y2.map(|(xx, yy)| Circle::new((xx, yy), 1, color2.filled())))?;

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
