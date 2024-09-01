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
