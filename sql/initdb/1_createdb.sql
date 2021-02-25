SELECT 'CREATE DATABASE testing_db'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'testing_db')\gexec
