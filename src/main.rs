use rand::Rng;
use textplots::{Chart, Plot, Shape};

//Linear regression model
#[derive(Debug)]
struct LinearRegression {
    weight: f64,
    bias: f64,
}

impl LinearRegression {
    //Creates a new Linear Regression model instance
    fn new() -> Self {
        let mut rng = rand::rng();
        Self {
            weight: rng.random(),
            bias: rng.random(),
        }
    }
//Performs a forward pass to predict the output
    fn forward(&self, x: f64) -> f64 {
        self.weight * x + self.bias
    }
//Calculates the Mean Squared Error (MSE) loss
    fn loss(&self, y_pred: f64, y_true: f64) -> f64 {
        (y_pred - y_true).powi(2)
    }
}

fn main() {
    // Generate synthetic data
    let mut rng = rand::rng();
    let data: Vec<(f64, f64)> = (0..100)
        .map(|_| {
            let x: f64 = rng.random_range(0.0..10.0);
            let noise: f64 = rng.random_range(-1.0..1.0);
            let y = 2.0 * x + 1.0 + noise;
            (x, y)
        })
        .collect();

    // Initialize the model
    let mut model = LinearRegression::new();

    // Training loop
    let learning_rate = 0.001;
    let epochs = 1000;
    for epoch in 0..epochs {
        let mut total_loss = 0.0;
        for (x, y) in &data {
            let y_pred = model.forward(*x);
            let loss = model.loss(y_pred, *y);
            total_loss += loss;

            model.weight -= learning_rate * 2.0 * (y_pred - y) * x;
            model.bias -= learning_rate * 2.0 * (y_pred - y);
        }
        // Print the loss every 100 epochs
        if epoch % 100 == 0 {
            println!("Epoch {}: Loss = {}", epoch, total_loss / data.len() as f64);
        }
    }
    // Evaluate the model
    let test_data: Vec<(f64, f64)> = (0..20)
        .map(|_| {
            let x: f64 = rng.random_range(0.0..10.0);
            let noise: f64 = rng.random_range(-1.0..1.0);
            let y = 2.0 * x + 1.0 + noise;
            (x, y)
        })
        .collect();

    let test_x: Vec<f64> = test_data.iter().map(|(x, _)| *x).collect();
    let test_y: Vec<f64> = test_data.iter().map(|(_, y)| *y).collect();
    let predictions: Vec<f64> = test_x.iter().map(|x| model.forward(*x)).collect();

    // Plot the results
    let true_points: Vec<(f32, f32)> = test_x
        .iter()
        .zip(test_y.iter())
        .map(|(x, y)| (*x as f32, *y as f32))
        .collect();

    let pred_points: Vec<(f32, f32)> = test_x
        .iter()
        .zip(predictions.iter())
        .map(|(x, y)| (*x as f32, *y as f32))
        .collect();

    println!("True data (x, y): {:?}", true_points);
    println!("Predictions (x, y): {:?}", pred_points);

    Chart::new(180, 60, 0.0, 10.0)
        .lineplot(&Shape::Points(&true_points))
        .lineplot(&Shape::Points(&pred_points))
        .display();

    println!("Model: {:?}", model);
}