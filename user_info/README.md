## 用户信息

#### 数据对象 user_info

| Field       | Type         | Comment                                |
| ----------- | ------------ | -------------------------------------- |
| appid       | varchar(255) | 用户应用id                              |
| web_url     | text         | webhooks推送地址                        |
| create_time | timestamp    | 数据创建时间                            |
| update_time | timestamp    |                                        |

#### 信息存储接口

POST $endpoint/user/info

请求样例：

```json
{
    "appid": "", // 用户应用id
    "web_url":"" //webhooks推送地址
}
```

返回样例：

```json
{
    "code": 0,
    "message": "message",
    "data": {
        "appid": "", // 用户应用id
        "web_url":"" //webhooks推送地址
    }
}
```

#### 获取信息

GET $endpoint/user/info/{appid}

返回样例：

```json
{
    "code": 0,
    "message": "message",
    "data": {
        "appid": "", // 对应 app 对象的 id
        "web_url":"", //webhooks推送地址
        "created": i64 //unix时间戳
    }
}
```
#### 获取信息列表

GET $endpoint/user/info

返回样例：

```json
{
    "code": 0,
    "message": "message",
    "data": {
        "total": i64,
        [
            {
            "appid": "", // 用户应用id
            "secret_key": "" ,//用户私钥
            "web_url":"", //webhooks推送地址
            "created": i64 //unix时间戳
            }
        ]
    }
}