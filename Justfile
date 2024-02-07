migrateup:
    sqlx migrate run

migratedown:
    sqlx migrate revert

test:
    cargo test

coverage:
    cargo tarpaulin