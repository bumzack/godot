use rand::distributions::Uniform;
use rand::prelude::*;

use crate::micrograd_rs_v3::{ValueRefV3, EPS2};

pub struct Neuron {
    weights: Vec<ValueRefV3>,
    bias: ValueRefV3,
    non_lin: bool,
}

impl Neuron {
    pub fn new(nin: usize, non_lin: bool, initializer: &mut dyn Initializer) -> Neuron {
        let mut weights = vec![];
        let init_values = initializer.get_values(nin);
        for i in 0..nin {
            weights.push(ValueRefV3::new_value(init_values[i], format!("weight {}", i)));
        }
        let bias = ValueRefV3::new_value(0.0, "bias".to_string());
        Neuron { weights, bias, non_lin }
    }

    pub fn new_weights_bias(weights: Vec<f64>, bias: f64, non_lin: bool) -> Neuron {
        let weights = weights
            .iter()
            .map(|w| ValueRefV3::new_value(*w, "w".to_string()))
            .collect();
        let bias = ValueRefV3::new_value(bias, "b".to_string());
        Neuron { weights, bias, non_lin }
    }

    pub fn forward(&self, xinp: &[ValueRefV3]) -> ValueRefV3 {
        assert!(xinp.len() == self.weights.len(), "input size does not match layer size");

        let x_w: Vec<ValueRefV3> = xinp
            .iter()
            .zip(self.weights.iter())
            .into_iter()
            .map(|(x, w)| x * w)
            .collect();

        let mut sum = ValueRefV3::new_value(0.0, "sum".to_string());
        for v in x_w {
            sum += v;
        }
        let out = &sum + &self.bias;
        if self.non_lin {
            out.relu()
        } else {
            out
        }
    }

    pub fn parameters(&self) -> Vec<ValueRefV3> {
        let mut params = vec![];
        self.weights.iter().for_each(|w| params.push(w.clone()));
        params.push(self.bias.clone());
        params
    }
}

pub trait Layer {
    fn forward(&self, xinp: &[ValueRefV3]) -> Vec<ValueRefV3>;
    fn parameters(&self) -> Vec<ValueRefV3>;
    fn name(&self) -> &String;
}

pub struct FC {
    neurons: Vec<Neuron>,
    name: String,
    non_lin: bool,
}

impl FC {
    pub fn new(nin: usize, nout: usize, non_lin: bool, name: String, initializer: &mut dyn Initializer) -> FC {
        let mut neurons = vec![];
        for _i in 0..nout {
            neurons.push(Neuron::new(nin, non_lin, initializer));
        }
        FC { neurons, name, non_lin }
    }
}

impl Layer for FC {
    fn forward(&self, xinp: &[ValueRefV3]) -> Vec<ValueRefV3> {
        let mut out = vec![];
        for n in &self.neurons {
            out.push(n.forward(xinp))
        }
        out
    }

    fn parameters(&self) -> Vec<ValueRefV3> {
        let mut params = vec![];
        self.neurons.iter().for_each(|n| params.append(&mut n.parameters()));
        params
    }

    fn name(&self) -> &String {
        &self.name
    }
}

pub struct Network {
    layers: Vec<Box<dyn Layer>>,
    loss: Box<dyn Loss>,
    optimizer: Box<dyn Optimizer>,
}

impl Network {
    pub fn new() -> Box<Network> {
        //TODO fix total_epochs = 0 mess
        let optimizer = Box::new(SGD::new(0.9, 0.0));
        let loss = Box::new(MaxMarginLoss::default());
        Box::new(Network {
            layers: vec![],
            loss,
            optimizer,
        })
    }

    pub fn optimizer(&mut self, optimizer: Box<dyn Optimizer>) {
        self.optimizer = optimizer;
    }

    pub fn loss(&mut self, loss: Box<dyn Loss>) {
        self.loss = loss;
    }

    // pub fn new_fully_connected(nin: usize, mut nouts: Vec<usize>) -> Box<Network> {
    //     let mut sizes = vec![];
    //     sizes.push(nin);
    //     sizes.append(&mut nouts);
    //
    //     let mut network = Network::new();
    //     for i in 0..sizes.len() - 1 {
    //         // println!("new layer nin {} -> nout {}", sizes[i], sizes[i + 1]);
    //         network.add_layer(Box::new(FC::new(sizes[i], sizes[i + 1])));
    //     }
    //     network
    // }

