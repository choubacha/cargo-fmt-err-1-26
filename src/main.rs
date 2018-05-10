mod foo {
    macro_rules! test {
        ($type_to_check:ident, $type_i:expr) => {
            pub fn $type_to_check() {
                let json = format!(
                    // Will indent 3 times with each cargo fmt call
                    r#"{{
                        "id":"1",
                        "paging_token":"7",
                        "type_i":{},
                        "type":"{}",
                        "transaction_hash":"123"
                    }}"#,
                    $type_i,
                    stringify!($type_to_check),
                );
                println!("{}", json);
            }
        };
    }

    test!(bar, 10);
}

fn main() {
    // Does not change with cargo fmt
    let test = r#"{{
        "id":"1",
        "paging_token":"7",
        "type_i":{},
        "type":"{}",
        "transaction_hash":"123"
    }}"#;

    // Does not change with cargo fmt
    let test = r#"{{
            "id":"1",
            "paging_token":"7",
            "type_i":{},
            "type":"{}",
            "transaction_hash":"123"
        }}"#;

    // Does not change with cargo fmt
    println!(
        r#"{{
            "id":"1",
            "paging_token":"7",
            "type_i":{},
            "type":"{}",
            "transaction_hash":"123"
        }}"#,
        12, "thing"
    );
    foo::bar();
}
