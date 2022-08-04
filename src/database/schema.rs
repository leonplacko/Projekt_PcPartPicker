table! {
    build_size (id) {
        id -> Varchar,
        motherboard_id -> Nullable<Varchar>,
        case_id -> Nullable<Varchar>,
        size -> Varchar,
    }
}

table! {
    cooling (id) {
        id -> Varchar,
        name -> Varchar,
        manufacturer -> Varchar,
        ventilators -> Nullable<Int4>,
        cpu_ventilator -> Nullable<Bool>,
        price -> Float4,
    }
}

table! {
    cpu (id) {
        id -> Varchar,
        name -> Varchar,
        manufacturer -> Varchar,
        cores -> Int4,
        cache -> Varchar,
        speed -> Float4,
        tdp -> Int4,
        price -> Float4,
    }
}

table! {
    gpu (id) {
        id -> Varchar,
        name -> Varchar,
        manufacturer -> Varchar,
        memory -> Int4,
        memory_type -> Varchar,
        tdp -> Int4,
        price -> Float4,
    }
}

table! {
    manufacturer (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

table! {
    motherboards (id) {
        id -> Varchar,
        name -> Varchar,
        manufacturer -> Varchar,
        ram_slots -> Int4,
        sata_slots -> Int4,
        nvme_slots -> Int4,
        price -> Float4,
    }
}

table! {
    pc_case (id) {
        id -> Varchar,
        name -> Varchar,
        manufacturer -> Varchar,
        dimensions -> Varchar,
        ventilators -> Int4,
        available_vent -> Int4,
        price -> Float4,
    }
}

table! {
    power_unit (id) {
        id -> Varchar,
        name -> Varchar,
        manufacturer -> Varchar,
        power -> Int4,
        price -> Float4,
    }
}

table! {
    ram (id) {
        id -> Varchar,
        name -> Varchar,
        manufacturer -> Varchar,
        speed -> Int4,
        capacity -> Int4,
        price -> Float4,
    }
}

table! {
    ram_slot (id) {
        id -> Varchar,
        motherboard_id -> Nullable<Varchar>,
        ram_id -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Varchar,
    }
}

table! {
    socket (id) {
        id -> Varchar,
        motherboard_id -> Nullable<Varchar>,
        cpu_id -> Nullable<Varchar>,
        socket_type -> Varchar,
    }
}

table! {
    storage_slots (id) {
        id -> Varchar,
        motherboard_id -> Nullable<Varchar>,
        storage_id -> Nullable<Varchar>,
        slot -> Varchar,
    }
}

table! {
    storages (id) {
        id -> Varchar,
        name -> Varchar,
        manufacturer -> Varchar,
        capacity -> Int4,
        speed -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
        price -> Float4,
    }
}

table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        password -> Varchar,
        isadmin -> Bool,
    }
}

joinable!(build_size -> motherboards (motherboard_id));
joinable!(build_size -> pc_case (case_id));
joinable!(ram_slot -> motherboards (motherboard_id));
joinable!(ram_slot -> ram (ram_id));
joinable!(socket -> cpu (cpu_id));
joinable!(socket -> motherboards (motherboard_id));
joinable!(storage_slots -> motherboards (motherboard_id));
joinable!(storage_slots -> storages (storage_id));

allow_tables_to_appear_in_same_query!(
    build_size,
    cooling,
    cpu,
    gpu,
    manufacturer,
    motherboards,
    pc_case,
    power_unit,
    ram,
    ram_slot,
    socket,
    storage_slots,
    storages,
    users,
);
