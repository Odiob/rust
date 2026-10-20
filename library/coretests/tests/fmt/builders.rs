mod debug_struct {
    use std::fmt;

    #[test]
    fn test_empty() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("Foo").finish()
            }
        }

        assert_eq!("Foo", format!("{Foo:?}"));
        assert_eq!("Foo", format!("{Foo:#?}"));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("Foo").field("bar", &true).finish()
            }
        }

        assert_eq!("Foo { bar: true }", format!("{Foo:?}"));
        assert_eq!(
            "Foo {
    bar: true,
}",
            format!("{Foo:#?}")
        );
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("Foo")
                    .field("bar", &true)
                    .field("baz", &format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        assert_eq!("Foo { bar: true, baz: 10/20 }", format!("{Foo:?}"));
        assert_eq!(
            "Foo {
    bar: true,
    baz: 10/20,
}",
            format!("{Foo:#?}")
        );
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("Foo")
                    .field("bar", &true)
                    .field("baz", &format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("Bar").field("foo", &Foo).field("hello", &"world").finish()
            }
        }

        assert_eq!(
            r#"Bar { foo: Foo { bar: true, baz: 10/20 }, hello: "world" }"#,
            format!("{Bar:?}")
        );
        assert_eq!(
            r#"Bar {
    foo: Foo {
        bar: true,
        baz: 10/20,
    },
    hello: "world",
}"#,
            format!("{Bar:#?}")
        );
    }

    #[test]
    fn test_empty_non_exhaustive() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("Foo").finish_non_exhaustive()
            }
        }

        assert_eq!("Foo { .. }", format!("{Foo:?}"));
        assert_eq!("Foo { .. }", format!("{Foo:#?}"));
    }

    #[test]
    fn test_multiple_and_non_exhaustive() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("Foo")
                    .field("bar", &true)
                    .field("baz", &format_args!("{}/{}", 10, 20))
                    .finish_non_exhaustive()
            }
        }

        assert_eq!("Foo { bar: true, baz: 10/20, .. }", format!("{Foo:?}"));
        assert_eq!(
            "Foo {
    bar: true,
    baz: 10/20,
    ..
}",
            format!("{Foo:#?}")
        );
    }

    #[test]
    fn test_nested_non_exhaustive() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("Foo")
                    .field("bar", &true)
                    .field("baz", &format_args!("{}/{}", 10, 20))
                    .finish_non_exhaustive()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("Bar")
                    .field("foo", &Foo)
                    .field("hello", &"world")
                    .finish_non_exhaustive()
            }
        }

        assert_eq!(
            r#"Bar { foo: Foo { bar: true, baz: 10/20, .. }, hello: "world", .. }"#,
            format!("{Bar:?}")
        );
        assert_eq!(
            r#"Bar {
    foo: Foo {
        bar: true,
        baz: 10/20,
        ..
    },
    hello: "world",
    ..
}"#,
            format!("{Bar:#?}")
        );
    }

    #[test]
    fn test_field_with() {
        struct Foo;
        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("Foo")
                    .field_with("bar", |f| f.write_str("true"))
                    .field_with("baz", |f| f.write_str("false"))
                    .finish()
            }
        }

        assert_eq!("Foo {\n    bar: true,\n    baz: false,\n}", format!("{Foo:#?}"))
    }
}

mod debug_tuple {
    use std::fmt;