    pub fn add_layer(&mut self, l: Box<dyn Layer>) {
        self.layers.push(l);
    }

    fn forward_internal(&self, xinp: &[f64]) -> Vec<ValueRefV3> {
        let mut x: Vec<ValueRefV3> = xinp
            .iter()
            .map(|x| ValueRefV3::new_value(*x, "x".to_string()))
            .collect();
        for (_idx, l) in self.layers.iter().enumerate() {
            // println!("forward layer idx {}   name {}", _idx, l.name());
            // println!("input x.len {}    x[0] Ï{:?},   x1 {:?}", x.len(), x[0].get_data(), x[1].get_data());

            //x.iter().for_each(|i| println!(" input {}", i.get_data()));
            x = l.forward(&x);
            //x.iter().for_each(|i| println!("output {}", i.get_data()));
        }
        x
    }

    pub fn parameters(&self) -> Vec<ValueRefV3> {
        let mut params = vec![];
        self.layers.iter().for_each(|l| params.append(&mut l.parameters()));
        params
    }

    pub fn forward(&self, xs: &[Vec<f64>]) -> Vec<ValueRefV3> {
        let mut y_pred: Vec<ValueRefV3> = vec![];
        for x in xs.iter() {
            let y = self.forward_internal(x);
            let y = y.get(0).unwrap().clone();
            // println!("y data {:?}  grad {:?}", y.get_data(), y.get_grad());
            y_pred.push(y);
        }
        y_pred
    }

    pub fn print_params(&self) {
        println!("beforeupdate");
        for p in self.parameters() {
            println!(
                "parameter '{}': data {}, grad {}",
                p.borrow().label(),
                p.get_data(),
                p.get_grad()
            );
        }
    }

    pub fn reset_grades(&self) {
        self.parameters().iter_mut().for_each(|p| p.set_grad(0.0));
    }

    pub fn update(&self, epoch: usize) {
        self.optimizer.update(self.parameters(), epoch);
    }

    pub fn calc_loss(&self, y_ground_truth: &[f64], y_pred: &[ValueRefV3], parameters: Vec<ValueRefV3>) -> ValueRefV3 {
        self.loss.calc_loss(y_ground_truth, y_pred, parameters)
    }
}

pub trait Loss {
    fn calc_loss(&self, y_ground_truth: &[f64], y_pred: &[ValueRefV3], parameters: Vec<ValueRefV3>) -> ValueRefV3;
}

#[derive(Default)]
pub struct MSELoss {}

impl Loss for MSELoss {
    fn calc_loss(&self, y_ground_truth: &[f64], y_pred: &[ValueRefV3], _parameters: Vec<ValueRefV3>) -> ValueRefV3 {
        let loss_vec: Vec<ValueRefV3> = y_pred
            .iter()
            .zip(y_ground_truth.iter())
            .into_iter()
            .map(|(ypred, ygr)| (ypred - *ygr).pow(2.0))
            .collect();
        //loss_vec.iter().for_each(|y| println!("loss_vec = {}", y.get_data()));
        let mut loss = ValueRefV3::new_value(0.0, "loss".to_string());
        for l in loss_vec.iter() {
            // println!("loss {} += l {} ", loss, l.get_data());
            loss = &loss + l;
        }
        loss
    }
}

#[derive(Default)]
pub struct MaxMarginLoss {}

impl Loss for MaxMarginLoss {
    fn calc_loss(&self, y_ground_truth: &[f64], y_pred: &[ValueRefV3], parameters: Vec<ValueRefV3>) -> ValueRefV3 {
        let loss_vec: Vec<ValueRefV3> = y_pred
            .iter()
            .zip(y_ground_truth.iter())
            .into_iter()
            .map(|(ypred, ygr)| (1.0_f64 - &(*ygr * ypred)).relu())
            .collect();
        let mut loss = ValueRefV3::new_value(0.0, "loss".to_string());
        for l in loss_vec.iter() {
            loss = &loss + l;
        }

        let data_loss = &loss / (loss_vec.len() as f64);
        let alpha = 0.0001_f64;
        // let sum_parameters = parameters.iter().map(|p| p * p).collect();
        let mut reg_loss = ValueRefV3::new_value(0.0, "reg_loss".to_string());
        parameters.iter().for_each(|p| reg_loss = &reg_loss + &(p * p));
        reg_loss = &reg_loss * alpha;
        let total_loss = &reg_loss + &data_loss;
        println!(
            "reg_loss {},   data_loss {},   total_loss {}     loss {}",
            reg_loss.get_data(),
            data_loss.get_data(),
            total_loss.get_data(),
            loss.get_data()
        );
        // accuracy

        total_loss
    }
}

