# simple-rust-calculator
Simple Calculator using Rust

This project involves creating a simple command-line calculator in Rust that performs basic arithmetic operations.

The calculator supports addition, subtraction, multiplication, and division of two floating-point numbers (f64).

The core functionalities are built using Rust enums and pattern matching to handle different arithmetic operations.

## Key Features
- Enum for Operations: The project defines an Operation enum with variants for Add, Subtract, Multiply, and Divide, each holding two f64 values.
- Pattern Matching: The calculate function uses pattern matching to determine which arithmetic operation to perform based on the Operation enum variant.
- User Input: The program prompts the user to input two numbers and an arithmetic operation.
- Input Parsing: User inputs are parsed into appropriate types (f64 for numbers and String for the operation).
- Dynamic Calculation: Based on the user's input, an Operation enum instance is created, and the calculate function is called to perform the operation.
- Result Display: The result of the calculation is printed to the console.

## Getting Started
This is an example of how you can set up this project locally. To get a local copy up and running, follow these steps.

### Installation
1. Clone the repo
   ```sh
   git clone https://github.com/ras-24/simple-rust-calculator.git
   ```
2. Go to lottery contract directory
   ```sh
   cd simple-rust-calculator
   ```
3. Install NPM packages
   ```sh
   npm install
   ```
4. On deploy.js file, please change to your seed phrase and infura endpoint
   ```sh
   const provider = new HDWalletProvider(
      'YOUR_METAMASK_SEED_PHRASE',
      // remember to change this to your own phrase!
      'YOUR_INFURA_SEPOLIA_URL_ENDPOINT'
      // remember to change this to your own endpoint!
   );
   ```
5. Deploy lottery contract
   ```sh
   node deploy.js
   ```
6. Copy **Contract Deployed Address** and **Contract ABI** to [lottery contract frontend](https://github.com/ras-24/lottery-react-contract/blob/main/src/lottery.js)

## License

Distributed under the MIT License. See `LICENSE` for more information.

