# dai（岱）

`dai` 是一款方便进行推送测试的 cli 工具。

* 支持多APP配置
* 支持多通道推送

目前已支持:

 * [x] huawei
 * [x] honor
 * [x] xiaomi
 * [ ] oppo
 * [ ] vivo
 * [ ] meizu
 * [ ] APNs
 * [ ] FCM

> Rust练习项目.....

## 安装

本地安装 `Rust` 环境， 拉取代码

```shell
cargo build --release
```

`target/release/dai` 即cli工具。

## 使用方式

* `dai app`
* `dai channel`
* `dai push`

### app命令

初始化应用及通道配置文件， 配置文件目录为 `~/.dai/config.toml`

```shell
dai app -i   # 初始化应用配置通道配置文件, 重复执行会覆盖历史文件
```

配置文件采用`toml`格式

```toml
# 当前激活的应用
active = "app0"

## 应用1
[configs.app0]
honor = { app_id = 0, client_id = "", client_secret = "" }
huawei = { app_id = 0, client_id = "", client_secret = "" }
xiaomi = { app_secret = "" }

## 应用2
[configs.app1]
honor = { app_id = 0, client_id = "", client_secret = "" }
huawei = { app_id = 0, client_id = "", client_secret = "" }
xiaomi = { app_secret = "" }
```

### channel 命令

为了最大定制化请求参数，参数采用`json`文件。使用 `channel` 命令生成参数模版。
模版生成路径为命令执行时的路径。

```shell
dai channel -i huawei      # 生成huawei通道请求参数
```

### push 命令

指定通道下发推送消息。


```shell
dai push huawei     # 如果不指定文件路径(-f) 默认读取当前路径下的huawei.json 文件

dai push huawei -f ~/huawei.json   # 指定文件推送消息
```

## 实现介绍

### 小米通道


小米推送详细请求参数可参考[小米推送单条推送接口](https://dev.mi.com/console/doc/detail?pId=1163#_0_0)

目前支持支持以下接口。

*V3接口-向某个regid或一组regid列表推送某条消息*
```
https://api.xmpush.xiaomi.com/v3/message/regid
```

*V3接口-向某个alias或一组alias列表推送某条消息*
```shell
https://api.xmpush.xiaomi.com/v3/message/alias

```
*V2接口-向某个account或一组account列表推送某条消息*

```shell
https://api.xmpush.xiaomi.com/v2/message/user_account
```

