-- Remove os índices da tabela de usuários
DROP INDEX IF EXISTS "uq_users_username";

DROP INDEX IF EXISTS "idx_users_email";

DROP INDEX IF EXISTS "idx_users_is_curator";

-- Remove a tabela de usuários
DROP TABLE IF EXISTS "users";

-- Remove a extensão do UUID
DROP EXTENSION IF EXISTS "uuid-ossp";