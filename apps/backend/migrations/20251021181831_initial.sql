-- Add migration script here
CREATE TYPE website_status AS ENUM ('Up', 'Down', 'Unknown');

CREATE TABLE website (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    url TEXT NOT NULL,
    time_added TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE region (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL
);

CREATE TABLE website_tick (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    response_time_ms INTEGER NOT NULL,
    status website_status NOT NULL,
    checked_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    region_id UUID NOT NULL,
    website_id UUID NOT NULL,
    CONSTRAINT fk_region FOREIGN KEY (region_id) REFERENCES region(id) ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT fk_website FOREIGN KEY (website_id) REFERENCES website(id) ON DELETE CASCADE ON UPDATE CASCADE
);

