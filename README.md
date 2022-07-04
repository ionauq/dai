# dai（岱）
A cli tool for transmit notification to manufacturer push notification service.

now support:
 * [x] honor
 * [ ] huawei
 * [ ] xiaomi
 * [ ] APNs
 * [ ] oppo
 * [ ] vivo
 * [ ] meizu
 * [ ] FCM

> 首次使用Rust解决需求，主要为了练习。

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

## 应用2
[configs.app1]
honor = { app_id = 0, client_id = "", client_secret = "" }
huawei = { app_id = 0, client_id = "", client_secret = "" }
```

### channel 命令

为了最大定制化请求参数，参数存储采用`json`文件方式。使用 `channel` 命令生成参数模版。目前路径为当前路径。

```shell
dai channel -i honor      # 生成honor通道请求参数
```


### push 命令

指定通道下发推送消息。


```shell
dai push honor     # 如果不指定文件路径(-f) 默认读取当前路径下的honor.json 文件
```