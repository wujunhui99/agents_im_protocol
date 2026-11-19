# agents_im_protocol

agents_im 的**契约仓库**：只放 protobuf 定义与生成代码，不放任何业务实现。
参考 [openimsdk/protocol](https://github.com/openimsdk/protocol) 的组织方式，把
「接口契约」从「服务实现」中彻底分离。

## 为什么单独一个仓库

- **契约先行**：改接口 = 改这个仓库 + 发版本；实现仓库通过 `go get` 升级，评审时 diff 只含接口变化。
- **多仓库复用**：服务、SDK、测试夹具、未来的网关/移动端共用同一份定义。
- **破坏性变更可见**：proto 包名即版本（`user.v1`），不兼容改动进 `v2` 包，旧客户端继续可用。
- **生成代码随契约走**：消费方不需要装 protoc，`go build` 即可。

## 目录

```
proto/<domain>/v1/*.proto     # 源定义
<domain>/v1/*.pb.go           # 生成代码（提交进仓库）
go.mod                        # module github.com/wujunhui99/agents_im_protocol
```

| domain | proto package | 使用方 |
|---|---|---|
| user | `user.v1` | user-rpc |
| auth | `auth.v1` | auth-rpc |
| friends | `friends.v1` | friends-rpc |
| groups | `groups.v1` | groups-rpc |
| msg | `msg.v1` | msg-rpc |
| agent | `agent.v1` | agent-rpc |
| admin | `admin.v1` | admin-rpc |
| media | `media.v1` | resource-rpc（media 能力） |
| mail | `mail.v1` | resource-rpc（mail 能力） |
| gateway | `gateway.v1` | msggateway（下行推送 gRPC 面） |

注意：`media` / `mail` 是 resource-rpc 一个进程里的两个 gRPC service，契约上仍保持
两个独立包，避免把"外部资源管理"耦合成一个大 proto。

## 变更流程

1. 改 `proto/`，`make gen` 重新生成。
2. 提交 PR（CI 会检查生成代码是否与 proto 一致）。
3. 合并后打 tag：`v0.2.0`。
4. 消费方 `go get github.com/wujunhui99/agents_im_protocol@v0.2.0`。

不兼容变更的判定：

- 删除 / 重命名 message 或 field、改 field 类型、改 field number → **不兼容**，进新的包版本（如 `user/v2`）。
- 新增 message、新增 field、新增 rpc → 兼容，升 minor。

## 本地联调

实现仓库在 `go.mod` 里可以用 replace 指向本地路径：

```
replace github.com/wujunhui99/agents_im_protocol => ../agents_im_protocol
```

提交前去掉 replace，改用 tag 版本，避免把本地路径带进 CI。

## 生成

需要 `protoc`、`protoc-gen-go`、`protoc-gen-go-grpc`：

```bash
make gen      # 重新生成并 go mod tidy
make verify   # CI 用：重新生成后要求 git 无 diff
```

## 多语言（Rust 及其他）

契约是语言中立的：`proto/` 是唯一事实来源，`option go_package` 只对 Go 生效，其他语言
生成器会忽略它。当前仓库已提供两套绑定：

| 语言 | 位置 | 生成方式 | 消费方式 |
|---|---|---|---|
| Go | `<domain>/v1/*.pb.go`（提交进仓库） | `make gen`（protoc） | `go get github.com/wujunhui99/agents_im_protocol@vX.Y.Z` |
| Rust | `rust/`（crate `agents-im-protocol`） | `build.rs` 里 `tonic-build` 编译期生成 | git 依赖 + tag，或后续发布到 crates.io |

Rust 侧验证：

```bash
cd rust && cargo check
```

新增一门语言（TypeScript / Java / Python …）不影响现有绑定：加一个 `gen/<lang>/` 目录
或独立 crate/package，在 CI 里加一条生成 + 编译检查即可。契约本身不需要改动。

### 版本与兼容

- 版本用 git tag（`v0.2.0`）表达，Go module 和 Cargo git 依赖都能直接用同一个 tag。
- proto 包名带版本（`user.v1`）：不兼容改动进 `user/v2`，老客户端继续用 v1。
- 兼容改动（加字段/加 rpc）升 minor；不兼容升新的包版本，仓库整体 tag 只做递增。

### Rust 侧注意事项

- 字段命名：Rust 生成 snake_case，proto 用 lower_snake_case 即可，不要依赖 JSON 名。
- 已知类型：`google.protobuf.Struct` 映射为 `prost_types::Struct`（`agent.proto` 用到）。
- 服务需要双向流时直接加 `stream`，tonic/prost 都支持；当前契约均为 unary。
- `build.rs` 依赖本机 `protoc`。如果希望消费方零工具链，可改为把生成的 `.rs` 提交进
  `rust/src/gen/`，与 Go 侧保持一致。

### 可选：Buf

如果后续要更强的 lint / breaking-change 检测，可引入 [Buf](https://buf.build)：
`buf breaking --against '.git#tag=vX.Y.Z'` 能在 CI 里拦住不兼容改动。当前先用
`make verify`（生成代码漂移检查）+ code review 兜底。

## 工具链版本（生成代码必须与之一致）

| 工具 | 版本 |
|---|---|
| protoc | 36.2 |
| protoc-gen-go | v1.36.11 |
| protoc-gen-go-grpc | v1.6.1 |

版本不一致会导致生成文件头部的版本注释变化，CI 的 `make verify` 会报漂移。
