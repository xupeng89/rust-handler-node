// import 'reflect-metadata'
// import { getModelUndoCahceQueryBuilder } from './index.ts'
// import dayjs from 'dayjs'
// import { ModelUndoEntityCache } from './model.undo.entity.cache.ts'
// import { In } from 'typeorm'

// // 查询数据
// export const getModelUndoCacheByFirstOne = async (modelId: string) => {
//   const queryBuilder = await getModelUndoCahceQueryBuilder()

//   return await queryBuilder.where({ modelId }).getMany()
// }

// // // 删除数据
// // export const deleteModelUndoCacheById = async (id: number, modelId: string) => {
// //     const queryBuilder = await getModelUndoCahceQueryBuilder()
// //     return await queryBuilder.delete().where({ id, modelId }).execute()
// // }

// // 插入一条数据
// export const insertModelUndoCache = async (
//   opType: string,
//   tableName: string,
//   oldData: string,
//   newData: string,
//   modelId: string,
// ) => {
//   const queryBuilder = await getModelUndoCahceQueryBuilder()

//   await queryBuilder
//     .delete()
//     .where({ status: In([1, 2]) })
//     .execute()

//   const insertData = {
//     opType: opType, // 操作类型
//     tableName: tableName, // 操作表名
//     oldData: oldData, // 需要恢复的旧数据
//     newData: newData,
//     status: 0,
//     operatorAt: dayjs().valueOf().toString(), // 操作时间
//     modelId: modelId,
//   }

//   return await queryBuilder.insert().values(insertData).execute()
// }

// export const insertManyValue = async () => {
//   const queryBuilder = await getModelUndoCahceQueryBuilder()
//   let value: Array<Omit<ModelUndoEntityCache, 'id'>> = []

//   for (let i = 1; i <= 100; i++) {
//     value.push({
//       opType: 'opType' + i, // 操作类型
//       tableName: 'tableName' + i, // 操作表名
//       oldData: 'oldData' + i, // 需要恢复的旧数据
//       newData: 'test' + i,
//       status: 0,
//       operatorAt: dayjs().valueOf().toString(), // 操作时间
//       modelId: 'modelId' + i,
//     })
//   }

//   await queryBuilder.insert().values(value).execute()
// }
