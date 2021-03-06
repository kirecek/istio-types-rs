// This file is generated by rust-protobuf 2.27.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `networking/v1beta1/proxy_config.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_27_1;

#[derive(PartialEq,Clone,Default)]
pub struct ProxyConfig {
    // message fields
    pub selector: ::protobuf::SingularPtrField<super::selector::WorkloadSelector>,
    pub concurrency: ::protobuf::SingularPtrField<::protobuf::well_known_types::Int32Value>,
    pub environment_variables: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    pub image: ::protobuf::SingularPtrField<ProxyImage>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ProxyConfig {
    fn default() -> &'a ProxyConfig {
        <ProxyConfig as ::protobuf::Message>::default_instance()
    }
}

impl ProxyConfig {
    pub fn new() -> ProxyConfig {
        ::std::default::Default::default()
    }

    // .istio.type.v1beta1.WorkloadSelector selector = 1;


    pub fn get_selector(&self) -> &super::selector::WorkloadSelector {
        self.selector.as_ref().unwrap_or_else(|| <super::selector::WorkloadSelector as ::protobuf::Message>::default_instance())
    }
    pub fn clear_selector(&mut self) {
        self.selector.clear();
    }

    pub fn has_selector(&self) -> bool {
        self.selector.is_some()
    }

    // Param is passed by value, moved
    pub fn set_selector(&mut self, v: super::selector::WorkloadSelector) {
        self.selector = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_selector(&mut self) -> &mut super::selector::WorkloadSelector {
        if self.selector.is_none() {
            self.selector.set_default();
        }
        self.selector.as_mut().unwrap()
    }

    // Take field
    pub fn take_selector(&mut self) -> super::selector::WorkloadSelector {
        self.selector.take().unwrap_or_else(|| super::selector::WorkloadSelector::new())
    }

    // .google.protobuf.Int32Value concurrency = 2;


    pub fn get_concurrency(&self) -> &::protobuf::well_known_types::Int32Value {
        self.concurrency.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Int32Value as ::protobuf::Message>::default_instance())
    }
    pub fn clear_concurrency(&mut self) {
        self.concurrency.clear();
    }

