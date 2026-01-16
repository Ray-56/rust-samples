CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    description VARCHAR(255) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (
        status IN ('pending', 'doing', 'completed')
    ),
    position INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC'),
    deleted_at TIMESTAMPTZ NULL
);

-- 创建触发器函数，用于更新 updated_at 字段
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP AT TIME ZONE 'UTC';
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 为 todos 表创建触发器
CREATE TRIGGER set_updated_at
BEFORE UPDATE ON todos
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

INSERT INTO todos (description, status, position)
VALUES 
    ('Learn SQL', 'pending', 1000),
    ('Learn Node.js', 'pending', 2000),
    ('Learn Express.js', 'pending', 3000),
    ('Learn MySQL', 'doing', 4000),
    ('Learn Sequelize', 'completed', 5000)