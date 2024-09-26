DROP TABLE IF EXISTS implants;
DROP TABLE IF EXISTS tasks;

CREATE TABLE implants (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    implant_id TEXT NOT NULL,
    username TEXT NOT NULL,
    domain TEXT NOT NULL,
    machine TEXT NOT NULL,
    process TEXT NOT NULL,
    versionOS  TEXT NOT NULL,
    arch TEXT NOT NULL,
    pid TEXT NOT NULL,
    lastcheckin TEXT NOT NULL,
    dead TEXT NOT NULL,
    naptime INTEGER NOT NULL,
    checkTask TEXT
);

CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    implant_id TEXT NOT NULL,
    operator_id TEXT NOT NULL,
    taskcmd TEXT NOT NULL,
    taskresponse TEXT,
    completedtask TEXT NOT NULL
);
