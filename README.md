# slackbot-lambda-rust
SlackAPIをLambda にコンテナ形式で登録した Rust から叩くサンプル 

静的リンクでデプロイするためmusl でbuild
buildにはそれ用のイメージを利用
```
docker run --rm -it -v  ここにパス:/home/rust/src ekidd/rust-musl-builder cargo build --release
```
