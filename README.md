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
