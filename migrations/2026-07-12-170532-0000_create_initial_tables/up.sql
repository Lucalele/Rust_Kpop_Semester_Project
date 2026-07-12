-- Your SQL goes here
PRAGMA foreign_keys = ON;

CREATE TABLE companies (
    company_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    company_name TEXT NOT NULL
);

CREATE TABLE labels (
    label_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    label_name TEXT NOT NULL,
    company_id INTEGER NOT NULL,

    FOREIGN KEY (company_id)
        REFERENCES companies(company_id)
        ON DELETE CASCADE
);

CREATE TABLE idols (
    idol_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    idol_gender TEXT NOT NULL,
    is_soloist BOOLEAN
);

CREATE TABLE idol_names (
    idol_name_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    idol_id INTEGER NOT NULL,
    name TEXT NOT NULL,

    FOREIGN KEY (idol_id)
        REFERENCES idols(idol_id)
        ON DELETE CASCADE,

    UNIQUE (idol_id, name)
);

CREATE TABLE idol_groups (
    group_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    group_name TEXT NOT NULL,
    debut_date DATE,

    gender TEXT NOT NULL
        CHECK (gender IN ('Male', 'Female', 'CoEd'))
);

CREATE TABLE subunits (
    subunit_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    subunit_name TEXT NOT NULL,
    parent_group_id INTEGER NOT NULL,
    debut_date DATE,

    gender TEXT NOT NULL
        CHECK (gender IN ('Male', 'Female', 'CoEd')),

    FOREIGN KEY (parent_group_id)
        REFERENCES idol_groups(group_id)
        ON DELETE CASCADE
);

CREATE TABLE project_groups (
    project_group_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    project_group_name TEXT NOT NULL,
    debut_date DATE,

    gender TEXT NOT NULL
        CHECK (gender IN ('Male', 'Female', 'CoEd'))
);

CREATE TABLE albums (
    album_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title TEXT NOT NULL,
    artist_id INTEGER NOT NULL,
    release_date DATE,
    language TEXT,
    version TEXT
);

CREATE TABLE idol_group_memberships (
    idol_id INTEGER NOT NULL,
    group_id INTEGER NOT NULL,

    PRIMARY KEY (idol_id, group_id),

    FOREIGN KEY (idol_id)
        REFERENCES idols(idol_id)
        ON DELETE CASCADE,

    FOREIGN KEY (group_id)
        REFERENCES idol_groups(group_id)
        ON DELETE CASCADE
);

CREATE TABLE idol_subunit_memberships (
    idol_id INTEGER NOT NULL,
    subunit_id INTEGER NOT NULL,

    PRIMARY KEY (idol_id, subunit_id),

    FOREIGN KEY (idol_id)
        REFERENCES idols(idol_id)
        ON DELETE CASCADE,

    FOREIGN KEY (subunit_id)
        REFERENCES subunits(subunit_id)
        ON DELETE CASCADE
);

CREATE TABLE idol_project_group_memberships (
    idol_id INTEGER NOT NULL,
    project_group_id INTEGER NOT NULL,

    PRIMARY KEY (idol_id, project_group_id),

    FOREIGN KEY (idol_id)
        REFERENCES idols(idol_id)
        ON DELETE CASCADE,

    FOREIGN KEY (project_group_id)
        REFERENCES project_groups(project_group_id)
        ON DELETE CASCADE
);

CREATE TABLE idol_companies (
    idol_id INTEGER NOT NULL,
    company_id INTEGER NOT NULL,

    PRIMARY KEY (idol_id, company_id),

    FOREIGN KEY (idol_id)
        REFERENCES idols(idol_id)
        ON DELETE CASCADE,

    FOREIGN KEY (company_id)
        REFERENCES companies(company_id)
        ON DELETE CASCADE
);

CREATE TABLE idol_labels (
    idol_id INTEGER NOT NULL,
    label_id INTEGER NOT NULL,

    PRIMARY KEY (idol_id, label_id),

    FOREIGN KEY (idol_id)
        REFERENCES idols(idol_id)
        ON DELETE CASCADE,

    FOREIGN KEY (label_id)
        REFERENCES labels(label_id)
        ON DELETE CASCADE
);

CREATE TABLE group_companies (
    group_id INTEGER NOT NULL,
    company_id INTEGER NOT NULL,

    PRIMARY KEY (group_id, company_id),

    FOREIGN KEY (group_id)
        REFERENCES idol_groups(group_id)
        ON DELETE CASCADE,

    FOREIGN KEY (company_id)
        REFERENCES companies(company_id)
        ON DELETE CASCADE
);

CREATE TABLE group_labels (
    group_id INTEGER NOT NULL,
    label_id INTEGER NOT NULL,

    PRIMARY KEY (group_id, label_id),

    FOREIGN KEY (group_id)
        REFERENCES idol_groups(group_id)
        ON DELETE CASCADE,

    FOREIGN KEY (label_id)
        REFERENCES labels(label_id)
        ON DELETE CASCADE
);

CREATE TABLE project_group_parents (
    project_group_id INTEGER NOT NULL,
    parent_group_id INTEGER NOT NULL,

    PRIMARY KEY (project_group_id, parent_group_id),

    FOREIGN KEY (project_group_id)
        REFERENCES project_groups(project_group_id)
        ON DELETE CASCADE,

    FOREIGN KEY (parent_group_id)
        REFERENCES idol_groups(group_id)
        ON DELETE CASCADE
);