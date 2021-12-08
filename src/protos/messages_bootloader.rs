// This file is generated by rust-protobuf 2.18.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]


#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `messages-bootloader.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct FirmwareErase {
    // message fields
    length: ::std::option::Option<u32>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a FirmwareErase {
    fn default() -> &'a FirmwareErase {
        <FirmwareErase as ::protobuf::Message>::default_instance()
    }
}

impl FirmwareErase {
    pub fn new() -> FirmwareErase {
        ::std::default::Default::default()
    }

    // optional uint32 length = 1;


    pub fn get_length(&self) -> u32 {
        self.length.unwrap_or(0)
    }
    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u32) {
        self.length = ::std::option::Option::Some(v);
    }
}

impl ::protobuf::Message for FirmwareErase {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.length = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.length {
            os.write_uint32(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> FirmwareErase {
        FirmwareErase::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "length",
                |m: &FirmwareErase| { &m.length },
                |m: &mut FirmwareErase| { &mut m.length },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<FirmwareErase>(
                "FirmwareErase",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static FirmwareErase {
        static instance: ::protobuf::rt::LazyV2<FirmwareErase> = ::protobuf::rt::LazyV2::INIT;
        instance.get(FirmwareErase::new)
    }
}

impl ::protobuf::Clear for FirmwareErase {
    fn clear(&mut self) {
        self.length = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FirmwareErase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FirmwareErase {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FirmwareRequest {
    // message fields
    offset: ::std::option::Option<u32>,
    length: ::std::option::Option<u32>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a FirmwareRequest {
    fn default() -> &'a FirmwareRequest {
        <FirmwareRequest as ::protobuf::Message>::default_instance()
    }
}

impl FirmwareRequest {
    pub fn new() -> FirmwareRequest {
        ::std::default::Default::default()
    }

    // optional uint32 offset = 1;


    pub fn get_offset(&self) -> u32 {
        self.offset.unwrap_or(0)
    }
    pub fn clear_offset(&mut self) {
        self.offset = ::std::option::Option::None;
    }

    pub fn has_offset(&self) -> bool {
        self.offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_offset(&mut self, v: u32) {
        self.offset = ::std::option::Option::Some(v);
    }

    // optional uint32 length = 2;


    pub fn get_length(&self) -> u32 {
        self.length.unwrap_or(0)
    }
    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u32) {
        self.length = ::std::option::Option::Some(v);
    }
}

impl ::protobuf::Message for FirmwareRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.offset = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.length = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.offset {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.offset {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.length {
            os.write_uint32(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> FirmwareRequest {
        FirmwareRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "offset",
                |m: &FirmwareRequest| { &m.offset },
                |m: &mut FirmwareRequest| { &mut m.offset },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "length",
                |m: &FirmwareRequest| { &m.length },
                |m: &mut FirmwareRequest| { &mut m.length },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<FirmwareRequest>(
                "FirmwareRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static FirmwareRequest {
        static instance: ::protobuf::rt::LazyV2<FirmwareRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(FirmwareRequest::new)
    }
}

impl ::protobuf::Clear for FirmwareRequest {
    fn clear(&mut self) {
        self.offset = ::std::option::Option::None;
        self.length = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FirmwareRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FirmwareRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FirmwareUpload {
    // message fields
    payload: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    hash: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a FirmwareUpload {
    fn default() -> &'a FirmwareUpload {
        <FirmwareUpload as ::protobuf::Message>::default_instance()
    }
}

impl FirmwareUpload {
    pub fn new() -> FirmwareUpload {
        ::std::default::Default::default()
    }

    // required bytes payload = 1;


    pub fn get_payload(&self) -> &[u8] {
        match self.payload.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.payload.is_none() {
            self.payload.set_default();
        }
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        self.payload.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional bytes hash = 2;


    pub fn get_hash(&self) -> &[u8] {
        match self.hash.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    pub fn has_hash(&self) -> bool {
        self.hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.hash.is_none() {
            self.hash.set_default();
        }
        self.hash.as_mut().unwrap()
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        self.hash.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for FirmwareUpload {
    fn is_initialized(&self) -> bool {
        if self.payload.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.payload)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.hash)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.payload.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.hash.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.payload.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.hash.as_ref() {
            os.write_bytes(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> FirmwareUpload {
        FirmwareUpload::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "payload",
                |m: &FirmwareUpload| { &m.payload },
                |m: &mut FirmwareUpload| { &mut m.payload },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "hash",
                |m: &FirmwareUpload| { &m.hash },
                |m: &mut FirmwareUpload| { &mut m.hash },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<FirmwareUpload>(
                "FirmwareUpload",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static FirmwareUpload {
        static instance: ::protobuf::rt::LazyV2<FirmwareUpload> = ::protobuf::rt::LazyV2::INIT;
        instance.get(FirmwareUpload::new)
    }
}

impl ::protobuf::Clear for FirmwareUpload {
    fn clear(&mut self) {
        self.payload.clear();
        self.hash.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FirmwareUpload {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FirmwareUpload {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SelfTest {
    // message fields
    payload: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SelfTest {
    fn default() -> &'a SelfTest {
        <SelfTest as ::protobuf::Message>::default_instance()
    }
}

impl SelfTest {
    pub fn new() -> SelfTest {
        ::std::default::Default::default()
    }

    // optional bytes payload = 1;


    pub fn get_payload(&self) -> &[u8] {
        match self.payload.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.payload.is_none() {
            self.payload.set_default();
        }
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        self.payload.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for SelfTest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.payload)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.payload.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.payload.as_ref() {
            os.write_bytes(1, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SelfTest {
        SelfTest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "payload",
                |m: &SelfTest| { &m.payload },
                |m: &mut SelfTest| { &mut m.payload },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SelfTest>(
                "SelfTest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SelfTest {
        static instance: ::protobuf::rt::LazyV2<SelfTest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SelfTest::new)
    }
}

impl ::protobuf::Clear for SelfTest {
    fn clear(&mut self) {
        self.payload.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SelfTest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SelfTest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19messages-bootloader.proto\x12\x1dhw.trezor.messages.bootloader\"'\
    \n\rFirmwareErase\x12\x16\n\x06length\x18\x01\x20\x01(\rR\x06length\"A\n\
    \x0fFirmwareRequest\x12\x16\n\x06offset\x18\x01\x20\x01(\rR\x06offset\
    \x12\x16\n\x06length\x18\x02\x20\x01(\rR\x06length\">\n\x0eFirmwareUploa\
    d\x12\x18\n\x07payload\x18\x01\x20\x02(\x0cR\x07payload\x12\x12\n\x04has\
    h\x18\x02\x20\x01(\x0cR\x04hash\"$\n\x08SelfTest\x12\x18\n\x07payload\
    \x18\x01\x20\x01(\x0cR\x07payloadB>\n#com.satoshilabs.trezor.lib.protobu\
    fB\x17TrezorMessageBootloaderJ\x99\t\n\x06\x12\x04\0\0+\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08%\n\x08\n\x01\x08\
    \x12\x03\x04\0<\n.\n\x02\x08\x01\x12\x03\x04\0<\x1a#\x20Sugar\x20for\x20\
    easier\x20handling\x20in\x20Java\n\n\x08\n\x01\x08\x12\x03\x05\08\n\t\n\
    \x02\x08\x08\x12\x03\x05\08\n\x83\x01\n\x02\x04\0\x12\x04\x0c\0\x0e\x01\
    \x1aw*\n\x20Request:\x20Ask\x20device\x20to\x20erase\x20its\x20firmware\
    \x20(so\x20it\x20can\x20be\x20replaced\x20via\x20FirmwareUpload)\n\x20@s\
    tart\n\x20@next\x20FirmwareRequest\n\n\n\n\x03\x04\0\x01\x12\x03\x0c\x08\
    \x15\n%\n\x04\x04\0\x02\0\x12\x03\r\x04\x1f\"\x18\x20length\x20of\x20new\
    \x20firmware\n\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\r\x04\x0c\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\r\r\x13\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\r\
    \x14\x1a\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\r\x1d\x1e\nF\n\x02\x04\x01\
    \x12\x04\x14\0\x17\x01\x1a:*\n\x20Response:\x20Ask\x20for\x20firmware\
    \x20chunk\n\x20@next\x20FirmwareUpload\n\n\n\n\x03\x04\x01\x01\x12\x03\
    \x14\x08\x17\n1\n\x04\x04\x01\x02\0\x12\x03\x15\x04\x1f\"$\x20offset\x20\
    of\x20requested\x20firmware\x20chunk\n\n\x0c\n\x05\x04\x01\x02\0\x04\x12\
    \x03\x15\x04\x0c\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x15\r\x13\n\x0c\n\
    \x05\x04\x01\x02\0\x01\x12\x03\x15\x14\x1a\n\x0c\n\x05\x04\x01\x02\0\x03\
    \x12\x03\x15\x1d\x1e\n1\n\x04\x04\x01\x02\x01\x12\x03\x16\x04\x1f\"$\x20\
    length\x20of\x20requested\x20firmware\x20chunk\n\n\x0c\n\x05\x04\x01\x02\
    \x01\x04\x12\x03\x16\x04\x0c\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x16\
    \r\x13\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x16\x14\x1a\n\x0c\n\x05\
    \x04\x01\x02\x01\x03\x12\x03\x16\x1d\x1e\nx\n\x02\x04\x02\x12\x04\x1f\0\
    \"\x01\x1al*\n\x20Request:\x20Send\x20firmware\x20in\x20binary\x20form\
    \x20to\x20the\x20device\n\x20@next\x20FirmwareRequest\n\x20@next\x20Succ\
    ess\n\x20@next\x20Failure\n\n\n\n\x03\x04\x02\x01\x12\x03\x1f\x08\x16\n0\
    \n\x04\x04\x02\x02\0\x12\x03\x20\x04\x1f\"#\x20firmware\x20to\x20be\x20l\
    oaded\x20into\x20device\n\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03\x20\x04\
    \x0c\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x20\r\x12\n\x0c\n\x05\x04\x02\
    \x02\0\x01\x12\x03\x20\x13\x1a\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x20\
    \x1d\x1e\n\"\n\x04\x04\x02\x02\x01\x12\x03!\x04\x1c\"\x15\x20hash\x20of\
    \x20the\x20payload\n\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03!\x04\x0c\n\
    \x0c\n\x05\x04\x02\x02\x01\x05\x12\x03!\r\x12\n\x0c\n\x05\x04\x02\x02\
    \x01\x01\x12\x03!\x13\x17\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03!\x1a\
    \x1b\nQ\n\x02\x04\x03\x12\x04)\0+\x01\x1aE*\n\x20Request:\x20Perform\x20\
    a\x20device\x20self-test\n\x20@next\x20Success\n\x20@next\x20Failure\n\n\
    \n\n\x03\x04\x03\x01\x12\x03)\x08\x10\n.\n\x04\x04\x03\x02\0\x12\x03*\
    \x04\x1f\"!\x20payload\x20to\x20be\x20used\x20in\x20self-test\n\n\x0c\n\
    \x05\x04\x03\x02\0\x04\x12\x03*\x04\x0c\n\x0c\n\x05\x04\x03\x02\0\x05\
    \x12\x03*\r\x12\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03*\x13\x1a\n\x0c\n\
    \x05\x04\x03\x02\0\x03\x12\x03*\x1d\x1e\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
