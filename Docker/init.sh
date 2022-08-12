#!/bin/bash
set -e

echo "init postgresql"
echo "SELECT 'CREATE DATABASE vod' WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'vod')\gexec" | psql

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "vod" <<-EOSQL
    CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
    CREATE TABLE IF NOT EXISTS video (
            id      VARCHAR(32) PRIMARY KEY,
            room    VARCHAR(6) NOT NULL,
            date    TIMESTAMPTZ ,
            url     VARCHAR(128) NOT NULL,
            thumbnail   VARCHAR(128) NOT NULL,
            created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    CREATE TABLE IF NOT EXISTS log (
        id      uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
        total   INTEGER NOT NULL,
        success_encrypted  INTEGER NOT NULL,
        created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );
EOSQL