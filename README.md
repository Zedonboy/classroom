# Classroom Project

A decentralized classroom management system built on the Internet Computer (ICP) platform using Rust and Vue.js.

## Prerequisites

Before you begin, ensure you have the following installed:

1. **Rust & Wasm Target**
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Add Wasm target
   rustup target add wasm32-unknown-unknown
   ```

2. **Internet Computer SDK**
   ```bash
   sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
   ```

3. **Candid Extractor**
   ```bash
   cargo install candid-extractor
   ```

4. **Node.js & npm** (version 16 or higher recommended)

## Quick Start

Follow these steps to get the project running locally:

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd classroom
   ```

2. **Start the local ICP replica**
   ```bash
   dfx start --background
   ```

3. **Prepare the environment**
   ```bash
   ./prep.sh
   ```

4. **Deploy the canisters**
   ```bash
   dfx deploy
   ```

5. **Generate Candid interfaces**
   ```bash
   dfx generate
   ```

6. **Install dependencies**
   ```bash
   # Root directory dependencies
   npm install

   # Frontend dependencies
   cd src/classroom_frontend
   npm install
   ```

7. **Start the development server**
   ```bash
   npm run dev
   ```

Your application will be available at:
- Backend: `http://localhost:4943?canisterId={asset_canister_id}`
- Frontend: `http://localhost:5173`

## Project Structure

```
classroom/
├── src/
│   ├── classroom_backend/    # Rust backend canister
│   │   └── src/
│   │       └── lib.rs
│   └── classroom_frontend/   # Vue.js frontend
│       ├── src/
│       └── package.json
├── dfx.json                  # DFX configuration
├── package.json
└── prep.sh                   # Environment preparation script
```

## Development Workflow

### Backend Changes

After making changes to the Rust backend:

1. Generate new Candid interfaces:
   ```bash
   dfx generate
   ```

2. Redeploy the canister:
   ```bash
   dfx deploy classroom_backend
   ```

### Frontend Changes

The development server will automatically reload when you make changes to the frontend code.

To build for production:
```bash
cd src/classroom_frontend
npm run build
```

## Deployment

To deploy to the IC mainnet:

1. Ensure you have ICP tokens for canister cycles
2. Configure your cycles wallet
3. Deploy using:
   ```bash
   dfx deploy --network ic
   ```

## Environment Variables

For frontend deployment without DFX, you'll need to handle the `DFX_NETWORK` environment variable. Options include:

- Setting `DFX_NETWORK=ic` for Webpack
- Configuring `dfx.json` with environment overrides
- Creating a custom `createActor` constructor

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## Troubleshooting

Common issues and solutions:

1. **Replica not starting**
   ```bash
   dfx stop
   dfx start --clean --background
   ```

2. **Build errors**
   ```bash
   # Clean and rebuild
   dfx canister delete --all
   dfx generate
   dfx deploy
   ```

## Additional Resources

- [Internet Computer Documentation](https://internetcomputer.org/docs/current/developer-docs/)
- [Rust Canister Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [Vue.js Documentation](https://vuejs.org/)

## License

[MIT License](LICENSE)

## Contact

For support or queries, please open an issue in the repository.