    #[test]
    fn test_empty() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_tuple("Foo").finish()
            }
        }

        assert_eq!("Foo", format!("{Foo:?}"));
        assert_eq!("Foo", format!("{Foo:#?}"));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_tuple("Foo").field(&true).finish()
            }
        }

        assert_eq!("Foo(true)", format!("{Foo:?}"));
        assert_eq!(
            "Foo(
    true,
)",
            format!("{Foo:#?}")
        );
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_tuple("Foo").field(&true).field(&format_args!("{}/{}", 10, 20)).finish()
            }
        }

        assert_eq!("Foo(true, 10/20)", format!("{Foo:?}"));
        assert_eq!(
            "Foo(
    true,
    10/20,
)",
            format!("{Foo:#?}")
        );
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_tuple("Foo").field(&true).field(&format_args!("{}/{}", 10, 20)).finish()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_tuple("Bar").field(&Foo).field(&"world").finish()
            }
        }

        assert_eq!(r#"Bar(Foo(true, 10/20), "world")"#, format!("{Bar:?}"));
        assert_eq!(
            r#"Bar(
    Foo(
        true,
        10/20,
    ),
    "world",
)"#,
            format!("{Bar:#?}")
        );
    }

    #[test]
    fn test_empty_non_exhaustive() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_tuple("Foo").finish_non_exhaustive()
            }
        }

        assert_eq!("Foo(..)", format!("{Foo:?}"));
        assert_eq!("Foo(..)", format!("{Foo:#?}"));
    }

    #[test]
    fn test_multiple_and_non_exhaustive() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_tuple("Foo")
                    .field(&true)
                    .field(&format_args!("{}/{}", 10, 20))
                    .finish_non_exhaustive()
            }
        }

        assert_eq!("Foo(true, 10/20, ..)", format!("{Foo:?}"));
        assert_eq!(
            "Foo(
    true,
    10/20,
    ..
)",
            format!("{Foo:#?}")
        );
    }

    #[test]
    fn test_nested_non_exhaustive() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_tuple("Foo")
                    .field(&true)
                    .field(&format_args!("{}/{}", 10, 20))
                    .finish_non_exhaustive()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_tuple("Bar").field(&Foo).field(&"world").finish_non_exhaustive()
            }
        }

        assert_eq!(r#"Bar(Foo(true, 10/20, ..), "world", ..)"#, format!("{Bar:?}"));
        assert_eq!(
            r#"Bar(
    Foo(
        true,
        10/20,
        ..
    ),
    "world",
    ..
)"#,
            format!("{Bar:#?}")
        );
    }
}

mod debug_map {
    use std::fmt;