pub trait Optimizer {
    fn update(&self, parameters: Vec<ValueRefV3>, epoch: usize);
}

pub struct SGD {
    learning_rate: f64,
    totol_epochs: f64,
}

impl Optimizer for SGD {
    fn update(&self, mut parameters: Vec<ValueRefV3>, epoch: usize) {
        let lr = 1.0 - self.learning_rate * epoch as f64 / self.totol_epochs as f64;
        for p in parameters.iter_mut() {
            let x = p.get_data();
            let grad = p.get_grad();
            p.set_data(x - (lr * grad));
        }
        println!(
            "epoch: {}/{}, learning_rate {}, actual learning_rate {}",
            epoch, self.totol_epochs, self.learning_rate, lr
        );
    }
}

impl SGD {
    pub fn new(learning_rate: f64, totol_epochs: f64) -> SGD {
        SGD {
            learning_rate,
            totol_epochs,
        }
    }
}

pub fn print_predictions(y_pred: Vec<ValueRefV3>, y_expected: &[f64]) {
    y_pred.iter().enumerate().for_each(|(idx, y)| {
        let res = (y.get_data() - y_expected[idx]).abs() < EPS2;
        println!(
            "y_pred[{}] = {}    expected {}.   pred ok? {}",
            idx,
            y.get_data(),
            y_expected[idx],
            res
        );
    });
}

pub trait Initializer {
    fn get_values(&mut self, cnt: usize) -> Vec<f64>;
}

pub struct RandomUniformInitializer {
    normal: Uniform<f64>,
    rng: StdRng,
}

impl Initializer for RandomUniformInitializer {
    fn get_values(&mut self, cnt: usize) -> Vec<f64> {
        let mut res = vec![];
        for _i in 0..cnt {
            let y: f64 = self.normal.sample(&mut self.rng);
            res.push(y);
        }
        res
    }
}

impl RandomUniformInitializer {
    pub fn new() -> RandomUniformInitializer {
        let rng = StdRng::seed_from_u64(1337);
        let normal = Uniform::from(-1.0..1.0);
        RandomUniformInitializer { normal, rng }
    }
}

pub struct PythonNumPyRandomValuesInitializer {
    values: [f64; 304],
    idx: usize,
}

impl Initializer for PythonNumPyRandomValuesInitializer {
    fn get_values(&mut self, cnt: usize) -> Vec<f64> {
        let mut res = vec![];
        for i in self.idx..self.idx + cnt {
            res.push(self.values[i]);
        }
        self.idx += cnt;
        res
    }
}

