// @generated automatically by Diesel CLI.

diesel::table! {
    albums (album_id) {
        album_id -> Integer,
        title -> Text,
        artist_id -> Integer,
        artist_type -> Text,
        release_date -> Nullable<Date>,
        language -> Nullable<Text>,
        version -> Nullable<Text>,
    }
}

diesel::table! {
    companies (company_id) {
        company_id -> Integer,
        company_name -> Text,
    }
}

diesel::table! {
    group_companies (group_id, company_id) {
        group_id -> Integer,
        company_id -> Integer,
    }
}

diesel::table! {
    group_labels (group_id, label_id) {
        group_id -> Integer,
        label_id -> Integer,
    }
}

diesel::table! {
    idol_companies (idol_id, company_id) {
        idol_id -> Integer,
        company_id -> Integer,
    }
}

diesel::table! {
    idol_group_memberships (idol_id, group_id) {
        idol_id -> Integer,
        group_id -> Integer,
    }
}

diesel::table! {
    idol_groups (group_id) {
        group_id -> Integer,
        group_name -> Text,
        debut_date -> Nullable<Date>,
        gender -> Text,
    }
}

diesel::table! {
    idol_labels (idol_id, label_id) {
        idol_id -> Integer,
        label_id -> Integer,
    }
}

diesel::table! {
    idol_names (idol_name_id) {
        idol_name_id -> Integer,
        idol_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    idol_project_group_memberships (idol_id, project_group_id) {
        idol_id -> Integer,
        project_group_id -> Integer,
    }
}

diesel::table! {
    idol_subunit_memberships (idol_id, subunit_id) {
        idol_id -> Integer,
        subunit_id -> Integer,
    }
}

diesel::table! {
    idols (idol_id) {
        idol_id -> Integer,
        idol_gender -> Text,
        is_soloist -> Nullable<Bool>,
    }
}

diesel::table! {
    labels (label_id) {
        label_id -> Integer,
        label_name -> Text,
        company_id -> Integer,
    }
}

diesel::table! {
    project_group_parents (project_group_id, parent_group_id) {
        project_group_id -> Integer,
        parent_group_id -> Integer,
    }
}

diesel::table! {
    project_groups (project_group_id) {
        project_group_id -> Integer,
        project_group_name -> Text,
        debut_date -> Nullable<Date>,
        gender -> Text,
    }
}

diesel::table! {
    subunits (subunit_id) {
        subunit_id -> Integer,
        subunit_name -> Text,
        parent_group_id -> Integer,
        debut_date -> Nullable<Date>,
        gender -> Text,
    }
}

diesel::joinable!(group_companies -> companies (company_id));
diesel::joinable!(group_companies -> idol_groups (group_id));
diesel::joinable!(group_labels -> idol_groups (group_id));
diesel::joinable!(group_labels -> labels (label_id));
diesel::joinable!(idol_companies -> companies (company_id));
diesel::joinable!(idol_companies -> idols (idol_id));
diesel::joinable!(idol_group_memberships -> idol_groups (group_id));
diesel::joinable!(idol_group_memberships -> idols (idol_id));
diesel::joinable!(idol_labels -> idols (idol_id));
diesel::joinable!(idol_labels -> labels (label_id));
diesel::joinable!(idol_names -> idols (idol_id));
diesel::joinable!(idol_project_group_memberships -> idols (idol_id));
diesel::joinable!(idol_project_group_memberships -> project_groups (project_group_id));
diesel::joinable!(idol_subunit_memberships -> idols (idol_id));
diesel::joinable!(idol_subunit_memberships -> subunits (subunit_id));
diesel::joinable!(labels -> companies (company_id));
diesel::joinable!(project_group_parents -> idol_groups (parent_group_id));
diesel::joinable!(project_group_parents -> project_groups (project_group_id));
diesel::joinable!(subunits -> idol_groups (parent_group_id));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    companies,
    group_companies,
    group_labels,
    idol_companies,
    idol_group_memberships,
    idol_groups,
    idol_labels,
    idol_names,
    idol_project_group_memberships,
    idol_subunit_memberships,
    idols,
    labels,
    project_group_parents,
    project_groups,
    subunits,
);