    #[test]
    fn test_empty() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map().finish()
            }
        }

        assert_eq!("{}", format!("{Foo:?}"));
        assert_eq!("{}", format!("{Foo:#?}"));
    }

    #[test]
    fn test_single() {
        struct Entry;

        impl fmt::Debug for Entry {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map().entry(&"bar", &true).finish()
            }
        }

        struct KeyValue;

        impl fmt::Debug for KeyValue {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map().key(&"bar").value(&true).finish()
            }
        }

        assert_eq!(format!("{Entry:?}"), format!("{KeyValue:?}"));
        assert_eq!(format!("{Entry:#?}"), format!("{KeyValue:#?}"));

        assert_eq!(r#"{"bar": true}"#, format!("{Entry:?}"));
        assert_eq!(
            r#"{
    "bar": true,
}"#,
            format!("{Entry:#?}")
        );
    }

    #[test]
    fn test_multiple() {
        struct Entry;

        impl fmt::Debug for Entry {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map()
                    .entry(&"bar", &true)
                    .entry(&10, &format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct KeyValue;

        impl fmt::Debug for KeyValue {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map()
                    .key(&"bar")
                    .value(&true)
                    .key(&10)
                    .value(&format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        assert_eq!(format!("{Entry:?}"), format!("{KeyValue:?}"));
        assert_eq!(format!("{Entry:#?}"), format!("{KeyValue:#?}"));

        assert_eq!(r#"{"bar": true, 10: 10/20}"#, format!("{Entry:?}"));
        assert_eq!(
            r#"{
    "bar": true,
    10: 10/20,
}"#,
            format!("{Entry:#?}")
        );
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map()
                    .entry(&"bar", &true)
                    .entry(&10, &format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map().entry(&"foo", &Foo).entry(&Foo, &"world").finish()
            }
        }

        assert_eq!(
            r#"{"foo": {"bar": true, 10: 10/20}, {"bar": true, 10: 10/20}: "world"}"#,
            format!("{Bar:?}")
        );
        assert_eq!(
            r#"{
    "foo": {
        "bar": true,
        10: 10/20,
    },
    {
        "bar": true,
        10: 10/20,
    }: "world",
}"#,
            format!("{Bar:#?}")
        );
    }

    #[test]
    fn test_entry_err() {
        // Ensure errors in a map entry don't trigger panics (#65231)
        use std::fmt::Write;

        struct ErrorFmt;

        impl fmt::Debug for ErrorFmt {
            fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
                Err(fmt::Error)
            }
        }

        struct KeyValue<K, V>(usize, K, V);

        impl<K, V> fmt::Debug for KeyValue<K, V>
        where
            K: fmt::Debug,
            V: fmt::Debug,
        {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                let mut map = fmt.debug_map();

                for _ in 0..self.0 {
                    map.entry(&self.1, &self.2);
                }

                map.finish()
            }
        }

        let mut buf = String::new();

        assert!(write!(&mut buf, "{:?}", KeyValue(1, ErrorFmt, "bar")).is_err());
        assert!(write!(&mut buf, "{:?}", KeyValue(1, "foo", ErrorFmt)).is_err());

        assert!(write!(&mut buf, "{:?}", KeyValue(2, ErrorFmt, "bar")).is_err());
        assert!(write!(&mut buf, "{:?}", KeyValue(2, "foo", ErrorFmt)).is_err());
    }

    #[test]
    #[should_panic]
    fn test_invalid_key_when_entry_is_incomplete() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map().key(&"bar").key(&"invalid").finish()
            }
        }

        let _ = format!("{Foo:?}");
    }

    #[test]
    #[should_panic]
    fn test_invalid_finish_incomplete_entry() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map().key(&"bar").finish()
            }
        }

        let _ = format!("{Foo:?}");
    }

    #[test]
    #[should_panic]
    fn test_invalid_value_before_key() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map().value(&"invalid").key(&"bar").finish()
            }
        }

        let _ = format!("{Foo:?}");
    }

    #[test]
    fn test_empty_non_exhaustive() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map().finish_non_exhaustive()
            }
        }

        assert_eq!("{..}", format!("{Foo:?}"));
        assert_eq!("{..}", format!("{Foo:#?}"));
    }

    #[test]
    fn test_multiple_and_non_exhaustive() {
        struct Entry;

        impl fmt::Debug for Entry {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map()
                    .entry(&"bar", &true)
                    .entry(&10, &format_args!("{}/{}", 10, 20))
                    .finish_non_exhaustive()
            }
        }

        struct KeyValue;

        impl fmt::Debug for KeyValue {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map()
                    .key(&"bar")
                    .value(&true)
                    .key(&10)
                    .value(&format_args!("{}/{}", 10, 20))
                    .finish_non_exhaustive()
            }
        }

        assert_eq!(format!("{Entry:?}"), format!("{KeyValue:?}"));
        assert_eq!(format!("{Entry:#?}"), format!("{KeyValue:#?}"));

        assert_eq!(r#"{"bar": true, 10: 10/20, ..}"#, format!("{Entry:?}"));
        assert_eq!(
            r#"{
    "bar": true,
    10: 10/20,
    ..
}"#,
            format!("{Entry:#?}")
        );
    }

    #[test]
    fn test_nested_non_exhaustive() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map()
                    .entry(&"bar", &true)
                    .entry(&10, &format_args!("{}/{}", 10, 20))
                    .finish_non_exhaustive()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map().entry(&"foo", &Foo).entry(&Foo, &"world").finish_non_exhaustive()
            }
        }

        assert_eq!(
            r#"{"foo": {"bar": true, 10: 10/20, ..}, {"bar": true, 10: 10/20, ..}: "world", ..}"#,
            format!("{Bar:?}")
        );
        assert_eq!(
            r#"{
    "foo": {
        "bar": true,
        10: 10/20,
        ..
    },
    {
        "bar": true,
        10: 10/20,
        ..
    }: "world",
    ..
}"#,
            format!("{Bar:#?}")
        );
    }
}

