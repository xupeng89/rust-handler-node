// import { Bench } from 'tinybench'

// import { addUndoLog, listUndoLogs, test } from '../index.js'

// // 并发配置
// const CONCURRENCY = 1 // 并发任务数
// const ITERATIONS = 100 // 每个任务执行次数

// // 封装并发函数
// async function runConcurrent(task: () => Promise<any>, concurrency = 1) {
//   const promises = Array.from({ length: concurrency }, () =>
//     (async () => {
//       for (let i = 0; i < ITERATIONS; i++) {
//         await task()
//       }
//     })(),
//   )
//   await Promise.all(promises)
// }
// const bench = new Bench({ time: 1000 })
// console.log('Initializing database connection...')
// await test()
// console.log('Database connection ready. Running benchmarks...')

// bench
//   .add('node Insert 100 rows (concurrent)', async () => {
//     await runConcurrent(() => addUndoLog('test', 'test', 'test', 'test', 'test'), CONCURRENCY)
//   })
//   .add('node Query all rows (concurrent)', async () => {
//     await listUndoLogs('test')
//   })
// // bench
// //   .add('Insert 100 rows', async () => {
// //     await Promise.all([
// //       addUndoLog('test1', 'test', 'test', 'test', 'test'),
// //       addUndoLog('test2', 'test', 'test', 'test', 'test'),
// //       addUndoLog('test3', 'test', 'test', 'test', 'test'),
// //     ])
// //     // await addUndoLog('test', 'test', 'test', 'test', 'test')
// //   })
// //   .add('Query all row', async () => {
// //     await listUndoLogs('test')
// //   })
// console.log('Running benchmarks...')
// await bench.run()

// console.table(bench.table())
