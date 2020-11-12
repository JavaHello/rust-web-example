# Rust Web 后端例子

实现了基本的登录注册，修改密码,验证码功能。请求数据都是`json`格式。

## 技术栈

|框架|功能|
|:-:|:-:|
|actix-web|web服务|
|rbatis|SQL-ORM|
|redis|redis客户端|
|jsonwebtoken|JWT|
|actix-web-httpauth|登录验证|
|validator|请求参数验证|

## 开发

执行`script/ddl/*.sql`文件建表

### 执行单元测试
```shell
cargo test
```
全部通过环境正确

### 启动
执行
```shell
cargo run
```
可访问对应接口

获取验证码 http://localhost:8000/verify_code
```json
{
    "vc_type": {
        "SignIn": "18969542172"
    },
    "len": 4
}
```

注册 http://localhost:8000/register
```json
{
    "password":"12345678",
    "username":"18969542172",
    "verify_code":"xxxx"
}
```

登录 http://localhost:8000/login
```json
{
    "username":"18969542172",
    "password":"123456789",
    "verify_code": "xxxx"
}
```

修改密码 http://localhost:8000/v1/change_password
```json
{
    "username":"18969542172",
    "old_password":"12345678",
    "new_password":"123456789",
    "verify_code":"xxxx"
}
```