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
//! Generated file from `networking/v1beta1/service_entry.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_27_1;

#[derive(PartialEq,Clone,Default)]
pub struct ServiceEntry {
    // message fields
    pub hosts: ::protobuf::RepeatedField<::std::string::String>,
    pub addresses: ::protobuf::RepeatedField<::std::string::String>,
    pub ports: ::protobuf::RepeatedField<super::gateway::Port>,
    pub location: ServiceEntry_Location,
    pub resolution: ServiceEntry_Resolution,
    pub endpoints: ::protobuf::RepeatedField<super::workload_entry::WorkloadEntry>,
    pub workload_selector: ::protobuf::SingularPtrField<super::sidecar::WorkloadSelector>,
    pub export_to: ::protobuf::RepeatedField<::std::string::String>,
    pub subject_alt_names: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ServiceEntry {
    fn default() -> &'a ServiceEntry {
        <ServiceEntry as ::protobuf::Message>::default_instance()
    }
}

impl ServiceEntry {
    pub fn new() -> ServiceEntry {
        ::std::default::Default::default()
    }

    // repeated string hosts = 1;


    pub fn get_hosts(&self) -> &[::std::string::String] {
        &self.hosts
    }
    pub fn clear_hosts(&mut self) {
        self.hosts.clear();
    }

    // Param is passed by value, moved
    pub fn set_hosts(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.hosts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_hosts(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.hosts
    }

    // Take field
    pub fn take_hosts(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.hosts, ::protobuf::RepeatedField::new())
    }

    // repeated string addresses = 2;


    pub fn get_addresses(&self) -> &[::std::string::String] {
        &self.addresses
    }
    pub fn clear_addresses(&mut self) {
        self.addresses.clear();
    }

    // Param is passed by value, moved
    pub fn set_addresses(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.addresses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_addresses(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.addresses
    }

    // Take field
    pub fn take_addresses(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.addresses, ::protobuf::RepeatedField::new())
    }

    // repeated .istio.networking.v1beta1.Port ports = 3;


    pub fn get_ports(&self) -> &[super::gateway::Port] {
        &self.ports
    }
    pub fn clear_ports(&mut self) {
        self.ports.clear();
    }

