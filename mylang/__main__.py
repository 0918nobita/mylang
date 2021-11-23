from wasmer import Instance, Module, Store, engine, wat2wasm
from wasmer_compiler_cranelift import Compiler

store = Store(engine.JIT(Compiler))

with open('add.wat', 'r') as file:
    wat = file.read()

wasm = wat2wasm(wat)

module = Module(store, wasm)

instance = Instance(module)

result: int = instance.exports.add(3, 4)
print(result)
