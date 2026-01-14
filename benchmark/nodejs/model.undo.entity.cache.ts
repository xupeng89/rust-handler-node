// import 'reflect-metadata'
// import { Entity, PrimaryGeneratedColumn, Column } from 'typeorm'

// @Entity({
//   name: 'model_undo_entity_cache',
//   comment: '保存操作回滚日志表',
// })
// export class ModelUndoEntityCache {
//   constructor(
//     id: number,
//     opType: string,
//     tableName: string,
//     oldData: string,
//     newData: string,
//     modelId: string,
//     status: number,
//     operatorAt: string,
//   ) {
//     this.id = id
//     this.opType = opType
//     this.tableName = tableName
//     this.oldData = oldData
//     this.newData = newData
//     this.modelId = modelId
//     this.status = status
//     this.operatorAt = operatorAt
//   }

//   @PrimaryGeneratedColumn()
//   id: number

//   @Column({ name: 'model_id', type: 'varchar', comment: '关联模型ID' })
//   modelId: string

//   @Column({ name: 'table_name', type: 'varchar', comment: '操作表名' })
//   tableName: string

//   @Column({
//     name: 'op_type',
//     type: 'varchar',
//     comment: '操作类型：insert/update/delete',
//   })
//   opType: string

//   @Column({ name: 'old_data', type: 'text', comment: '操作前数据（JSON）' })
//   oldData: string

//   @Column({ name: 'new_data', type: 'text', comment: '操作后数据（JSON）' })
//   newData: string

//   @Column({
//     name: 'status',
//     type: 'int',
//     comment: '状态：0=正常, 1=已撤销, 2=已重做',
//     default: 0,
//   })
//   status: number
//   // 只有正常和已经重做的，可以被撤销，重做只能针对已撤销

//   @Column({
//     name: 'operator_at',
//     type: 'datetime',
//     comment: '操作时间',
//     default: () => 'CURRENT_TIMESTAMP',
//   })
//   operatorAt: string
// }
