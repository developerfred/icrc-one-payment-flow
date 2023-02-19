# Payment App on the Internet Computer

This is a simple payment app built on the Internet Computer, allowing users to send payments to other accounts on the platform. This app is built using TypeScript and the Tailwind CSS framework for styling.

## Getting Started

### Prerequisites
To run this project, you'll need the following installed on your system:

1. Node.js
2. Internet Computer Development Environment (DFX)

### Installing
1. Clone this repository to your local machine.
2. Install the required dependencies by running npm install in the project directory.

### Running the app
1. Start the local development environment by running dfx start in a terminal.
2. In another terminal, create a new canister by running dfx canister create --all.
3. Deploy the canister by running dfx deploy.
4. Copy the ID of the deployed canister and paste it in the sendPayment function in the index.ts file.
5. Start the web app by running npm start in the project directory.
6. Open your web browser and navigate to http://localhost:8000 to access the payment app.

### How to use
1. Enter the payment amount in ICPT in the Amount (ICPT) input field.
2. Enter the payee's account ID in the Payee input field.
3. Click on the Send Payment button to send the payment.
4. If the payment is successful, the result will be logged in the browser console.

### Code Structure

The project consists of two main files:

1. index.html: Contains the HTML structure of the payment app.
2. index.ts: Contains the TypeScript code for interacting with the canister.

## Built With

1. TypeScript - A superset of JavaScript that adds static typing to the language.
2. Tailwind CSS - A utility-first CSS framework for building custom user interfaces.

Author
[codingsh](mailto:codingsh@pm.me)

## Acknowledgments
DFINITY - For providing the Internet Computer platform and development tools.