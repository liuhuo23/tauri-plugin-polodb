
# Tauri Plugin polodb

一个用于 Tauri 应用的 polodb 嵌入式数据库插件，支持 JS 侧通过 invoke 进行增删查改操作。

## 安装


在 tauri 项目的 `Cargo.toml` 添加依赖：

```toml
[dependencies]
tauri-plugin-polodb = { git = "https://github.com/liuhuo23/tauri-plugin-polodb.git" }
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
```js
await invoke('plugin:polodb|inser_one', {
  collection: '集合名',
  value: { ... } // 任意可序列化对象
});
返回: true/false
```

### insert_many
批量插入多条数据
```js
await invoke('plugin:polodb|insert_many', {
  collection: '集合名',
  payloads: [ {...}, {...} ]
});
返回: true/false
```

### find
查询数据，支持条件、分页、排序
```js
await invoke('plugin:polodb|find', {
  collection: '集合名',
  filter: { ... }, // 可选，查询条件
  limit: 10,      // 可选，最大条数
  skip: 0,        // 可选，跳过条数
  sort: { ... }   // 可选，排序条件
});
返回: [ {...}, {...} ] // 文档数组
```

### find_one
查询单条数据
```js
await invoke('plugin:polodb|find_one', {
  collection: '集合名',
  filter: { ... }
});
返回: { ... } 或 null
```

### delete_many
批量删除数据
```js
await invoke('plugin:polodb|delete_many', {
  collection: '集合名',
  filter: { ... }
});
返回: 删除数量（number）
```

### count_documents
统计集合文档数量
```js
await invoke('plugin:polodb|count_documents', {
  collection: '集合名'
});
返回: 数量（number）
```

### drop_collection
删除集合
```js
await invoke('plugin:polodb|drop_collection', {
  collection: '集合名'
});
返回: true/false
```

### list_collection_names
获取所有集合名
```js
await invoke('plugin:polodb|list_collection_names');
返回: [ '集合1', '集合2', ... ]
```

### create_collection
新建集合
```js
await invoke('plugin:polodb|create_collection', {
  name: '集合名'
});
返回: true/false
```

## 贡献

欢迎 issue 和 PR！
