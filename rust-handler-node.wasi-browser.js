import {
  createOnMessage as __wasmCreateOnMessageForFsProxy,
  getDefaultContext as __emnapiGetDefaultContext,
  instantiateNapiModuleSync as __emnapiInstantiateNapiModuleSync,
  WASI as __WASI,
} from '@napi-rs/wasm-runtime'



const __wasi = new __WASI({
  version: 'preview1',
})

const __wasmUrl = new URL('./rust-handler-node.wasm32-wasi.wasm', import.meta.url).href
const __emnapiContext = __emnapiGetDefaultContext()


const __sharedMemory = new WebAssembly.Memory({
  initial: 4000,
  maximum: 65536,
  shared: true,
})

const __wasmFile = await fetch(__wasmUrl).then((res) => res.arrayBuffer())

const {
  instance: __napiInstance,
  module: __wasiModule,
  napiModule: __napiModule,
} = __emnapiInstantiateNapiModuleSync(__wasmFile, {
  context: __emnapiContext,
  asyncWorkPoolSize: 4,
  wasi: __wasi,
  onCreateWorker() {
    const worker = new Worker(new URL('./wasi-worker-browser.mjs', import.meta.url), {
      type: 'module',
    })

    return worker
  },
  overwriteImports(importObject) {
    importObject.env = {
      ...importObject.env,
      ...importObject.napi,
      ...importObject.emnapi,
      memory: __sharedMemory,
    }
    return importObject
  },
  beforeInit({ instance }) {
    for (const name of Object.keys(instance.exports)) {
      if (name.startsWith('__napi_register__')) {
        instance.exports[name]()
      }
    }
  },
})
export default __napiModule.exports
export const BinaryFuncCode = __napiModule.exports.BinaryFuncCode
export const autoShutter = __napiModule.exports.autoShutter
export const autoShutterCache = __napiModule.exports.autoShutterCache
export const confConfig = __napiModule.exports.confConfig
export const confFunctionPic = __napiModule.exports.confFunctionPic
export const confGraphicCustom = __napiModule.exports.confGraphicCustom
export const confModel = __napiModule.exports.confModel
export const confPfModelParams = __napiModule.exports.confPfModelParams
export const confSystemVariable = __napiModule.exports.confSystemVariable
export const confUnit = __napiModule.exports.confUnit
export const heryDetail = __napiModule.exports.heryDetail
export const initDB = __napiModule.exports.initDB
export const modelHandle = __napiModule.exports.modelHandle
export const physicalBinarySync = __napiModule.exports.physicalBinarySync
export const physicalCalc = __napiModule.exports.physicalCalc
export const physicalComponent = __napiModule.exports.physicalComponent
export const pointInfor = __napiModule.exports.pointInfor
export const shutterHandle = __napiModule.exports.shutterHandle
export const undoRedoHandle = __napiModule.exports.undoRedoHandle
export const variableCurveHandle = __napiModule.exports.variableCurveHandle
