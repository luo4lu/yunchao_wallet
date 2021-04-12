## B端修改密码

#### 数据对象 url_statu

| Field       | Type         | Comment                                |
| ----------- | ------------ | -------------------------------------- |
| user_id     | varchar(255) | 用户id                                 |
| url         | text         | 跳转银联的地址                          |
| status      | bool         | 标记数据是否已被使用                     |
| create_time | timestamp    | 数据创建时间                            |
| update_time | timestamp    |                                        |

#### 信息存储接口

POST $endpoint/change/passworld

请求样例：

```json
{
    "id": "", // 用户id
    "url":"" //跳转银联的地址
}
```

返回样例：

```json
{
    "code": 0,
    "message": "message",
    "data": {
        "id": "", // 用户id
        "url":"", //跳转银联的地址
        "status":bool //状态
    }
}
```

#### 获取信息

GET $endpoint/change/status/{userid}

返回样例：

```json
{
    "code": 0,
    "message": "message",
    "data": {
        "id": "", // 用户id
        "url":"", //跳转银联的地址
        "status":bool //状态
    }
}
```
