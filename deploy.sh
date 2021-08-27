# Deploy backend.
cd backend
cargo build --release
cd ..

# Deploy frontend.
cd frontend
npm run build
serve -s build -l 5000
cd ..