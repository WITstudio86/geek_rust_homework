# HOME WORK - WEEK 2

## chacha20poly1305

```shell
# 生成 key 到指定文件
$ rcli text gen-key -o fixtures/chacha20.key
# 使用 key 加密文件(从 stdin 从文件的话需要提供 -i 参数)
$ rcli text encrypt -o fixtures/chacha20.sign -k fixtures/chacha20.key
# 使用 key 解密文件`
$ rcli text decrypt -i fixtures/chacha20.sign -o fixtures/chacha20.txt -k fixtures/chacha20.key
```

## jwt

```shell
# 生成 token
$ cargo run  -- jwt sign --sub acme --aud device1 --exp 14d
# 生成 token 到指定文件
$ cargo run  -- jwt sign --sub acme --aud device1 --exp 14d > fixtures/jwt.token
# 验证 token 从 stdin
$ cargo run  -- jwt verify
# 验证 token 从文件
$ cargo run  -- jwt verify -t fixtures/jwt.token
```
