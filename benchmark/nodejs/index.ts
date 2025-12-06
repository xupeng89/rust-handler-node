import 'reflect-metadata'
import { DataSource } from 'typeorm'
import { ModelUndoEntityCache } from './model.undo.entity.cache.ts'

// 配置缓存数据库
export const cacheDataSourceSqlLite = new DataSource({
  type: 'better-sqlite3', // 设定链接的数据库类型
  database: ':memory:', // 数据库存放地址
  // database: './test.sql',
  synchronize: true, // 确保每次运行应用程序时实体都将与数据库同步
  logging: false, // 日志，默认在控制台中打印，数组列举错误类型枚举
  entities: [ModelUndoEntityCache], // 实体或模型表
  statementCacheSize: 1000, // Sqlite 查询 Statement 缓存大小, 默认100
  enableWAL: true,
  prepareDatabase: async (data) => {
    console.log('prepareCacheDatabase', data)
  },
})

export const getModelUndoCahceQueryBuilder = async () => {
  if (!cacheDataSourceSqlLite.isInitialized) {
    await cacheDataSourceSqlLite.initialize()
  }
  return cacheDataSourceSqlLite.getRepository(ModelUndoEntityCache).createQueryBuilder('model_undo_entity_cache')
}