impl PythonNumPyRandomValuesInitializer {
    pub fn new() -> Self {
        let values = [
            0.23550571390294128,
            0.06653114721000164,
            -0.26830328150124894,
            0.1715747078045431,
            -0.6686254326224383,
            0.6487474938152629,
            -0.23259038277158273,
            0.5792256498313748,
            0.8434530197925192,
            -0.3847332240409951,
            0.9844941451716409,
            -0.5901079958448365,
            0.31255526637777775,
            0.8246106857787521,
            -0.7814232047574572,
            0.6408752595662697,
            -0.20252189189007108,
            -0.8693137391598071,
            0.39841666323128555,
            -0.3037961142013801,
            -0.19282493884310759,
            0.6032250931493106,
            0.6001302646227185,
            0.32749776568749045,
            0.6650130652363544,
            0.1889136153241595,
            -0.07813264062433589,
            0.9151267732861252,
            0.5914405264235476,
            -0.3725442040076463,
            0.3810827422406471,
            0.8301999957053683,
            -0.08568482691922008,
            -0.4702876239420326,
            -0.598037011209763,
            -0.8653994554527067,
            0.05088685407468296,
            0.23734644010332318,
            0.15459549089529045,
            -0.9122391928398941,
            -0.18505999501786086,
            0.30584552737905213,
            0.23949109098065002,
            0.35119774963171047,
            0.26999576683073867,
            -0.6059558972032326,
            -0.4301483303818887,
            -0.09534359352124744,
            0.833061635489087,
            0.5964776511293395,
            -0.37143418174288434,
            0.5908148577342738,
            0.22158648570764017,
            -0.1356625769718156,
            0.5808552090645627,
            0.09921848842252134,
            0.5519936528601597,
            0.11082037875863104,
            0.2915133730664663,
            0.6968596263439943,
            -0.572699001261544,
            0.94892201097003,
            0.05815161059370322,
            0.05689619757216291,
            0.5506426045691593,
            -0.8991315551643992,
            -0.01068087363780501,
            0.47299771880745967,
            -0.08962899486130538,
            0.797578856715021,
            0.6099780726775426,
            -0.024825326467644793,
            0.5043619819611675,
            0.45774158735550596,
            -0.29478212096243595,
            0.11675968465796172,
            0.1379511601427985,
            -0.48542469349832285,
            -0.8664235814101062,
            -0.7390189923668276,
            -0.8822004511411428,
            -0.6597694707506181,
            0.6399602752689382,
            -0.6162690156778836,
            0.9053331545059524,
            0.667051974729419,
            0.7551658608563221,
            0.10907067581950436,
            -0.14588865117400673,
            0.2127856122995495,
            0.7622713432716846,
            0.8620382404752289,
            -0.1401108907535058,
            0.48216393547230973,
            -0.6888711593157701,
            0.2678404966193193,
            -0.3053994271093132,
            -0.8631814836201597,
            -0.29515687142070823,
            0.35372633701181444,
            0.20192101990676137,
            0.43475517949093345,
            -0.6169565150718037,
            -0.03186709594911474,
            0.22634427889578657,
            0.10564268012149869,
            -0.6805473384045992,
            0.422794461121468,
            0.6853554447554182,
            -0.21409905516555439,
            -0.6109356015626146,
            0.5254595422399804,
            0.24979744746643195,
            -0.16494497754636983,
            -0.6818144661499881,
            -0.06157981422579417,
            0.3953098897513252,
            -0.3566554480884392,
            -0.9395269671087605,
            0.19989246416270823,
            0.28261231537882425,
            -0.3861199056619302,
            0.8859519356381196,
            0.09224139623540206,
            0.5616028368830122,
            0.7479929232402773,
            -0.5498104256800536,
            -0.38944426340050686,
            -0.11986910432370723,
            -0.2418861692296186,
            0.27309902578900536,
            -0.7118613477995166,
            0.640699986750376,
            0.5251887402876205,
            -0.5265767665889542,
            0.6262237833195563,
            -0.8283374538902439,
            0.38807184998509303,
            -0.315003423604574,
            0.6825221766793921,
            -0.44958052796535997,
            0.054321569495217936,
            0.18838831645682874,
            -0.22248475258825984,
            -0.7209187740512764,
            0.7176790825016579,
            0.008555182533857453,
            -0.24243019229834561,
            0.27898488110769337,
            -0.7726321568042522,
            -0.5139485701725583,
            -0.8954946921521039,
            0.581615741803986,
            -0.5750613904646755,
            0.06993657839881884,
            0.8578625660652908,
            0.15993906511777078,
            -0.7940725880755064,
            0.7128617267763828,
            0.9005136363586974,
            0.01164370432983386,
            0.18191594886177542,
            0.28846645419632666,
            0.28220903218440285,
            -0.007467712517625236,
            -0.9683122463720533,
            -0.703733854503761,
            0.9555727255393986,
            0.8304099868721302,
            0.29860725600901694,
            0.23053684069095115,
            0.8609716364376814,
            0.470379750754194,
            -0.958287981521013,
            0.5814512996793573,
            -0.6753502452813329,
            0.5036443505111738,
            0.2955698675260916,
            0.4217229281756927,
            0.5984472102024547,
            -0.07808249126985456,
            0.6103717000192679,
            0.34645800749824374,
            0.504683663142057,
            -0.9498847321986592,
            -0.14743838678191312,
            0.5844330583547752,
            -0.7950857611747761,
            -0.6601994025531952,
            0.43550433241342956,
            0.8151878759155218,
            0.2604257711713296,
            0.7177690445444254,
            -0.686154027517816,
            -0.6063064618924185,
            -0.843421963461304,
            0.10008562568600432,
            0.8240605653030353,
            0.15495085113716178,
            -0.5089384775906294,
            0.8286765053073863,
            -0.8822610314096722,
            -0.5451509553109077,
            0.5761953058198175,
            -0.3434024177268147,
            0.10319781991345178,
            -0.05383238577004734,
            -0.6116507489401757,
            -0.8524536182338882,
            0.6964865423661555,
            0.32268922233815234,
            -0.2781907279339124,
            0.16059637633929102,
            -0.9446155428863412,
            0.8742237211553019,
            0.3582002209547388,
            0.9042806512794279,
            -0.8816062876600146,
            0.10041983326299175,
            -0.7698683314750423,
            0.30407601555374275,
            -0.20349872174164796,
            -0.4433144849231998,
            0.12433118993925452,
            -0.4258729196203048,
            0.3790360826044181,
            -0.9859455101873194,
            0.7028423162448694,
            -0.40122067203805645,
            -0.25683960260938843,
            0.5346953520807405,
            -0.35517369191511716,
            0.5121727526610462,
            -0.8868545578539118,
            0.518934991832354,
            -0.8928025540682154,
            0.5236713643981046,
            0.6018056040412896,
            0.24634309741440386,
            -0.20561868737419142,
            -0.652542799532154,
            -0.0065261577446391605,
            0.3493423738090866,
            -0.6324860574376863,
            -0.8530409123740017,
            -0.6218486564139833,
            0.9327230982583281,
            0.2793857831208002,
            0.5689184786100774,
            -0.6840675708965678,
            -0.5558656769249497,
            0.20592862129017364,
            -0.8391389406223104,
            -0.5529892816922855,
            -0.6278982991453468,
            -0.9592572536299122,
            0.9196221821038293,
            0.045865737597233114,
            0.5127293960073278,
            -0.7914104355296121,
            0.848793948186239,
            -0.3571964013350297,
            -0.8965914398912116,
            0.4191281777225171,
            -0.01884218503850832,
            0.6545963733751365,
            -0.3484979864252389,
            0.554377859246435,
            0.1689761071111946,
            -0.3388547761206535,
            0.397274795414263,
            -0.7930174038445066,
            0.077052593637436,
            0.3936052761946094,
            -0.8761639684113867,
            0.37877479983298445,
            -0.20339223773668702,
            -0.9231467276681604,
            -0.2020186036807059,
            0.9605940825345125,
            -0.9182348746309268,
            0.22922046437756505,
            -0.13194342373337498,
            -0.08703882869490953,
            0.984078978320559,
            0.19426273589837106,
            0.2294204003823488,
            0.12301724420660465,
            0.9128783824023976,
            -0.820982404658368,
            0.9648285595338895,
            0.3470665940198512,
            0.5436156893249604,
            0.49097996014038525,
            -0.9353940167321961,
            -0.707696853463387,
            -0.543868634071563,
            0.24162175370353833,
            -0.6723901907230767,
            -0.5973053326809556,
            0.6457663814022516,
            -0.2271549182489696,
            -0.3223491002609964,
            -0.2532524374373504,
        ];
        PythonNumPyRandomValuesInitializer { values, idx: 0 }
    }
}

