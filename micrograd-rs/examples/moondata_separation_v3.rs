use std::f64::consts::PI;
use std::time::Instant;

use plotters::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::StdRng;
use rand::{thread_rng, Rng, SeedableRng};
use rand_distr::Normal;

use micrograd_rs::micrograd_rs_engine_v3::{MaxMarginLoss, Network, FC, SGD};

fn main() {
    let mut r = StdRng::seed_from_u64(1337);
    let normal = Uniform::from(-1.0..1.0);

    // config
    let use_python_data = true;

    let (epochs, network, x, y) = if use_python_data {
        let epochs = 300;
        let (x, y) = moondata_python();

        // network config
        let nin = 2;
        let n_hidden = 16;
        let nout = 1;

        let input_layer = FC::new(nin, n_hidden, &normal, &mut r);
        let hidden_layer1 = FC::new(n_hidden, n_hidden, &normal, &mut r);
        let output_layer = FC::new(n_hidden, nout, &normal, &mut r);

        let mut network = Network::new();
        network.add_layer(Box::new(input_layer));
        network.add_layer(Box::new(hidden_layer1));
        network.add_layer(Box::new(output_layer));

        let optimizer = SGD::new(0.9, epochs as f64);
        let loss = MaxMarginLoss::new();
        network.optimizer(Box::new(optimizer));
        network.loss(Box::new(loss));

        (epochs, network, x, y)
    } else {
        let cnt_samples = 50; // 50 per color -> 100 total like in the jupyter notebook
        let epochs = 400;
        let (x, y) = prepare_data(cnt_samples);

        // network config
        let nin = 2;
        let n_hidden = 16;
        let nout = 1;

        let input_layer = FC::new(nin, n_hidden, &normal, &mut r);
        let hidden_layer1 = FC::new(n_hidden, n_hidden, &normal, &mut r);
        let output_layer = FC::new(n_hidden, nout, &normal, &mut r);

        let mut network = Network::new();
        network.add_layer(Box::new(input_layer));
        network.add_layer(Box::new(hidden_layer1));
        network.add_layer(Box::new(output_layer));

        let optimizer = SGD::new(0.9, epochs as f64);
        let loss = MaxMarginLoss::new();

        network.optimizer(Box::new(optimizer));
        network.loss(Box::new(loss));

        (epochs, network, x, y)
    };

    println!("x.len {}", x.len());
    println!("y.len {}", y.len());

    println!("number of parameters {}", network.parameters().len());

    for i in 0..50 {
        println!("parameter {}", network.parameters()[i].get_data());
    }

    let y_pred = network.forward(&x);
    let loss = network.calc_loss(&y, &y_pred, network.parameters());

    println!("before training     loss {} ", loss.get_data(),);

    let start = Instant::now();
    let mut y_pred = vec![];

    for i in 0..epochs {
        // forward pass
        y_pred = network.forward(&x);

        // calculate loss
        let mut loss = network.calc_loss(&y, &y_pred, network.parameters());

        // print_params(&mlp);
        // backward pass consists of 2 steps
        network.reset_grades();
        loss.backward();

        if i == 0 {
            for i in 0..30 {
                println!(
                    "p data {} grad {}",
                    network.parameters()[i].get_data(),
                    network.parameters()[i].get_grad()
                );
            }
        }

        // update parameters
        network.update(i);

        let accuracies: Vec<bool> = y_pred
            .iter()
            .zip(y.iter())
            .map(|(y_pred_i, y_ground_truth_i)| (y_pred_i.get_data() > 0.0_f64) == (*y_ground_truth_i > 0.0_f64))
            .collect();
        let success = accuracies.iter().filter(|&a| *a).count() as f64;
        let accuracy = success / accuracies.len() as f64;

        // keep track of loss improvement
        println!(
            "iteration {}   loss {}, accuracy {:.4}%",
            i + 1,
            loss.get_data(),
            accuracy * 100.0
        );
    }
    println!("y_pred.len()  {}", y_pred.len());

    // print_predictions(y_pred, &y);
    //  model. print_params();

    let duration = start.elapsed();
    println!("training took {:?}", duration);

    plot_result(&network, &x, &y, "moondate_own_data_network".to_string());
    println!("DONE");
}

fn prepare_data(cnt_samples: usize) -> (Vec<Vec<f64>>, Vec<f64>) {
    // x1, y1 red points
    // x1, y1 blue points
    let (x1, y1, x2, y2) = create_moondata(cnt_samples, 0.1);

    let y_red = vec![1.0; cnt_samples];
    let y_blue = vec![-1_f64; cnt_samples];

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
    println!("x.len()  {}", x.len());
    println!("y.len()  {}", y.len());
    (x, y)
}

fn plot_result(
    network: &Network,
    x: &Vec<Vec<f64>>,
    y: &Vec<f64>,
    filename: String,
) -> Result<(), Box<dyn std::error::Error>> {
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
            let prediction = network.forward(&input);

            prediction.iter().for_each(|p| {
                let color = if p.get_data() > 0.0 { RED } else { BLUE };
                res.push((x, y, color))
            });
        }
    }

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

