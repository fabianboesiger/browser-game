# Deploy backend.
cd backend
sqlite3 database.db < up.sql
cargo build --release
target/release/browser-game-backend &
cd ..

# Deploy frontend.
cd frontend
npm run build
serve -s build -l 5000 &
cd ..