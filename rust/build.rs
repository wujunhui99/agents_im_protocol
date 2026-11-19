const PROTOS: &[&str] = &[
    "../proto/admin/v1/admin.proto",
    "../proto/agent/v1/agent.proto",
    "../proto/auth/v1/auth.proto",
    "../proto/friends/v1/friends.proto",
    "../proto/gateway/v1/gateway.proto",
    "../proto/groups/v1/groups.proto",
    "../proto/mail/v1/mail.proto",
    "../proto/media/v1/media.proto",
    "../proto/msg/v1/msg.proto",
    "../proto/user/v1/user.proto",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_protos(PROTOS, &["../proto"])?;

    println!("cargo:rerun-if-changed=../proto");
    Ok(())
}
