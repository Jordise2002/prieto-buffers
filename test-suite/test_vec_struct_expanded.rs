#[rustc_test_marker = "test::array::test_vec_struct"]
#[doc(hidden)]
pub const test_vec_struct: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test::array::test_vec_struct"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "src/test/array.rs",
        start_line: 88usize,
        start_col: 4usize,
        end_line: 88usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::UnitTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_vec_struct()),
    ),
};
fn test_vec_struct() {
    use prieto_buffers::PrietoBuffersSerde;
    use rand::RngExt;
    struct InnerTestStruct {
        a: Vec<u8>,
    }
    impl InnerTestStruct {
        pub fn skip_field(bytes: &[u8], field_type: prieto_buffers::FieldType) -> u32 {
            match field_type {
                prieto_buffers::FieldType::Struct => {
                    let field_count = bytes[0] as u32;
                    let mut offset: u32 = 1;
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "Skipping struct with {0} fields\n",
                                field_count,
                            ),
                        );
                    };
                    for _ in 0..field_count {
                        let field_header = bytes[offset as usize];
                        offset += 1;
                        let field_type = prieto_buffers::FieldType::from_u8(
                                field_header >> 5,
                            )
                            .unwrap();
                        offset
                            += InnerTestStruct::skip_field(
                                &bytes[offset as usize..],
                                field_type,
                            );
                    }
                    offset
                }
                prieto_buffers::FieldType::Array => {
                    let mut size: u32 = 0;
                    size.deserialize(&bytes);
                    let mut offset: u32 = size_of::<u32>() as u32;
                    let field_header = bytes[offset as usize];
                    let field_type = prieto_buffers::FieldType::from_u8(field_header)
                        .unwrap();
                    offset += 1;
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "Skipping array of type {0:?} with size {1}\n",
                                field_type,
                                size,
                            ),
                        );
                    };
                    for _ in 0..size {
                        offset
                            += InnerTestStruct::skip_field(
                                &bytes[offset as usize..],
                                field_type,
                            );
                    }
                    offset
                }
                _ => {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "Skipping field of type {0:?} with size {1}\n",
                                field_type,
                                field_type.get_size(),
                            ),
                        );
                    };
                    field_type.get_size() as u32
                }
            }
        }
    }
    impl PrietoBuffersSerde for InnerTestStruct {
        fn get_size_with_options(
            &self,
            options: prieto_buffers::SerializeOptions,
        ) -> u32 {
            let mut size = 1;
            if self.a.should_serialize() {
                size += self.a.get_size_with_options(options) + 1;
            }
            size
        }
        fn get_type(&self) -> prieto_buffers::FieldType {
            prieto_buffers::FieldType::Struct
        }
        fn serialize_with_options(
            &self,
            bytes: &mut [u8],
            options: prieto_buffers::SerializeOptions,
        ) {
            let mut offset: u32 = 0;
            let mut field_amount: u8 = 0;
            if self.a.should_serialize() {
                field_amount += 1;
            }
            bytes[offset as usize] = field_amount;
            offset += 1;
            {
                let mut options = options.clone();
                if false {
                    options.is_zero_ended_string = true;
                }
                if self.a.should_serialize() {
                    self.a
                        .serialize_with_header(
                            0u8,
                            &mut bytes[offset as usize..],
                            Some(options),
                        );
                    offset += self.a.get_size() + 1;
                }
            }
        }
        fn deserialize_with_options(
            &mut self,
            bytes: &[u8],
            options: prieto_buffers::SerializeOptions,
        ) {
            let mut offset: u32 = 0;
            let mut counter: u8 = 0;
            let field_count = bytes[offset as usize];
            offset += 1;
            for _ in 0..field_count {
                let field_header = bytes[offset as usize];
                offset += 1;
                let field_id = field_header & 0b00011111;
                let field_type = prieto_buffers::FieldType::from_u8(field_header >> 5)
                    .unwrap();
                let field_size = match field_id {
                    0u8 => {
                        if self.a.get_type() == field_type {
                            self.a
                                .deserialize_with_options(
                                    &bytes[offset as usize..],
                                    options,
                                );
                            self.a.get_size()
                        } else {
                            let field_size = InnerTestStruct::skip_field(
                                &bytes[offset as usize..],
                                field_type,
                            );
                            {
                                ::std::io::_eprint(
                                    format_args!(
                                        "skip field {0} with type {1:?} and size {2}\n",
                                        field_id,
                                        field_type,
                                        field_size,
                                    ),
                                );
                            };
                            field_size
                        }
                    }
                    _ => {
                        InnerTestStruct::skip_field(
                            &bytes[offset as usize..],
                            field_type,
                        )
                    }
                };
                offset += field_size;
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for InnerTestStruct {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for InnerTestStruct {
        #[inline]
        fn eq(&self, other: &InnerTestStruct) -> bool {
            self.a == other.a
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for InnerTestStruct {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "InnerTestStruct",
                "a",
                &&self.a,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for InnerTestStruct {
        #[inline]
        fn default() -> InnerTestStruct {
            InnerTestStruct {
                a: ::core::default::Default::default(),
            }
        }
    }
    struct TestStruct {
        b: Vec<InnerTestStruct>,
        c: u8,
    }
    impl TestStruct {
        pub fn skip_field(bytes: &[u8], field_type: prieto_buffers::FieldType) -> u32 {
            match field_type {
                prieto_buffers::FieldType::Struct => {
                    let field_count = bytes[0] as u32;
                    let mut offset: u32 = 1;
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "Skipping struct with {0} fields\n",
                                field_count,
                            ),
                        );
                    };
                    for _ in 0..field_count {
                        let field_header = bytes[offset as usize];
                        offset += 1;
                        let field_type = prieto_buffers::FieldType::from_u8(
                                field_header >> 5,
                            )
                            .unwrap();
                        offset
                            += TestStruct::skip_field(
                                &bytes[offset as usize..],
                                field_type,
                            );
                    }
                    offset
                }
                prieto_buffers::FieldType::Array => {
                    let mut size: u32 = 0;
                    size.deserialize(&bytes);
                    let mut offset: u32 = size_of::<u32>() as u32;
                    let field_header = bytes[offset as usize];
                    let field_type = prieto_buffers::FieldType::from_u8(field_header)
                        .unwrap();
                    offset += 1;
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "Skipping array of type {0:?} with size {1}\n",
                                field_type,
                                size,
                            ),
                        );
                    };
                    for _ in 0..size {
                        offset
                            += TestStruct::skip_field(
                                &bytes[offset as usize..],
                                field_type,
                            );
                    }
                    offset
                }
                _ => {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "Skipping field of type {0:?} with size {1}\n",
                                field_type,
                                field_type.get_size(),
                            ),
                        );
                    };
                    field_type.get_size() as u32
                }
            }
        }
    }
    impl PrietoBuffersSerde for TestStruct {
        fn get_size_with_options(
            &self,
            options: prieto_buffers::SerializeOptions,
        ) -> u32 {
            let mut size = 1;
            if self.b.should_serialize() {
                size += self.b.get_size_with_options(options) + 1;
            }
            if self.c.should_serialize() {
                size += self.c.get_size_with_options(options) + 1;
            }
            size
        }
        fn get_type(&self) -> prieto_buffers::FieldType {
            prieto_buffers::FieldType::Struct
        }
        fn serialize_with_options(
            &self,
            bytes: &mut [u8],
            options: prieto_buffers::SerializeOptions,
        ) {
            let mut offset: u32 = 0;
            let mut field_amount: u8 = 0;
            if self.b.should_serialize() {
                field_amount += 1;
            }
            if self.c.should_serialize() {
                field_amount += 1;
            }
            bytes[offset as usize] = field_amount;
            offset += 1;
            {
                let mut options = options.clone();
                if false {
                    options.is_zero_ended_string = true;
                }
                if self.b.should_serialize() {
                    self.b
                        .serialize_with_header(
                            0u8,
                            &mut bytes[offset as usize..],
                            Some(options),
                        );
                    offset += self.b.get_size() + 1;
                }
            }
            {
                let mut options = options.clone();
                if false {
                    options.is_zero_ended_string = true;
                }
                if self.c.should_serialize() {
                    self.c
                        .serialize_with_header(
                            1u8,
                            &mut bytes[offset as usize..],
                            Some(options),
                        );
                    offset += self.c.get_size() + 1;
                }
            }
        }
        fn deserialize_with_options(
            &mut self,
            bytes: &[u8],
            options: prieto_buffers::SerializeOptions,
        ) {
            let mut offset: u32 = 0;
            let mut counter: u8 = 0;
            let field_count = bytes[offset as usize];
            offset += 1;
            for _ in 0..field_count {
                let field_header = bytes[offset as usize];
                offset += 1;
                let field_id = field_header & 0b00011111;
                let field_type = prieto_buffers::FieldType::from_u8(field_header >> 5)
                    .unwrap();
                let field_size = match field_id {
                    0u8 => {
                        if self.b.get_type() == field_type {
                            self.b
                                .deserialize_with_options(
                                    &bytes[offset as usize..],
                                    options,
                                );
                            self.b.get_size()
                        } else {
                            let field_size = TestStruct::skip_field(
                                &bytes[offset as usize..],
                                field_type,
                            );
                            {
                                ::std::io::_eprint(
                                    format_args!(
                                        "skip field {0} with type {1:?} and size {2}\n",
                                        field_id,
                                        field_type,
                                        field_size,
                                    ),
                                );
                            };
                            field_size
                        }
                    }
                    1u8 => {
                        if self.c.get_type() == field_type {
                            self.c
                                .deserialize_with_options(
                                    &bytes[offset as usize..],
                                    options,
                                );
                            self.c.get_size()
                        } else {
                            let field_size = TestStruct::skip_field(
                                &bytes[offset as usize..],
                                field_type,
                            );
                            {
                                ::std::io::_eprint(
                                    format_args!(
                                        "skip field {0} with type {1:?} and size {2}\n",
                                        field_id,
                                        field_type,
                                        field_size,
                                    ),
                                );
                            };
                            field_size
                        }
                    }
                    _ => TestStruct::skip_field(&bytes[offset as usize..], field_type),
                };
                offset += field_size;
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TestStruct {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TestStruct {
        #[inline]
        fn eq(&self, other: &TestStruct) -> bool {
            self.c == other.c && self.b == other.b
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TestStruct {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "TestStruct",
                "b",
                &self.b,
                "c",
                &&self.c,
            )
        }
    }
    struct TestStructCompatible {
        b: u8,
        c: u8,
    }
    impl TestStructCompatible {
        pub fn skip_field(bytes: &[u8], field_type: prieto_buffers::FieldType) -> u32 {
            match field_type {
                prieto_buffers::FieldType::Struct => {
                    let field_count = bytes[0] as u32;
                    let mut offset: u32 = 1;
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "Skipping struct with {0} fields\n",
                                field_count,
                            ),
                        );
                    };
                    for _ in 0..field_count {
                        let field_header = bytes[offset as usize];
                        offset += 1;
                        let field_type = prieto_buffers::FieldType::from_u8(
                                field_header >> 5,
                            )
                            .unwrap();
                        offset
                            += TestStructCompatible::skip_field(
                                &bytes[offset as usize..],
                                field_type,
                            );
                    }
                    offset
                }
                prieto_buffers::FieldType::Array => {
                    let mut size: u32 = 0;
                    size.deserialize(&bytes);
                    let mut offset: u32 = size_of::<u32>() as u32;
                    let field_header = bytes[offset as usize];
                    let field_type = prieto_buffers::FieldType::from_u8(field_header)
                        .unwrap();
                    offset += 1;
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "Skipping array of type {0:?} with size {1}\n",
                                field_type,
                                size,
                            ),
                        );
                    };
                    for _ in 0..size {
                        offset
                            += TestStructCompatible::skip_field(
                                &bytes[offset as usize..],
                                field_type,
                            );
                    }
                    offset
                }
                _ => {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "Skipping field of type {0:?} with size {1}\n",
                                field_type,
                                field_type.get_size(),
                            ),
                        );
                    };
                    field_type.get_size() as u32
                }
            }
        }
    }
    impl PrietoBuffersSerde for TestStructCompatible {
        fn get_size_with_options(
            &self,
            options: prieto_buffers::SerializeOptions,
        ) -> u32 {
            let mut size = 1;
            if self.b.should_serialize() {
                size += self.b.get_size_with_options(options) + 1;
            }
            if self.c.should_serialize() {
                size += self.c.get_size_with_options(options) + 1;
            }
            size
        }
        fn get_type(&self) -> prieto_buffers::FieldType {
            prieto_buffers::FieldType::Struct
        }
        fn serialize_with_options(
            &self,
            bytes: &mut [u8],
            options: prieto_buffers::SerializeOptions,
        ) {
            let mut offset: u32 = 0;
            let mut field_amount: u8 = 0;
            if self.b.should_serialize() {
                field_amount += 1;
            }
            if self.c.should_serialize() {
                field_amount += 1;
            }
            bytes[offset as usize] = field_amount;
            offset += 1;
            {
                let mut options = options.clone();
                if false {
                    options.is_zero_ended_string = true;
                }
                if self.b.should_serialize() {
                    self.b
                        .serialize_with_header(
                            0u8,
                            &mut bytes[offset as usize..],
                            Some(options),
                        );
                    offset += self.b.get_size() + 1;
                }
            }
            {
                let mut options = options.clone();
                if false {
                    options.is_zero_ended_string = true;
                }
                if self.c.should_serialize() {
                    self.c
                        .serialize_with_header(
                            1u8,
                            &mut bytes[offset as usize..],
                            Some(options),
                        );
                    offset += self.c.get_size() + 1;
                }
            }
        }
        fn deserialize_with_options(
            &mut self,
            bytes: &[u8],
            options: prieto_buffers::SerializeOptions,
        ) {
            let mut offset: u32 = 0;
            let mut counter: u8 = 0;
            let field_count = bytes[offset as usize];
            offset += 1;
            for _ in 0..field_count {
                let field_header = bytes[offset as usize];
                offset += 1;
                let field_id = field_header & 0b00011111;
                let field_type = prieto_buffers::FieldType::from_u8(field_header >> 5)
                    .unwrap();
                let field_size = match field_id {
                    0u8 => {
                        if self.b.get_type() == field_type {
                            self.b
                                .deserialize_with_options(
                                    &bytes[offset as usize..],
                                    options,
                                );
                            self.b.get_size()
                        } else {
                            let field_size = TestStructCompatible::skip_field(
                                &bytes[offset as usize..],
                                field_type,
                            );
                            {
                                ::std::io::_eprint(
                                    format_args!(
                                        "skip field {0} with type {1:?} and size {2}\n",
                                        field_id,
                                        field_type,
                                        field_size,
                                    ),
                                );
                            };
                            field_size
                        }
                    }
                    1u8 => {
                        if self.c.get_type() == field_type {
                            self.c
                                .deserialize_with_options(
                                    &bytes[offset as usize..],
                                    options,
                                );
                            self.c.get_size()
                        } else {
                            let field_size = TestStructCompatible::skip_field(
                                &bytes[offset as usize..],
                                field_type,
                            );
                            {
                                ::std::io::_eprint(
                                    format_args!(
                                        "skip field {0} with type {1:?} and size {2}\n",
                                        field_id,
                                        field_type,
                                        field_size,
                                    ),
                                );
                            };
                            field_size
                        }
                    }
                    _ => {
                        TestStructCompatible::skip_field(
                            &bytes[offset as usize..],
                            field_type,
                        )
                    }
                };
                offset += field_size;
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TestStructCompatible {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TestStructCompatible {
        #[inline]
        fn eq(&self, other: &TestStructCompatible) -> bool {
            self.b == other.b && self.c == other.c
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TestStructCompatible {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "TestStructCompatible",
                "b",
                &self.b,
                "c",
                &&self.c,
            )
        }
    }
    let mut rng = rand::rng();
    let a: TestStruct = TestStruct {
        b: <[_]>::into_vec(
            ::alloc::boxed::box_new([
                InnerTestStruct {
                    a: <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            rng.random(),
                            rng.random(),
                            rng.random(),
                        ]),
                    ),
                },
                InnerTestStruct {
                    a: <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            rng.random(),
                            rng.random(),
                            rng.random(),
                        ]),
                    ),
                },
                InnerTestStruct {
                    a: <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            rng.random(),
                            rng.random(),
                            rng.random(),
                        ]),
                    ),
                },
            ]),
        ),
        c: rng.random(),
    };
    let mut b: TestStruct = TestStruct {
        b: <[_]>::into_vec(
            ::alloc::boxed::box_new([
                InnerTestStruct {
                    a: <[_]>::into_vec(::alloc::boxed::box_new([0, 0, 0])),
                },
                InnerTestStruct {
                    a: <[_]>::into_vec(::alloc::boxed::box_new([0, 0, 0])),
                },
                InnerTestStruct {
                    a: <[_]>::into_vec(::alloc::boxed::box_new([0, 0, 0])),
                },
            ]),
        ),
        c: 0,
    };
    let mut c: TestStructCompatible = TestStructCompatible { b: 0, c: 0 };
    let size = a.get_size();
    let mut output = Vec::new();
    output.resize(size as usize, 0);
    a.serialize(output.as_mut_slice());
    b.deserialize(output.as_slice());
    c.deserialize(output.as_slice());
    match (&a, &b) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    {
        ::std::io::_eprint(format_args!("A: {0:?}\n", a));
    };
    {
        ::std::io::_eprint(format_args!("C: {0:?}\n", c));
    };
    {
        ::std::io::_eprint(format_args!("output: {0:?}\n", output));
    };
    match (&a.c, &c.c) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
