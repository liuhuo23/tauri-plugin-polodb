import { invoke } from '@tauri-apps/api/core'

export async function inser_one(collection: String, value: String): Promise<boolean> {
  return await invoke<boolean>('plugin:polodb|inser_one', {
    payload: {
      collection,
      value,
    },
  })
}


export async function insert_many(collection: String, values: String[]): Promise<boolean> {
  return await invoke<boolean>('plugin:polodb|insert_many', {
    payloads: values.map(value => ({ collection, value })),
  })
}


export async function find(collection: String, filter: String): Promise<any[]> {
  return await invoke<any[]>('plugin:polodb|find', {
    collection,
    filter,
  })
}

export async function delete_many(collection: String, filter: String): Promise<boolean> {
  return await invoke<boolean>('plugin:polodb|delete_many', {
    collection,
    filter,
  })
}

export async function count_documents(collection: String, filter: String): Promise<number> {
  return await invoke<number>('plugin:polodb|count_documents', {
    collection,
    filter,
  })
}

export async function drop_collection(collection: String): Promise<boolean> {
  return await invoke<boolean>('plugin:polodb|drop_collection', {
    collection,
  })
}

export async function list_collection_names(): Promise<string[]> {
  return await invoke<string[]>('plugin:polodb|list_collection_names')
}

export async function create_collection(collection: String): Promise<boolean> {
  return await invoke<boolean>('plugin:polodb|create_collection', {
    collection,
  })
}