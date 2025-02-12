# IkegamiDrivingSchoolAvailabilityChecker

池上自動車教習所における技能教習の予約空き状況を知らせます。

discordに通知するため、[discordのbotを作成](https://discord.com/developers/applications)しておいてください。その上で作成したdiscord botのtokenと、送信するチャットのIDを`config.toml`に入力してください。

また、`config.toml`には[技能実習の予約ページ](https://www.e-license.jp/el31/mSg1DWxRvAI-brGQYS-1OA%3D%3D)のIDとパスワードを入力してください。

この辺は環境変数にしたほうがいいと分かってはいますが、どうやるのかが分かりませんでした。未来の自分に期待しています。

---

Dockerfileの`FROM rust:my`のイメージは以下の通り。

```
FROM ubuntu:20.04
ENV DEBIAN_FRONTEND=noninteractive
RUN /bin/sh -c apt update
RUN /bin/sh -c apt install tzdata
RUN /bin/sh -c ln -sf /usr/share/zoneinfo/Asia/Tokyo /etc/localtime
RUN /bin/sh -c apt install -y curl build-essential
RUN /bin/sh -c curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH
```

ガバガバ管理ですまない。
