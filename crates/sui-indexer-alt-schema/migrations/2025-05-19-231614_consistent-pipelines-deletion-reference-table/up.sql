CREATE TABLE IF NOT EXISTS obj_info_deletion_reference
(
    object_id                   BYTEA         NOT NULL,
    cp_sequence_number          BIGINT        NOT NULL,
    -- Primary and sole query pattern is `SELECT * FROM obj_info_deletion_reference WHERE
    -- cp_sequence_number >= from AND cp_sequence_number < to_exclusive`
    PRIMARY KEY (cp_sequence_number, object_id)
);

CREATE TABLE IF NOT EXISTS coin_balance_buckets_deletion_reference
(
    object_id                   BYTEA         NOT NULL,
    cp_sequence_number          BIGINT        NOT NULL,
    -- Primary and sole query pattern is `SELECT * FROM coin_balance_buckets_deletion_reference
    -- WHERE cp_sequence_number >= from AND cp_sequence_number < to_exclusive`
    PRIMARY KEY (cp_sequence_number, object_id)
);

CREATE TABLE IF NOT EXISTS obj_versions_deletion_reference
(
    object_id                   BYTEA         NOT NULL,
    object_version              BIGINT        NOT NULL,
    cp_sequence_number          BIGINT        NOT NULL,
    PRIMARY KEY (object_id, object_version)
);

CREATE INDEX IF NOT EXISTS obj_versions_deletion_reference_cp_sequence_number
ON obj_versions_deletion_reference (cp_sequence_number);
