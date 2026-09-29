# netflix-block-verify

检测当前网络环境能否观看 Netflix，并识别出口地区。

- **完整解锁**：自制 + 非自制均可观看
- **仅自制**：只能看 Netflix 自制内容
- **无法观看**：当前网络不可用

## 用法

```bash
cargo run
```

程序请求两个 Netflix 影片页，跟随重定向后按最终状态码与 URL 中的地区码判定结果。建议挂代理后运行。

## 构建

```bash
cargo build --release
```

![image](https://github.com/panxianhaoo/netflix-block-verify/assets/30815101/719e94f0-a6b0-4562-bb37-5d507f4edc5c)
