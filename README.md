# dai（岱）

`dai` 是一款方便进行推送测试的 cli 工具。

* 支持多APP配置
* 支持多通道推送

目前已支持:

 * [x] honor
 * [x] huawei
 * [ ] xiaomi
 * [ ] APNs
 * [ ] oppo
 * [ ] vivo
 * [ ] meizu
 * [ ] FCM


> Rust练习项目.....

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

为了最大定制化请求参数，参数采用`json`文件。使用 `channel` 命令生成参数模版。
模版生成路径为命令执行时的路径。

```shell
dai channel -i honor      # 生成honor通道请求参数
```


### push 命令

指定通道下发推送消息。


```shell
dai push honor     # 如果不指定文件路径(-f) 默认读取当前路径下的honor.json 文件
```