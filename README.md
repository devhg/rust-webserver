# rust-webserver

几个模块

- [x] [解析HTTP报文内容](./http)
- [x] [httpserver](./httpserver)
- [x] [tcpclient，仅实现echo](./tcpclient)
- [x] [tcpserver，仅实现echo](./tcpclient)


> https://www.bilibili.com/video/BV1RP4y1G7KF


### HTTP Server
```
cargo run -p httpserver
```

GET 127.0.0.1:8000/  ==> public/index.html
GET 127.0.0.1:8000/**  ==> public/**
GET 127.0.0.1:8000/api/shopping/orders  ==> data/orders.json