#[cfg(test)]
mod tests {
    use crate::micrograd_rs_engine_v3::{
        Layer, Loss, MaxMarginLoss, Network, Neuron, PythonNumPyRandomValuesInitializer, FC,
    };
    use crate::micrograd_rs_v2::assert_two_float;
    use crate::micrograd_rs_v3::ValueRefV3;

    #[test]
    pub fn max_margin_loss() {
        let loss = MaxMarginLoss::default();

        let yb = vec![4.7, 5.9, 11.5];
        let scores = vec![
            ValueRefV3::new_value(-2.4, "a".to_string()),
            ValueRefV3::new_value(-4.6, "b".to_string()),
            ValueRefV3::new_value(9.2, "c".to_string()),
        ];

        let parameters = vec![
            ValueRefV3::new_value(1.3, "p1".to_string()),
            ValueRefV3::new_value(2.3, "p2".to_string()),
            ValueRefV3::new_value(4.5, "p3".to_string()),
        ];

        // python  reg_loss 0.002723  data_loss 13.473333333333333   total_loss     13.476056333333332

        let l = loss.calc_loss(&yb, &scores, parameters);

        let l_expected = 13.476056333333332;

        println!("l expected  {},   actual {}", l_expected, l.get_data());

        assert_two_float(l_expected, l.get_data());
    }

