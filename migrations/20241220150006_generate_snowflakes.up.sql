CREATE SEQUENCE IF NOT EXISTS public.global_id_sequence;

CREATE OR REPLACE FUNCTION id_generator(OUT result TEXT) AS $$
DECLARE
    epoch BIGINT := 1610850820000;
    seq_id BIGINT;
    now_millis BIGINT;
    shard_id INT := 1;
    generated_id BIGINT;
BEGIN
    -- Get the next sequence value modulo 1024
    SELECT nextval('public.global_id_sequence') % 1024 INTO seq_id;

    -- Get the current time in milliseconds since epoch
    SELECT FLOOR(EXTRACT(EPOCH FROM clock_timestamp()) * 1000) INTO now_millis;

    -- Generate the ID as a BIGINT
    generated_id := (now_millis - epoch) << 23;
    generated_id := generated_id | (shard_id << 10);
    generated_id := generated_id | seq_id;

    -- Convert the BIGINT result to a string
    result := generated_id::TEXT;
END;
$$ LANGUAGE PLPGSQL;
