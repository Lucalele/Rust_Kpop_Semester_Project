-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS project_group_parents;
DROP TABLE IF EXISTS group_labels;
DROP TABLE IF EXISTS group_companies;
DROP TABLE IF EXISTS idol_labels;
DROP TABLE IF EXISTS idol_companies;
DROP TABLE IF EXISTS idol_project_group_memberships;
DROP TABLE IF EXISTS idol_subunit_memberships;
DROP TABLE IF EXISTS idol_group_memberships;

DROP TABLE IF EXISTS albums;
DROP TABLE IF EXISTS project_groups;
DROP TABLE IF EXISTS subunits;
DROP TABLE IF EXISTS idol_groups;
DROP TABLE IF EXISTS idol_names;
DROP TABLE IF EXISTS idols;
DROP TABLE IF EXISTS labels;
DROP TABLE IF EXISTS companies;