    #[test]
    pub fn max_margin_loss2() {
        let loss = MaxMarginLoss::default();

        let yb = vec![-4.7, -5.9, -11.5];
        let scores = vec![
            ValueRefV3::new_value(-2.4, "a".to_string()),
            ValueRefV3::new_value(-4.6, "b".to_string()),
            ValueRefV3::new_value(9.2, "c".to_string()),
        ];

        let parameters = vec![
            ValueRefV3::new_value(1.3, "p1".to_string()),
            ValueRefV3::new_value(2.3, "p2".to_string()),
            ValueRefV3::new_value(4.5, "p3".to_string()),
        ];

        let l = loss.calc_loss(&yb, &scores, parameters);

        let l_expected = 35.602723;

        println!("l expected  {},   actual {}", l_expected, l.get_data());

        assert_two_float(l_expected, l.get_data());
    }

    #[test]
    pub fn max_margin_loss3() {
        let loss = MaxMarginLoss::default();

        let yb = vec![-4.7, -5.9, -11.5];
        let scores = vec![
            ValueRefV3::new_value(2.4, "a".to_string()),
            ValueRefV3::new_value(4.6, "b".to_string()),
            ValueRefV3::new_value(-9.2, "c".to_string()),
        ];

        let parameters = vec![
            ValueRefV3::new_value(1.3, "p1".to_string()),
            ValueRefV3::new_value(2.3, "p2".to_string()),
            ValueRefV3::new_value(4.5, "p3".to_string()),
        ];

        let l = loss.calc_loss(&yb, &scores, parameters);
        let l_expected = 13.476056333333332;

        println!("l expected  {},   actual {}", l_expected, l.get_data());

        assert_two_float(l_expected, l.get_data());
    }

    #[test]
    pub fn test_neuron() {
        let mut initializer = PythonNumPyRandomValuesInitializer::new();
        let n = Neuron::new(2, true, &mut initializer);

        let x_inp = [2.0, 3.0];

        let mut x = vec![];

        for i in x_inp {
            let tmp = ValueRefV3::new_value(i, "w".to_string());
            x.push(tmp);
        }

        let mut y = n.forward(&x);

        y.backward();

        let y_expected = 0.6706048694358875;
        let w1_grad_expected = 2.0;
        let w2_grad_expected = 3.0;
        let w1_expected = 0.23550571390294128;
        let w2_expected = 0.06653114721000164;

        println!("y expected  {},   actual {}", y_expected, y.get_data());
        println!(
            "w1 data expected  {},   actual {}",
            n.parameters()[0].get_data(),
            w1_expected
        );
        println!(
            "w2 data expected  {},   actual {}",
            n.parameters()[1].get_data(),
            w2_expected
        );
        println!(
            "w1 grad expected  {},   actual {}",
            n.parameters()[0].get_grad(),
            w1_grad_expected
        );
        println!(
            "w2 grad expected  {},   actual {}",
            n.parameters()[1].get_grad(),
            w2_grad_expected
        );

        assert_two_float(n.parameters()[0].get_data(), w1_expected);
        assert_two_float(n.parameters()[1].get_data(), w2_expected);
        assert_two_float(n.parameters()[0].get_grad(), w1_grad_expected);
        assert_two_float(n.parameters()[1].get_grad(), w2_grad_expected);

        assert_two_float(y_expected, y.get_data());
    }