mod debug_set {
    use std::fmt;

    #[test]
    fn test_empty() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_set().finish()
            }
        }

        assert_eq!("{}", format!("{Foo:?}"));
        assert_eq!("{}", format!("{Foo:#?}"));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_set().entry(&true).finish()
            }
        }

        assert_eq!("{true}", format!("{Foo:?}"));
        assert_eq!(
            "{
    true,
}",
            format!("{Foo:#?}")
        );
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_set().entry(&true).entry(&format_args!("{}/{}", 10, 20)).finish()
            }
        }

        assert_eq!("{true, 10/20}", format!("{Foo:?}"));
        assert_eq!(
            "{
    true,
    10/20,
}",
            format!("{Foo:#?}")
        );
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_set().entry(&true).entry(&format_args!("{}/{}", 10, 20)).finish()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_set().entry(&Foo).entry(&"world").finish()
            }
        }

        assert_eq!(r#"{{true, 10/20}, "world"}"#, format!("{Bar:?}"));
        assert_eq!(
            r#"{
    {
        true,
        10/20,
    },
    "world",
}"#,
            format!("{Bar:#?}")
        );
    }

    #[test]
    fn test_empty_non_exhaustive() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_set().finish_non_exhaustive()
            }
        }

        assert_eq!("{..}", format!("{Foo:?}"));
        assert_eq!("{..}", format!("{Foo:#?}"));
    }

    #[test]
    fn test_multiple_and_non_exhaustive() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_set()
                    .entry(&true)
                    .entry(&format_args!("{}/{}", 10, 20))
                    .finish_non_exhaustive()
            }
        }

        assert_eq!("{true, 10/20, ..}", format!("{Foo:?}"));
        assert_eq!(
            "{
    true,
    10/20,
    ..
}",
            format!("{Foo:#?}")
        );
    }

    #[test]
    fn test_nested_non_exhaustive() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_set()
                    .entry(&true)
                    .entry(&format_args!("{}/{}", 10, 20))
                    .finish_non_exhaustive()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_set().entry(&Foo).entry(&"world").finish_non_exhaustive()
            }
        }

        assert_eq!(r#"{{true, 10/20, ..}, "world", ..}"#, format!("{Bar:?}"));
        assert_eq!(
            r#"{
    {
        true,
        10/20,
        ..
    },
    "world",
    ..
}"#,
            format!("{Bar:#?}")
        );
    }
}

mod debug_list {
    use std::fmt;

