CREATE TABLE IF NOT EXISTS mountain_pass (
                                             id SERIAL PRIMARY KEY,
                                             name VARCHAR(255) NOT NULL,
                                             country VARCHAR(255),
                                             ascent INT,
                                             latitude DECIMAL(9, 6),
                                             longitude DECIMAL(9, 6),
                                             category VARCHAR(255),
                                             UNIQUE (country, name)
);