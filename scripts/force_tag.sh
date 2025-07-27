#!/bin/bash
set -e

# 允许指定版本，默认 v0.0.1
TAG=${1:-v0.0.1}

# 校验 tag 格式（必须为 vX.Y.Z）
if [[ ! $TAG =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "错误：tag 格式不正确，必须为 vX.Y.Z，如 v1.2.3"
  exit 1
fi

# 删除本地 tag
git tag -d $TAG || true

# 删除远程 tag
git push origin :refs/tags/$TAG || true

# 新建 tag
git tag $TAG

# 强制推送 tag
git push origin $TAG --force

echo "Tag $TAG 已强制重建并推送到远程仓库。"