    pub fn has_concurrency(&self) -> bool {
        self.concurrency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_concurrency(&mut self, v: ::protobuf::well_known_types::Int32Value) {
        self.concurrency = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_concurrency(&mut self) -> &mut ::protobuf::well_known_types::Int32Value {
        if self.concurrency.is_none() {
            self.concurrency.set_default();
        }
        self.concurrency.as_mut().unwrap()
    }

    // Take field
    pub fn take_concurrency(&mut self) -> ::protobuf::well_known_types::Int32Value {
        self.concurrency.take().unwrap_or_else(|| ::protobuf::well_known_types::Int32Value::new())
    }

    // repeated .istio.networking.v1beta1.ProxyConfig.EnvironmentVariablesEntry environment_variables = 3;


    pub fn get_environment_variables(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.environment_variables
    }
    pub fn clear_environment_variables(&mut self) {
        self.environment_variables.clear();
    }

    // Param is passed by value, moved
    pub fn set_environment_variables(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.environment_variables = v;
    }

    // Mutable pointer to the field.
    pub fn mut_environment_variables(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.environment_variables
    }

    // Take field
    pub fn take_environment_variables(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.environment_variables, ::std::collections::HashMap::new())
    }

    // .istio.networking.v1beta1.ProxyImage image = 4;


    pub fn get_image(&self) -> &ProxyImage {
        self.image.as_ref().unwrap_or_else(|| <ProxyImage as ::protobuf::Message>::default_instance())
    }
    pub fn clear_image(&mut self) {
        self.image.clear();
    }

    pub fn has_image(&self) -> bool {
        self.image.is_some()
    }

    // Param is passed by value, moved
    pub fn set_image(&mut self, v: ProxyImage) {
        self.image = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_image(&mut self) -> &mut ProxyImage {
        if self.image.is_none() {
            self.image.set_default();
        }
        self.image.as_mut().unwrap()
    }

    // Take field
    pub fn take_image(&mut self) -> ProxyImage {
        self.image.take().unwrap_or_else(|| ProxyImage::new())
    }
}

impl ::protobuf::Message for ProxyConfig {
    fn is_initialized(&self) -> bool {
        for v in &self.selector {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.concurrency {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.image {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.selector)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.concurrency)?;
                },
                3 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.environment_variables)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.image)?;
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
        if let Some(ref v) = self.selector.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.concurrency.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(3, &self.environment_variables);
        if let Some(ref v) = self.image.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.selector.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.concurrency.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(3, &self.environment_variables, os)?;
        if let Some(ref v) = self.image.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn new() -> ProxyConfig {
        ProxyConfig::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::selector::WorkloadSelector>>(
                "selector",
                |m: &ProxyConfig| { &m.selector },
                |m: &mut ProxyConfig| { &mut m.selector },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Int32Value>>(
                "concurrency",
                |m: &ProxyConfig| { &m.concurrency },
                |m: &mut ProxyConfig| { &mut m.concurrency },
            ));
            fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                "environment_variables",
                |m: &ProxyConfig| { &m.environment_variables },
                |m: &mut ProxyConfig| { &mut m.environment_variables },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ProxyImage>>(
                "image",
                |m: &ProxyConfig| { &m.image },
                |m: &mut ProxyConfig| { &mut m.image },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ProxyConfig>(
                "ProxyConfig",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ProxyConfig {
        static instance: ::protobuf::rt::LazyV2<ProxyConfig> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ProxyConfig::new)
    }
}

impl ::protobuf::Clear for ProxyConfig {
    fn clear(&mut self) {
        self.selector.clear();
        self.concurrency.clear();
        self.environment_variables.clear();
        self.image.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProxyConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProxyConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ProxyImage {
    // message fields
    pub image_type: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ProxyImage {
    fn default() -> &'a ProxyImage {
        <ProxyImage as ::protobuf::Message>::default_instance()
    }
}

impl ProxyImage {
    pub fn new() -> ProxyImage {
        ::std::default::Default::default()
    }

    // string image_type = 1;


    pub fn get_image_type(&self) -> &str {
        &self.image_type
    }
    pub fn clear_image_type(&mut self) {
        self.image_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_image_type(&mut self, v: ::std::string::String) {
        self.image_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_image_type(&mut self) -> &mut ::std::string::String {
        &mut self.image_type
    }

    // Take field
    pub fn take_image_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.image_type, ::std::string::String::new())
    }
}

impl ::protobuf::Message for ProxyImage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.image_type)?;
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
        if !self.image_type.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.image_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.image_type.is_empty() {
            os.write_string(1, &self.image_type)?;
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

    fn new() -> ProxyImage {
        ProxyImage::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "image_type",
                |m: &ProxyImage| { &m.image_type },
                |m: &mut ProxyImage| { &mut m.image_type },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ProxyImage>(
                "ProxyImage",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ProxyImage {
        static instance: ::protobuf::rt::LazyV2<ProxyImage> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ProxyImage::new)
    }
}

impl ::protobuf::Clear for ProxyImage {
    fn clear(&mut self) {
        self.image_type.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProxyImage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProxyImage {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%networking/v1beta1/proxy_config.proto\x12\x18istio.networking.v1beta1\
    \x1a\x1egoogle/protobuf/wrappers.proto\x1a\x1btype/v1beta1/selector.prot\
    o\"\x89\x03\n\x0bProxyConfig\x12@\n\x08selector\x18\x01\x20\x01(\x0b2$.i\
    stio.type.v1beta1.WorkloadSelectorR\x08selector\x12=\n\x0bconcurrency\
    \x18\x02\x20\x01(\x0b2\x1b.google.protobuf.Int32ValueR\x0bconcurrency\
    \x12t\n\x15environment_variables\x18\x03\x20\x03(\x0b2?.istio.networking\
    .v1beta1.ProxyConfig.EnvironmentVariablesEntryR\x14environmentVariables\
    \x12:\n\x05image\x18\x04\x20\x01(\x0b2$.istio.networking.v1beta1.ProxyIm\
    ageR\x05image\x1aG\n\x19EnvironmentVariablesEntry\x12\x10\n\x03key\x18\
    \x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\tR\x05value:\
    \x028\x01\"+\n\nProxyImage\x12\x1d\n\nimage_type\x18\x01\x20\x01(\tR\tim\
    ageTypeB!Z\x1fistio.io/api/networking/v1beta1b\x06proto3\
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
