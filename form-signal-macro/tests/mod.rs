#[macro_use]
extern crate form_signal_macro;

#[test]
fn basic_test() {
    _ = leptos::create_runtime();

    #[derive(FormState, PartialEq, Eq, Debug, Clone, Default)]
    struct TestStruct {
        value: String,
    }

    let d = TestStruct {
        value: "test".to_string(),
    };

    let dd = TestStructFormState::from(d.clone());

    let dd: TestStruct = (&dd).into();

    assert_eq!(d, dd);
}

#[test]
fn nested_test() {
    _ = leptos::create_runtime();

    #[derive(FormState, PartialEq, Eq, Debug, Clone, Default)]
    struct TestNestedStruct {
        value: String,
    }

    #[derive(FormState, PartialEq, Eq, Debug, Clone, Default)]
    struct TestStruct {
        #[nested]
        value: TestNestedStruct,
    }

    let d = TestStruct {
        value: TestNestedStruct {
            value: "test".to_string(),
        },
    };

    let dd = TestStructFormState::from(d.clone());

    let dd: TestStruct = (&dd).into();

    assert_eq!(d, dd);
}

#[test]
fn tuple_nested_test() {
    _ = leptos::create_runtime();

    #[derive(FormState, PartialEq, Eq, Debug, Clone, Default)]
    struct TestNestedStruct(String);

    #[derive(FormState, PartialEq, Eq, Debug, Clone, Default)]
    struct TestStruct(#[nested] TestNestedStruct);

    let d = TestStruct(TestNestedStruct("test".to_string()));

    let dd = TestStructFormState::from(d.clone());

    let dd: TestStruct = (&dd).into();

    assert_eq!(d, dd);
}

#[test]
fn nested_generics() {
    _ = leptos::create_runtime();

    #[derive(FormState, PartialEq, Eq, Debug, Clone, Default)]
    struct TestNestedStruct {
        #[iterable]
        value: Vec<String>,
    }

    #[derive(FormState, PartialEq, Eq, Debug, Clone, Default)]
    struct TestStruct {
        #[nested]
        value: TestNestedStruct,
    }

    let d = TestStruct {
        value: TestNestedStruct {
            value: vec![
                "test".to_string(),
                "test 1".to_string(),
                "test 2".to_string(),
            ],
        },
    };

    let dd = TestStructFormState::from(d.clone());

    let dd: TestStruct = (&dd).into();

    assert_eq!(d, dd);
}
