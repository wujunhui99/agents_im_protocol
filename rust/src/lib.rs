//! agents_im gRPC 契约的 Rust 绑定。
//!
//! 所有模块由 `proto/` 在构建期生成（见 build.rs），与 Go 侧共用同一份 proto。
//! 服务名、字段、包版本（`*.v1`）保持一致；不兼容变更会进新的包版本。

pub mod admin {
    tonic::include_proto!("admin.v1");
}
pub mod agent {
    tonic::include_proto!("agent.v1");
}
pub mod auth {
    tonic::include_proto!("auth.v1");
}
pub mod friends {
    tonic::include_proto!("friends.v1");
}
pub mod gateway {
    tonic::include_proto!("gateway.v1");
}
pub mod groups {
    tonic::include_proto!("groups.v1");
}
pub mod mail {
    tonic::include_proto!("mail.v1");
}
pub mod media {
    tonic::include_proto!("media.v1");
}
pub mod msg {
    tonic::include_proto!("msg.v1");
}
pub mod user {
    tonic::include_proto!("user.v1");
}
