# Linear Regression Model
## 📌 Overview 
This project showcases how to create an instance of the simple linear regression model from a Rust program that uses the `burn` library. This instance is then utilized to forecast the output of the function `y = 2x + 1` by training with synthetic data. The project uses the `burn` library release 0.16.0 and the specified dependencies within the `Cargo.toml` file.

## Table of Contents
1. [📌 Overview](#-overview)  
2. [🛠️ Setup](#️-setup)  
3. [📈 Approach](#-approach)
4. [📤 Program Execution Results](#-program-execution-results)
5. [📚Resources Used](#-resources-used)
6. [📖 Learning Reflections](#-learning-reflections)

## 🛠️ Setup
To set up the project, do the following:
## Prerequisites
Ensure you have Rust installed. If you do not have Rust installed, you can install Rust using `rustup`.

## Checking Rust Installation

First, check if Rust is already installed on your system:
```sh
rustc --version
```
If Rust is installed, you will get the version of Rust installed.
    
**Rust installed**:

If Rust has not been installed, you can install it through `rustup`, which is the official suggested installation method of Rust.

1.  **Open your terminal or command prompt.**
2.  **Run the following command:**
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
3.  **Verify the installation using:**
 ```sh
 rustc --version
```
## Cloning the Repository

To clone this repository, run the following commands in terminal:

   ```sh
   git clone https://github.com/demifarquhar01/linear_regression.git
   cd linear_regression
   ```

## Running the Code
To run the code, use the following command:
```sh
cargo run
``
This will run the linear regression model, train it on simulated data, and print out the results, including the loss values, final model parameters, and a plot of the true and predicted data points.
  ```

## 📈 Approach
1. **Define the Model**:
  - Use `LinearRegression` struct to define the model with `weight` and `bias`.
  - Define methods for creating a new instance, making predictions (`forward`), and calculating the loss (`loss`).

2. **Generate Synthetic Data**:
 - Generated data points based on the function `y = 2x + 1 + noise`, where noise is some random number to make the data more real.
   
3. **Training Loop**:
  - Used gradient descent to update model parameters for 1000 epochs.
  - Printed loss every 100 epochs to observe the training progress.

4. **Evaluation**:
  - Validated the model against new unseen test data.
  - Plotted both the actual and predicted data points using the `textplots` crate.
  
  ## 📤 Program Execution Results
  **Sample Output:**
  
 ![image_alt](https://github.com/demifarquhar01/linear_regression/blob/98278736b0189882c564552e87713fc475e682e2/output1.png)
 ![image_alt](https://github.com/demifarquhar01/linear_regression/blob/98278736b0189882c564552e87713fc475e682e2/output2.png)

  
  ## 📚 Resources Used
  **Rust Documentation**:
  - [The Rust Programming Language](https://doc.rust-lang.org/book/)
  - [rand crate documentation](https://docs.rs/rand/latest/rand/)
  - [burn crate documentation](https://docs.rs/burn/latest/burn/)
  - [burn-ndarray crate documentation](https://docs.rs/burn-ndarray/latest/burn_ndarray/)
  - [textplots crate documentation](https://docs.rs/textplots/latest/textplots/)

**Git Assistance**:
- [Git](https://www.jetbrains.com/help/rust/using-git-integration.html)

**AI Assistance**:
  - AI tools was particularly helpful in correcting warnings.
  - Helped in rectifying mistakes which I was geeting.
  - Explained some code snippets that I didn't undestand and functions.
    
**Documentation**:
  - Immersed a great deal in Rust and crate documentation to explain how to use the libraries and features which had been employed.
  - The guide provided good examples and explanations, which were crucial in using the model.
  - Guided how to use Git integration in RustRover effectively

## 📖 Learning Reflections

**Challenges Faced**

**Outdated Functions**:
 - Encountered issues with an existing function that did not support the current version.
 - Learned about the importance of keeping dependencies up-to-date and searching for deprecation warnings.

**Limited Resources**:
 - Faced challenges due to a shortage of resources and tutorials on machine learning in Rust.
 - Resolved this by referring to general Rust documentation and experimenting with the code.

Overall, this project was a valuable learning experience in developing a machine learning model in Rust, working with functions, and using different crates for machine learning and data visualization.
