// import 'reflect-metadata'
// import { Bench } from 'tinybench'

// import { insertManyValue, insertModelUndoCache, getModelUndoCacheByFirstOne } from './nodejs/undo.service.ts'
// import { cacheDataSourceSqlLite } from './nodejs/index.ts'

// async function run() {
//   // 初始化数据库
//   if (!cacheDataSourceSqlLite.isInitialized) {
//     await cacheDataSourceSqlLite.initialize()
//   }
//   // 预热数据库
//   await insertManyValue()
// }

// await run()

// // 并发配置
// const CONCURRENCY = 1 // 并发任务数
// const ITERATIONS = 100 // 每个任务执行次数

// const bench = new Bench({ time: 1000 }) // 每个任务大约运行时间（ms）

// // 封装并发函数
// async function runConcurrent(task: () => Promise<void>, concurrency = 1) {
//   const promises = Array.from({ length: concurrency }, () =>
//     (async () => {
//       for (let i = 0; i < ITERATIONS; i++) {
//         await task()
//       }
//     })(),
//   )
//   await Promise.all(promises)
// }

// // 添加 benchmark 任务
// bench
//   .add('node Insert 100 rows (concurrent)', async () => {
//     await runConcurrent(() => insertModelUndoCache('test', 'test', 'test', 'test', 'test'), CONCURRENCY)
//   })
//   .add('node Query all rows (concurrent)', async () => {
//     await getModelUndoCacheByFirstOne('test')
//   })

// // 执行 benchmark
// await bench.run()

// // 输出结果
// console.table(bench.table())
