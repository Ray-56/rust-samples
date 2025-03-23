-- Add migration script here
CREATE TABLE todos (
    id INT AUTO_INCREMENT PRIMARY KEY,
    description VARCHAR(255) NOT NULL,
    status ENUM(
        'pending',
        'doing',
        'completed'
    ) NOT NULL DEFAULT 'pending',
    position INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

INSERT INTO
    todos (description, status, position)
VALUES ('Learn SQL', 'pending', 1000),
    ('Learn Node.js', 'pending', 2000),
    (
        'Learn Express.js',
        'pending',
        3000
    ),
    ('Learn MySQL', 'doing', 4000),
    (
        'Learn Sequelize',
        'completed',
        5000
    );