    // Param is passed by value, moved
    pub fn set_ports(&mut self, v: ::protobuf::RepeatedField<super::gateway::Port>) {
        self.ports = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ports(&mut self) -> &mut ::protobuf::RepeatedField<super::gateway::Port> {
        &mut self.ports
    }

    // Take field
    pub fn take_ports(&mut self) -> ::protobuf::RepeatedField<super::gateway::Port> {
        ::std::mem::replace(&mut self.ports, ::protobuf::RepeatedField::new())
    }

    // .istio.networking.v1beta1.ServiceEntry.Location location = 4;


    pub fn get_location(&self) -> ServiceEntry_Location {
        self.location
    }
    pub fn clear_location(&mut self) {
        self.location = ServiceEntry_Location::MESH_EXTERNAL;
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: ServiceEntry_Location) {
        self.location = v;
    }

    // .istio.networking.v1beta1.ServiceEntry.Resolution resolution = 5;


    pub fn get_resolution(&self) -> ServiceEntry_Resolution {
        self.resolution
    }
    pub fn clear_resolution(&mut self) {
        self.resolution = ServiceEntry_Resolution::NONE;
    }

    // Param is passed by value, moved
    pub fn set_resolution(&mut self, v: ServiceEntry_Resolution) {
        self.resolution = v;
    }

    // repeated .istio.networking.v1beta1.WorkloadEntry endpoints = 6;


    pub fn get_endpoints(&self) -> &[super::workload_entry::WorkloadEntry] {
        &self.endpoints
    }
    pub fn clear_endpoints(&mut self) {
        self.endpoints.clear();
    }

    // Param is passed by value, moved
    pub fn set_endpoints(&mut self, v: ::protobuf::RepeatedField<super::workload_entry::WorkloadEntry>) {
        self.endpoints = v;
    }

    // Mutable pointer to the field.
    pub fn mut_endpoints(&mut self) -> &mut ::protobuf::RepeatedField<super::workload_entry::WorkloadEntry> {
        &mut self.endpoints
    }

    // Take field
    pub fn take_endpoints(&mut self) -> ::protobuf::RepeatedField<super::workload_entry::WorkloadEntry> {
        ::std::mem::replace(&mut self.endpoints, ::protobuf::RepeatedField::new())
    }

    // .istio.networking.v1beta1.WorkloadSelector workload_selector = 9;


    pub fn get_workload_selector(&self) -> &super::sidecar::WorkloadSelector {
        self.workload_selector.as_ref().unwrap_or_else(|| <super::sidecar::WorkloadSelector as ::protobuf::Message>::default_instance())
    }
    pub fn clear_workload_selector(&mut self) {
        self.workload_selector.clear();
    }

    pub fn has_workload_selector(&self) -> bool {
        self.workload_selector.is_some()
    }

    // Param is passed by value, moved
    pub fn set_workload_selector(&mut self, v: super::sidecar::WorkloadSelector) {
        self.workload_selector = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_workload_selector(&mut self) -> &mut super::sidecar::WorkloadSelector {
        if self.workload_selector.is_none() {
            self.workload_selector.set_default();
        }
        self.workload_selector.as_mut().unwrap()
    }

    // Take field
    pub fn take_workload_selector(&mut self) -> super::sidecar::WorkloadSelector {
        self.workload_selector.take().unwrap_or_else(|| super::sidecar::WorkloadSelector::new())
    }

    // repeated string export_to = 7;


    pub fn get_export_to(&self) -> &[::std::string::String] {
        &self.export_to
    }
    pub fn clear_export_to(&mut self) {
        self.export_to.clear();
    }

    // Param is passed by value, moved
    pub fn set_export_to(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.export_to = v;
    }

    // Mutable pointer to the field.
    pub fn mut_export_to(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.export_to
    }

    // Take field
    pub fn take_export_to(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.export_to, ::protobuf::RepeatedField::new())
    }

    // repeated string subject_alt_names = 8;


    pub fn get_subject_alt_names(&self) -> &[::std::string::String] {
        &self.subject_alt_names
    }
    pub fn clear_subject_alt_names(&mut self) {
        self.subject_alt_names.clear();
    }

    // Param is passed by value, moved
    pub fn set_subject_alt_names(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.subject_alt_names = v;
    }

    // Mutable pointer to the field.
    pub fn mut_subject_alt_names(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.subject_alt_names
    }

    // Take field
    pub fn take_subject_alt_names(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.subject_alt_names, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for ServiceEntry {
    fn is_initialized(&self) -> bool {
        for v in &self.ports {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.endpoints {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.workload_selector {
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
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.hosts)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.addresses)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ports)?;
                },
                4 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.location, 4, &mut self.unknown_fields)?
                },
                5 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.resolution, 5, &mut self.unknown_fields)?
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.endpoints)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.workload_selector)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.export_to)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.subject_alt_names)?;
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
        for value in &self.hosts {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.addresses {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.ports {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.location != ServiceEntry_Location::MESH_EXTERNAL {
            my_size += ::protobuf::rt::enum_size(4, self.location);
        }
        if self.resolution != ServiceEntry_Resolution::NONE {
            my_size += ::protobuf::rt::enum_size(5, self.resolution);
        }
        for value in &self.endpoints {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.workload_selector.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.export_to {
            my_size += ::protobuf::rt::string_size(7, &value);
        };
        for value in &self.subject_alt_names {
            my_size += ::protobuf::rt::string_size(8, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.hosts {
            os.write_string(1, &v)?;
        };
        for v in &self.addresses {
            os.write_string(2, &v)?;
        };
        for v in &self.ports {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.location != ServiceEntry_Location::MESH_EXTERNAL {
            os.write_enum(4, ::protobuf::ProtobufEnum::value(&self.location))?;
        }
        if self.resolution != ServiceEntry_Resolution::NONE {
            os.write_enum(5, ::protobuf::ProtobufEnum::value(&self.resolution))?;
        }
        for v in &self.endpoints {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.workload_selector.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.export_to {
            os.write_string(7, &v)?;
        };
        for v in &self.subject_alt_names {
            os.write_string(8, &v)?;
        };
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

    fn new() -> ServiceEntry {
        ServiceEntry::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "hosts",
                |m: &ServiceEntry| { &m.hosts },
                |m: &mut ServiceEntry| { &mut m.hosts },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "addresses",
                |m: &ServiceEntry| { &m.addresses },
                |m: &mut ServiceEntry| { &mut m.addresses },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::gateway::Port>>(
                "ports",
                |m: &ServiceEntry| { &m.ports },
                |m: &mut ServiceEntry| { &mut m.ports },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ServiceEntry_Location>>(
                "location",
                |m: &ServiceEntry| { &m.location },
                |m: &mut ServiceEntry| { &mut m.location },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ServiceEntry_Resolution>>(
                "resolution",
                |m: &ServiceEntry| { &m.resolution },
                |m: &mut ServiceEntry| { &mut m.resolution },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::workload_entry::WorkloadEntry>>(
                "endpoints",
                |m: &ServiceEntry| { &m.endpoints },
                |m: &mut ServiceEntry| { &mut m.endpoints },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::sidecar::WorkloadSelector>>(
                "workload_selector",
                |m: &ServiceEntry| { &m.workload_selector },
                |m: &mut ServiceEntry| { &mut m.workload_selector },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "export_to",
                |m: &ServiceEntry| { &m.export_to },
                |m: &mut ServiceEntry| { &mut m.export_to },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "subject_alt_names",
                |m: &ServiceEntry| { &m.subject_alt_names },
                |m: &mut ServiceEntry| { &mut m.subject_alt_names },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ServiceEntry>(
                "ServiceEntry",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ServiceEntry {
        static instance: ::protobuf::rt::LazyV2<ServiceEntry> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ServiceEntry::new)
    }
}

impl ::protobuf::Clear for ServiceEntry {
    fn clear(&mut self) {
        self.hosts.clear();
        self.addresses.clear();
        self.ports.clear();
        self.location = ServiceEntry_Location::MESH_EXTERNAL;
        self.resolution = ServiceEntry_Resolution::NONE;
        self.endpoints.clear();
        self.workload_selector.clear();
        self.export_to.clear();
        self.subject_alt_names.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServiceEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServiceEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ServiceEntry_Location {
    MESH_EXTERNAL = 0,
    MESH_INTERNAL = 1,
}

impl ::protobuf::ProtobufEnum for ServiceEntry_Location {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ServiceEntry_Location> {
        match value {
            0 => ::std::option::Option::Some(ServiceEntry_Location::MESH_EXTERNAL),
            1 => ::std::option::Option::Some(ServiceEntry_Location::MESH_INTERNAL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ServiceEntry_Location] = &[
            ServiceEntry_Location::MESH_EXTERNAL,
            ServiceEntry_Location::MESH_INTERNAL,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<ServiceEntry_Location>("ServiceEntry.Location", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for ServiceEntry_Location {
}

impl ::std::default::Default for ServiceEntry_Location {
    fn default() -> Self {
        ServiceEntry_Location::MESH_EXTERNAL
    }
}

impl ::protobuf::reflect::ProtobufValue for ServiceEntry_Location {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ServiceEntry_Resolution {
    NONE = 0,
    STATIC = 1,
    DNS = 2,
    DNS_ROUND_ROBIN = 3,
}

impl ::protobuf::ProtobufEnum for ServiceEntry_Resolution {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ServiceEntry_Resolution> {
        match value {
            0 => ::std::option::Option::Some(ServiceEntry_Resolution::NONE),
            1 => ::std::option::Option::Some(ServiceEntry_Resolution::STATIC),
            2 => ::std::option::Option::Some(ServiceEntry_Resolution::DNS),
            3 => ::std::option::Option::Some(ServiceEntry_Resolution::DNS_ROUND_ROBIN),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ServiceEntry_Resolution] = &[
            ServiceEntry_Resolution::NONE,
            ServiceEntry_Resolution::STATIC,
            ServiceEntry_Resolution::DNS,
            ServiceEntry_Resolution::DNS_ROUND_ROBIN,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<ServiceEntry_Resolution>("ServiceEntry.Resolution", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for ServiceEntry_Resolution {
}

impl ::std::default::Default for ServiceEntry_Resolution {
    fn default() -> Self {
        ServiceEntry_Resolution::NONE
    }
}

impl ::protobuf::reflect::ProtobufValue for ServiceEntry_Resolution {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n&networking/v1beta1/service_entry.proto\x12\x18istio.networking.v1beta\
    1\x1a\x1fgoogle/api/field_behavior.proto\x1a\x20networking/v1beta1/gatew\
    ay.proto\x1a\x20networking/v1beta1/sidecar.proto\x1a'networking/v1beta1/\
    workload_entry.proto\"\x84\x05\n\x0cServiceEntry\x12\x19\n\x05hosts\x18\
    \x01\x20\x03(\tR\x05hostsB\x03\xe0A\x02\x12\x1c\n\taddresses\x18\x02\x20\
    \x03(\tR\taddresses\x129\n\x05ports\x18\x03\x20\x03(\x0b2\x1e.istio.netw\
    orking.v1beta1.PortR\x05portsB\x03\xe0A\x02\x12K\n\x08location\x18\x04\
    \x20\x01(\x0e2/.istio.networking.v1beta1.ServiceEntry.LocationR\x08locat\
    ion\x12V\n\nresolution\x18\x05\x20\x01(\x0e21.istio.networking.v1beta1.S\
    erviceEntry.ResolutionR\nresolutionB\x03\xe0A\x02\x12E\n\tendpoints\x18\
    \x06\x20\x03(\x0b2'.istio.networking.v1beta1.WorkloadEntryR\tendpoints\
    \x12W\n\x11workload_selector\x18\t\x20\x01(\x0b2*.istio.networking.v1bet\
    a1.WorkloadSelectorR\x10workloadSelector\x12\x1b\n\texport_to\x18\x07\
    \x20\x03(\tR\x08exportTo\x12*\n\x11subject_alt_names\x18\x08\x20\x03(\tR\
    \x0fsubjectAltNames\"0\n\x08Location\x12\x11\n\rMESH_EXTERNAL\x10\0\x12\
    \x11\n\rMESH_INTERNAL\x10\x01\"@\n\nResolution\x12\x08\n\x04NONE\x10\0\
    \x12\n\n\x06STATIC\x10\x01\x12\x07\n\x03DNS\x10\x02\x12\x13\n\x0fDNS_ROU\
    ND_ROBIN\x10\x03B!Z\x1fistio.io/api/networking/v1beta1b\x06proto3\
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