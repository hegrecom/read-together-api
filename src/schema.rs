table! {
    group_members (id) {
        id -> Integer,
        user_id -> Integer,
        group_id -> Integer,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    groups (id) {
        id -> Integer,
        name -> Varchar,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    plan_content_user_activities (id) {
        id -> Integer,
        plan_content_id -> Integer,
        user_id -> Integer,
        activity -> Enum,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    plan_contents (id) {
        id -> Integer,
        plan_id -> Integer,
        sequence -> Integer,
        content -> Text,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    plans (id) {
        id -> Integer,
        group_id -> Integer,
        name -> Varchar,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    plans_participants (id) {
        id -> Integer,
        user_id -> Integer,
        plan_id -> Integer,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    user_tokens (id) {
        id -> Integer,
        user_id -> Integer,
        token -> Varchar,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    users (id) {
        id -> Integer,
        email -> Varchar,
        password -> Varchar,
        name -> Varchar,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

allow_tables_to_appear_in_same_query!(
    group_members,
    groups,
    plan_content_user_activities,
    plan_contents,
    plans,
    plans_participants,
    user_tokens,
    users,
);
