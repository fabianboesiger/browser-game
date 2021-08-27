# Deploy backend.
cd backend
sqlite3 database.db < up.sql
cargo run &
cd ..

# Deploy frontend.
cd frontend
npm start &
cd ..