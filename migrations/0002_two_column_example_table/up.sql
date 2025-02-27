CREATE TABLE IF NOT EXISTS position (
    x INT,
    y INT,

    CHECK (x_must_be_bigger_than_y(x,y))
)