    #[test]
    fn test_empty() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_list().finish()
            }
        }

        assert_eq!("[]", format!("{Foo:?}"));
        assert_eq!("[]", format!("{Foo:#?}"));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_list().entry(&true).finish()
            }
        }

        assert_eq!("[true]", format!("{Foo:?}"));
        assert_eq!(
            "[
    true,
]",
            format!("{Foo:#?}")
        );
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_list().entry(&true).entry(&format_args!("{}/{}", 10, 20)).finish()
            }
        }

        assert_eq!("[true, 10/20]", format!("{Foo:?}"));
        assert_eq!(
            "[
    true,
    10/20,
]",
            format!("{Foo:#?}")
        );
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_list().entry(&true).entry(&format_args!("{}/{}", 10, 20)).finish()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_list().entry(&Foo).entry(&"world").finish()
            }
        }

        assert_eq!(r#"[[true, 10/20], "world"]"#, format!("{Bar:?}"));
        assert_eq!(
            r#"[
    [
        true,
        10/20,
    ],
    "world",
]"#,
            format!("{Bar:#?}")
        );
    }

    #[test]
    fn test_empty_non_exhaustive() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_list().finish_non_exhaustive()
            }
        }

        assert_eq!("[..]", format!("{Foo:?}"));
        assert_eq!("[..]", format!("{Foo:#?}"));
    }

    #[test]
    fn test_multiple_non_exhaustive() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_list()
                    .entry(&true)
                    .entry(&format_args!("{}/{}", 10, 20))
                    .finish_non_exhaustive()
            }
        }

        assert_eq!("[true, 10/20, ..]", format!("{Foo:?}"));
        assert_eq!(
            "[
    true,
    10/20,
    ..
]",
            format!("{Foo:#?}")
        );
    }

    #[test]
    fn test_nested_non_exhaustive() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_list()
                    .entry(&true)
                    .entry(&format_args!("{}/{}", 10, 20))
                    .finish_non_exhaustive()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_list().entry(&Foo).entry(&"world").finish_non_exhaustive()
            }
        }

        assert_eq!(r#"[[true, 10/20, ..], "world", ..]"#, format!("{Bar:?}"));
        assert_eq!(
            r#"[
    [
        true,
        10/20,
        ..
    ],
    "world",
    ..
]"#,
            format!("{Bar:#?}")
        );
    }
}

