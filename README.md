# rust template

## use template

```shell
# install cargo-generate
$ cargo install cargo-generate
# use template
$ cargo generate WITstudio86/rust-template
```

## install pre-commit

- install pre-commit

```shell   “‘壳
brew install pre-commit
# or
pipx install pre-commit
```

- install pre-commit in the project

```shell
# install pre-commit
$ pre-commit install
```

## cargo denny

we need to install cargo deny我们需要安装防货器

```shell
# run cargo denny
cargo install --locked cargo-deny
```   ｀｀｀

## typos

```shell   “‘壳   “‘壳
cargo install --locked cargo-typos
```   ｀｀｀

## git cliff

```shell   “‘壳   “‘壳
cargo install --locked git-cliff
```   ｀｀｀

## nextest

```shell   “‘壳   “‘壳
cargo install --locked nextest
```   ｀｀｀

## The role of files

- [./.pre-commit-config.yaml](.pre-commit-config.yaml)
  - 存放 pre-commit 的配置代码, 说明检查的步骤
- [./.gitignore](.gitignore)
  - 存放 git 忽略的文件
- [./deny.toml](deny.toml)
  - 存放 deny 的依赖检查配置
- [./cliff.toml](cliff.toml)
  - 生成 changelog 的配置
- [.github/workflows/build.yml](.github/workflows/build.yml)
  - github 的 CI 配置 , action 需要执行的指令
