extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src/types")
         // TODO: automate
        .inputs(&[
            "api/networking/v1beta1/destination_rule.proto",
            "api/networking/v1beta1/gateway.proto",
            "api/networking/v1beta1/proxy_config.proto",
            "api/networking/v1beta1/service_entry.proto",
            "api/networking/v1beta1/sidecar.proto",
            "api/networking/v1beta1/virtual_service.proto",
            "api/networking/v1beta1/workload_entry.proto",
            "api/networking/v1beta1/workload_group.proto",

            "api/type/v1beta1/selector.proto",
        ])
        .include("api")
        .run()
        .expect("Running protoc failed.");
}