#[test]
fn test_formatting_parameters_are_forwarded() {
    use std::collections::{BTreeMap, BTreeSet};
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Foo {
        bar: u32,
        baz: u32,
    }
    let struct_ = Foo { bar: 1024, baz: 7 };
    let tuple = (1024, 7);
    let list = [1024, 7];
    let mut map = BTreeMap::new();
    map.insert("bar", 1024);
    map.insert("baz", 7);
    let mut set = BTreeSet::new();
    set.insert(1024);
    set.insert(7);

    assert_eq!(format!("{struct_:03?}"), "Foo { bar: 1024, baz: 007 }");
    assert_eq!(format!("{tuple:03?}"), "(1024, 007)");
    assert_eq!(format!("{list:03?}"), "[1024, 007]");
    assert_eq!(format!("{map:03?}"), r#"{"bar": 1024, "baz": 007}"#);
    assert_eq!(format!("{set:03?}"), "{007, 1024}");
    assert_eq!(
        format!("{struct_:#03?}"),
        "
Foo {
    bar: 1024,
    baz: 007,
}
    "
        .trim()
    );
    assert_eq!(
        format!("{tuple:#03?}"),
        "
(
    1024,
    007,
)
    "
        .trim()
    );
    assert_eq!(
        format!("{list:#03?}"),
        "
[
    1024,
    007,
]
    "
        .trim()
    );
    assert_eq!(
        format!("{map:#03?}"),
        r#"
{
    "bar": 1024,
    "baz": 007,
}
    "#
        .trim()
    );
    assert_eq!(
        format!("{set:#03?}"),
        "
{
    007,
    1024,
}
    "
        .trim()
    );
}

mod pretty_padding {
    use std::fmt::{self, Formatter, Write};

    /// Writes its text with `write_str`.
    struct Raw(&'static str);

    impl fmt::Debug for Raw {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.write_str(self.0)
        }
    }

    /// Writes its text with one `write_char` per character.
    struct Chars(&'static str);

    impl fmt::Debug for Chars {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            self.0.chars().try_for_each(|c| f.write_char(c))
        }
    }

    /// `N { i: .. }` nested `self.0` levels deep around `x`.
    struct Nest(usize);

    impl fmt::Debug for Nest {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self.0 {
                0 => f.write_str("x"),
                n => f.debug_struct("N").field("i", &Nest(n - 1)).finish(),
            }
        }
    }

    /// The pretty output of `Nest(depth)`.
    fn nest_pretty(depth: usize) -> String {
        if depth == 0 {
            return "x".to_string();
        }
        let mut lines = vec!["N {".to_string()];
        lines.extend((1..depth).map(|k| format!("{}i: N {{", "    ".repeat(k))));
        lines.push(format!("{}i: x,", "    ".repeat(depth)));
        lines.extend((1..depth).rev().map(|k| format!("{}}},", "    ".repeat(k))));
        lines.push("}".to_string());
        lines.join("\n")
    }

    #[test]
    fn test_deep_nesting() {
        for depth in [0, 1, 2, 200] {
            assert_eq!(nest_pretty(depth), format!("{:#?}", Nest(depth)));
        }
    }

    #[test]
    fn test_newlines_in_values() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
                let list: [&dyn fmt::Debug; 2] = [&Raw("a\nb"), &Chars("\nc\n")];
                fmt.debug_struct("Foo").field("l", &list).finish()
            }
        }

        assert_eq!(
            "Foo {\n    l: [\n        a\n        b,\n        \n        c\n        ,\n    ],\n}",
            format!("{Foo:#?}")
        );
    }

    #[test]
    fn test_map_keys_with_newlines() {
        struct Map;

        impl fmt::Debug for Map {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
                fmt.debug_map()
                    .entry(&Raw("k1\nk2"), &Raw("v1\nv2"))
                    .entry(&Raw("k\n"), &Chars("v"))
                    .finish()
            }
        }

        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("Foo").field("m", &Map).finish()
            }
        }

        assert_eq!("{\n    k1\n    k2: v1\n    v2,\n    k\n    : v,\n}", format!("{Map:#?}"));
        assert_eq!(
            "Foo {\n    m: {\n        k1\n        k2: v1\n        v2,\n        k\n        : v,\n    },\n}",
            format!("{Foo:#?}")
        );
    }

    #[test]
    fn test_with_options_inside_pretty() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
                let options = fmt.options();
                fmt.with_options(options).debug_struct("Foo").field("a", &1).finish()
            }
        }

        assert_eq!("[\n    Foo {\n        a: 1,\n    },\n]", format!("{:#?}", [Foo]));
    }

    #[test]
    fn test_new_formatter_inside_pretty() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
                let options = fmt.options();
                fmt::Debug::fmt(&Nest(2), &mut Formatter::new(fmt, options))
            }
        }

        assert_eq!(
            "[\n    N {\n        i: N {\n            i: x,\n        },\n    },\n]",
            format!("{:#?}", [Foo])
        );
    }

    #[test]
    fn test_write_inside_pretty() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
                write!(fmt, "<{:#?}>", Nest(1))?;
                fmt::Write::write_fmt(fmt, format_args!("[{:#?}]\n", Nest(1)))?;
                let value = format_args!("{:#?}|{:?}", Nest(2), Nest(1));
                fmt.debug_struct("A").field("f", &value).finish()
            }
        }

        assert_eq!(
            "[\n    <N {\n        i: x,\n    }>[N {\n        i: x,\n    }]\n    A {\n        \
             f: N {\n            i: N {\n                i: x,\n            },\n        }\
             |N { i: x },\n    },\n]",
            format!("{:#?}", [Foo])
        );
    }

    #[test]
    fn test_caught_panic_inside_pretty() {
        struct Boom;

        impl fmt::Debug for Boom {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
                fmt.write_str("b\n")?;
                panic!("boom")
            }
        }

        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
                let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    fmt.debug_struct("P").field("a", &Boom).finish()
                }));
                assert!(caught.is_err());
                fmt.debug_struct("Q").field("c", &"d\ne").finish()?;
                fmt.write_str("\nafter")
            }
        }

        assert_eq!(
            "[\n    P {\n        a: b\n    Q {\n        c: \"d\\ne\",\n    }\n    after,\n    \
             P {\n        a: b\n    Q {\n        c: \"d\\ne\",\n    }\n    after,\n]",
            format!("{:#?}", [Foo, Foo])
        );
    }
}