    #[test]
    pub fn test_layer() {
        let mut initializer = PythonNumPyRandomValuesInitializer::new();
        let l = FC::new(2, 1, true, "testlayer".to_string(), &mut initializer);

        let x_inp = [2.0, 3.0];

        let mut x = vec![];

        for i in x_inp {
            let tmp = ValueRefV3::new_value(i, "w".to_string());
            x.push(tmp);
        }

        let mut y = l.forward(&x)[0].clone();
        y.backward();

        let y_expected = 0.6706048694358875;
        let w1_grad_expected = 2.0;
        let w2_grad_expected = 3.0;
        let w1_expected = 0.23550571390294128;
        let w2_expected = 0.06653114721000164;

        println!("y expected  {},   actual {}", y_expected, y.get_data());
        println!(
            "w1 data expected  {},   actual {}",
            l.parameters()[0].get_data(),
            w1_expected
        );
        println!(
            "w2 data expected  {},   actual {}",
            l.parameters()[1].get_data(),
            w2_expected
        );
        println!(
            "w1 grad expected  {},   actual {}",
            l.parameters()[0].get_grad(),
            w1_grad_expected
        );
        println!(
            "w2 grad expected  {},   actual {}",
            l.parameters()[1].get_grad(),
            w2_grad_expected
        );

        assert_two_float(l.parameters()[0].get_data(), w1_expected);
        assert_two_float(l.parameters()[1].get_data(), w2_expected);
        assert_two_float(l.parameters()[0].get_grad(), w1_grad_expected);
        assert_two_float(l.parameters()[1].get_grad(), w2_grad_expected);

        assert_two_float(y_expected, y.get_data());
    }

    #[test]
    pub fn test_network() {
        let mut initializer = PythonNumPyRandomValuesInitializer::new();
        let l = FC::new(2, 1, true, "testlayer".to_string(), &mut initializer);
        let mut network = Network::new();
        network.add_layer(Box::new(l));

        let x_inp = vec![2.0, 3.0];

        // let mut x = vec![];
        //
        // for i in x_inp {
        //     let tmp = ValueRefV3::new_value(i, "w".to_string());
        //     x.push(tmp);
        // }

        let x_inp = vec![x_inp];
        let mut y = network.forward(&x_inp)[0].clone();

        y.backward();

        let y_expected = 0.6706048694358875;
        let w1_grad_expected = 2.0;
        let w2_grad_expected = 3.0;
        let w1_expected = 0.23550571390294128;
        let w2_expected = 0.06653114721000164;

        println!("y expected  {},   actual {}", y_expected, y.get_data());
        println!(
            "w1 data expected  {},   actual {}",
            network.parameters()[0].get_data(),
            w1_expected
        );
        println!(
            "w2 data expected  {},   actual {}",
            network.parameters()[1].get_data(),
            w2_expected
        );
        println!(
            "w1 grad expected  {},   actual {}",
            network.parameters()[0].get_grad(),
            w1_grad_expected
        );
        println!(
            "w2 grad expected  {},   actual {}",
            network.parameters()[1].get_grad(),
            w2_grad_expected
        );

        assert_two_float(network.parameters()[0].get_data(), w1_expected);
        assert_two_float(network.parameters()[1].get_data(), w2_expected);
        assert_two_float(network.parameters()[0].get_grad(), w1_grad_expected);
        assert_two_float(network.parameters()[1].get_grad(), w2_grad_expected);

        assert_two_float(y_expected, y.get_data());
    }

    #[test]
    pub fn test_bla() {
        let ygr = &1.2_f64;
        let y_pred = &ValueRefV3::new_value(0.5, "y_pred".to_string());
        let res1 = (1.0_f64 + &(-*ygr * y_pred)).relu();
        let res2 = (1.0_f64 - &(*ygr * y_pred)).relu();
        let a = &(-*ygr * y_pred);
        let b = -&(*ygr * y_pred);

        let c = 1.0_f64 + &(-*ygr * y_pred);
        let d = 1.0_f64 - &(*ygr * y_pred);

        let res_expected = 0.4;

        println!("a = {}  ", a);
        println!("b = {}  ", b);
        println!("c = {}  ", c);
        println!("d = {}  ", d);

        println!("res expected {}   res1 actuale {}", res_expected, res1.get_data());
        println!("res expected {}   res2 actuale {}", res_expected, res2.get_data());
        assert_two_float(res_expected, res1.get_data());
        assert_two_float(res_expected, res2.get_data());
    }
}
