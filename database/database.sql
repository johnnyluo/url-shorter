CREATE DATABASE IF NOT EXISTS url_shortener;
USE url_shortener;
CREATE TABLE IF NOT EXISTS urls (
    id INT NOT NULL AUTO_INCREMENT,
    user_id INT NOT NULL,
    target_url VARCHAR(1024) NOT NULL,
    shortened_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);
CREATE TABLE IF NOT EXISTS users (
    id INT NOT NULL AUTO_INCREMENT,
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);
INSERT INTO users (username, password) VALUES ('admin', 'admin');
INSERT INTO urls (user_id, target_url, shortened_id) VALUES (1, 'https://www.google.com', 'abc123');