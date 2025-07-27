
# Tauri Plugin polodb

一个用于 Tauri 应用的 polodb 嵌入式数据库插件，支持 JS 侧通过 invoke 进行增删查改操作。

## 安装


在 tauri 项目的 `Cargo.toml` 添加依赖：

```toml
[dependencies]
tauri-plugin-polodb = { git = "https://github.com/liuhuo23/tauri-plugin-polodb.git" }
```

### 前端安装

使用 npm：
```sh
npm install tauri-plugin-polodb
```

使用 pnpm：
```sh
pnpm add tauri-plugin-polodb
```

使用 yarn：
```sh
yarn add tauri-plugin-polodb
```

## Rust 端用法

插件初始化：

```rust
tauri::Builder::default()
    .plugin(tauri_plugin_polodb::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
```

## JS 端用法

以插入和查询为例：

```js
import { invoke } from '@tauri-apps/api/core';

// 插入一条数据
await invoke('plugin:polodb|inser_one', {
  collection: 'default',
  value: { name: 'Copilot', age: 1 }
});

// 查询数据
const result = await invoke('plugin:polodb|find', {
  collection: 'default',
  filter: { name: 'Copilot' },
  limit: 10,
  skip: 0,
  sort: { age: 1 }
});
console.log(result); // 返回数组
```



## 支持的 API

### inser_one
插入单条数据
**参数：**
- collection: string 集合名
- value: object 任意可序列化对象
**返回：** boolean 是否成功
```js
await invoke('plugin:polodb|inser_one', {
  collection: '集合名',
  value: { ... }
});
```

### insert_many
批量插入多条数据
**参数：**
- collection: string 集合名
- payloads: object[] 数据数组
**返回：** boolean 是否成功
```js
await invoke('plugin:polodb|insert_many', {
  collection: '集合名',
  payloads: [ {...}, {...} ]
});
```

### find
查询数据，支持条件、分页、排序
**参数：**
- collection: string 集合名
- filter: object 可选，查询条件
- limit: number 可选，最大条数
- skip: number 可选，跳过条数
- sort: object 可选，排序条件
**返回：** object[] 文档数组
```js
await invoke('plugin:polodb|find', {
  collection: '集合名',
  filter: { ... },
  limit: 10,
  skip: 0,
  sort: { ... }
});
```

### find_one
查询单条数据
**参数：**
- collection: string 集合名
- filter: object 查询条件
**返回：** object/null
```js
await invoke('plugin:polodb|find_one', {
  collection: '集合名',
  filter: { ... }
});
```

### delete_many
批量删除数据
**参数：**
- collection: string 集合名
- filter: object 删除条件
**返回：** number 删除数量
```js
await invoke('plugin:polodb|delete_many', {
  collection: '集合名',
  filter: { ... }
});
```

### count_documents
统计集合文档数量
**参数：**
- collection: string 集合名
**返回：** number 文档数量
```js
await invoke('plugin:polodb|count_documents', {
  collection: '集合名'
});
```

### drop_collection
删除集合
**参数：**
- collection: string 集合名
**返回：** boolean 是否成功
```js
await invoke('plugin:polodb|drop_collection', {
  collection: '集合名'
});
```

### list_collection_names
获取所有集合名
**参数：** 无
**返回：** string[] 集合名数组
```js
await invoke('plugin:polodb|list_collection_names');
```


### update_one
更新单条数据
**参数：**
- collection: string 集合名
- filter: object 查询条件
- update: object 更新内容
**返回：** boolean 是否成功
```js
await invoke('plugin:polodb|update_one', {
  collection: '集合名',
  filter: { ... },
  update: { ... }
});
```

### update_many
批量更新数据
**参数：**
- collection: string 集合名
- filter: object 查询条件
- update: object 更新内容
**返回：** boolean 是否成功
```js
await invoke('plugin:polodb|update_many', {
  collection: '集合名',
  filter: { ... },
  update: { ... }
});
```

### rename_collection
重命名集合
**参数：**
- old_name: string 原集合名
- new_name: string 新集合名
**返回：** boolean 是否成功
```js
await invoke('plugin:polodb|rename_collection', {
  old_name: '集合1',
  new_name: '集合2'
});
```
## 贡献

欢迎 issue 和 PR！
