# 项目名称
### 系统框架
* koa
* typeorm
* typescript
* nodeman
### 系统描述=
[主要完成接口开发中的钱包对象和结算对象](https://github.com/Curdata-project/yunchaoplus-docs/tree/main/3.%E7%BB%9F%E4%B8%80%E6%94%AF%E4%BB%98%E8%B4%A6%E6%88%B7)

## 项目目录结构
* dist 打包目录
* logs 运行日志目录
* sql sql脚本对象
* src 源码目录
  * app.ts 启动文件
  * config.ts 配置文件
  * controller 控制层恩建
    * schema 参数校验schema
  * entity 数据库实体文件
  * util 工具类文件
* package.json 依赖包文件
## 数据定义
### 数据表定义 src/entity/
* wallet
* settle

## API 定义
1. [创建钱包对象](https://github.com/Curdata-project/yunchaoplus-docs/blob/main/3.%E7%BB%9F%E4%B8%80%E6%94%AF%E4%BB%98%E8%B4%A6%E6%88%B7/2.2%E5%88%9B%E5%BB%BA%E9%92%B1%E5%8C%85%E5%AF%B9%E8%B1%A1.md)
2. [锁定解锁钱包对象](https://github.com/Curdata-project/yunchaoplus-docs/blob/main/3.%E7%BB%9F%E4%B8%80%E6%94%AF%E4%BB%98%E8%B4%A6%E6%88%B7/2.3%E9%94%81%E5%AE%9A%E8%A7%A3%E9%94%81%E9%92%B1%E5%8C%85%E5%AF%B9%E8%B1%A1.md)
3. [通过审核钱包对象](https://github.com/Curdata-project/yunchaoplus-docs/blob/main/3.%E7%BB%9F%E4%B8%80%E6%94%AF%E4%BB%98%E8%B4%A6%E6%88%B7/2.4通过审核钱包对象.md)
4. [查询钱包对象](https://github.com/Curdata-project/yunchaoplus-docs/blob/main/3.%E7%BB%9F%E4%B8%80%E6%94%AF%E4%BB%98%E8%B4%A6%E6%88%B7/2.5查询钱包对象.md)
5. [查询钱包对象列表](https://github.com/Curdata-project/yunchaoplus-docs/blob/main/3.%E7%BB%9F%E4%B8%80%E6%94%AF%E4%BB%98%E8%B4%A6%E6%88%B7/2.6查询钱包对象列表.md)
6. [创建结算对象](https://github.com/Curdata-project/yunchaoplus-docs/blob/main/3.%E7%BB%9F%E4%B8%80%E6%94%AF%E4%BB%98%E8%B4%A6%E6%88%B7/3.2创建结算对象.md)
7. [查询结算对象](https://github.com/Curdata-project/yunchaoplus-docs/blob/main/3.%E7%BB%9F%E4%B8%80%E6%94%AF%E4%BB%98%E8%B4%A6%E6%88%B7/3.3查询结算对象.md)
8. [查询结算对象列表](https://github.com/Curdata-project/yunchaoplus-docs/blob/main/3.%E7%BB%9F%E4%B8%80%E6%94%AF%E4%BB%98%E8%B4%A6%E6%88%B7/3.4查询结算对象列表.md)
9. [删除结算对象](https://github.com/Curdata-project/yunchaoplus-docs/blob/main/3.%E7%BB%9F%E4%B8%80%E6%94%AF%E4%BB%98%E8%B4%A6%E6%88%B7/3.5删除结算对象.md)

## 开发环境搭建
1. yarn install
2. yarn dev
## 生产环境设置
1. docker打包
```shell
sh build.sh
```
2. docker容器运行
```shell
sh start.sh
```
## CI/CD

### Branch

要构建的仓库分支

### ConfigMap
#### docker配置路径 /app/config.json
```json
{
  "serverPort":8080,
  "db":{
    "type":"postgres",
    "username": "postgres",
    "host": "docker.for.mac.host.internal",
    "port": 5432,
    "database": "postgres",
    "password": "123456",
    "schema":"wallet",
    "entities":["src/entity/*.ts","entity/*.js"]
  }
}

```

### PersistentVolume

### Ports

默认8080，可通过src/config.ts中的serverPort进行修改

### Getaway

### Linked Service
依赖posgresql数据库，用来存在钱包和结算对象
### InitContainer
初始化docker容器对象，已在Dockerfile中定义
