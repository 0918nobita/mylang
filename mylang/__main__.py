from wasmer import Instance, Module, Store, engine
from wasmer_compiler_cranelift import Compiler

store = Store(engine.JIT(Compiler))

module = Module(store, open('add.wasm', 'rb').read())

instance = Instance(module)

result: int = instance.exports.add(3, 4)
print(result)