// data demo from one sklearn.moondata ouput
fn moondata_python() -> (Vec<Vec<f64>>, Vec<f64>) {
    let x_coord = vec![
        1.1221146075409685,
        -0.8188294130836691,
        1.613709660987544,
        -0.9230091836872232,
        0.14385146230962004,
        0.16447246572042107,
        1.338770617485968,
        0.8711486149043729,
        1.8312994627973058,
        0.4875712016409384,
        0.03746235110753285,
        -0.44391685255324553,
        -0.8122294944924991,
        1.6355231237135142,
        0.47353903666605085,
        0.753549316407056,
        0.2642128178390429,
        1.4275572560879635,
        -0.37235605731048926,
        -0.9613019674806484,
        0.7808584676599848,
        0.9166090294455649,
        1.0470380903106813,
        -0.060363054164925364,
        0.029189538040355685,
        -0.3957322548540574,
        -0.1046459096804331,
        1.8811000369217943,
        0.8274087792502122,
        1.2977711151467368,
        -0.6768928469782971,
        0.5295299531938408,
        -0.8438022913586616,
        0.2659847077207925,
        0.13740386164239876,
        -0.9104393600278938,
        1.3374003113508148,
        1.0225771946111462,
        1.0249013176024238,
        -0.7508958968579265,
        1.2028163168774058,
        -0.46910212171699023,
        0.7408366765236178,
        0.7869051168340989,
        -0.1300519139789317,
        0.8040230610618104,
        0.28330367002596785,
        -0.7110553964392495,
        0.3026245209248618,
        0.8079146318191945,
        -0.9416269141193683,
        1.1408148478377786,
        -0.15713752222511088,
        1.7150437047638079,
        0.37246575434044527,
        -0.7430957763302336,
        0.6814248911657479,
        0.8686124799437664,
        1.002295748910335,
        -1.0221939055140943,
        -0.6107625808608383,
        -0.7766830470454348,
        1.1053000144325091,
        -0.17890459717580573,
        0.4049830567528997,
        1.8145730036084153,
        -0.7912856268533403,
        1.9817314854963066,
        0.732427955867351,
        2.111416474903194,
        2.1620559786436147,
        0.8962454195875718,
        0.3734937590184516,
        0.5849645542492797,
        0.17632978792377418,
        0.10911564587188148,
        0.29753274826363046,
        0.0960465719727507,
        0.4746436366619598,
        1.1163471114571442,
        0.5536275879061199,
        0.18952809933886133,
        1.9456691368692929,
        -0.08462662114050497,
        1.1685800833655082,
        0.12446846728801658,
        2.0338992398932274,
        -0.02800053795270451,
        0.7409348125505573,
        0.7939470343137883,
        0.8687426245818287,
        1.6187443465007663,
        1.4298656424500784,
        1.9748043513307652,
        1.8535634717599803,
        1.749121635119567,
        -0.6856688883596882,
        1.752374345064964,
        0.18078955117509088,
        0.12108297264758441,
    ];

    let y_coord = vec![
        0.08147717341718697,
        0.058790063907949996,
        -0.12464590012238597,
        0.3652288989804421,
        0.04438004920995997,
        0.11738345677751341,
        -0.23800993320667369,
        -0.42271758719088287,
        -0.14104382791190478,
        0.6390928295021813,
        0.42358809046232426,
        0.8967393121026791,
        0.9120909237685235,
        -0.3499967597774375,
        0.9573425992910307,
        0.6237271428204225,
        -0.242419828423408,
        -0.3725103576299691,
        0.9566917098622661,
        0.32609011168610447,
        0.7974894021183523,
        -0.42763843805483504,
        -0.5444924698666057,
        0.11960908813305368,
        0.306838997007325,
        0.89654389483245,
        1.117883128977461,
        0.29920256798265166,
        0.3449771713089005,
        -0.36654315099838575,
        0.8555995736897581,
        0.947355940971469,
        0.604739822447823,
        0.8873219859622185,
        0.39785689354023546,
        -0.09709664085661444,
        -0.36741197399620207,
        -0.39752649343440544,
        -0.548639298268643,
        0.2532877224572353,
        0.08115381856609327,
        0.7807962193408648,
        0.4592325371321422,
        0.7662474609621819,
        1.119389404636236,
        -0.4231544735547401,
        -0.2194407109638437,
        0.7116388049480358,
        -0.09338908609846948,
        0.33653832558974994,
        0.16801857627931402,
        -0.46289718509868166,
        0.9321066421784753,
        -0.183620087722965,
        -0.12607841243567364,
        0.6989516672603877,
        0.685634239342246,
        -0.3727861752564913,
        0.21687478467387197,
        0.3975082133877554,
        0.8311713693476314,
        0.6436093225034623,
        0.21916281149260236,
        1.0695977352727732,
        0.8264783367858084,
        0.034525376226522034,
        0.20006880934562235,
        0.4607605346796224,
        -0.39965754330624315,
        0.18064070830876056,
        0.4942338188202992,
        0.46153606282567994,
        1.0449830116530658,
        -0.32125984073767117,
        0.19745571898583383,
        0.48155701068292406,
        0.9958363866186999,
        -0.04626986295341696,
        -0.10512647440393183,
        -0.41553437298946216,
        -0.42312582098348556,
        1.01835654559919,
        -0.0953034039958919,
        1.0726234174534877,
        -0.028481070091387548,
        1.0572503108231874,
        0.28472979916305263,
        0.17076763930165323,
        0.4141143831184791,
        0.5597258004647198,
        -0.5301471985675346,
        -0.3258452871084818,
        -0.47334210330960585,
        -0.17793162185350148,
        0.3422639581001494,
        0.028339024709346988,
        0.46535693570295134,
        0.16452051430367814,
        -0.0002954176191645175,
        1.0655522522437006,
    ];

    let y_int = vec![
        0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1,
        1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1,
        1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0,
    ];

    let mut x = vec![];
    let mut y = vec![];

    for i in 0..x_coord.len() {
        x.push(vec![x_coord[i], y_coord[i]]);
        y.push(y_int[i] as f64);
    }
    println!("xlen() {}", x.len());
    println!("ylen() {}", y.len());
    (x, y)
}
