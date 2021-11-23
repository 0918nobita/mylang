from wasmer import Module, Store, engine, wat2wasm
from wasmer_compiler_cranelift import Compiler

from mylang.add_wasm import Exports
from mylang.wasmer_wrapper import Instance

def test_add_wasm() -> None:
    store = Store(engine.JIT(Compiler))

    with open('add.wat', 'r') as file:
        wat = file.read()

    wasm = wat2wasm(wat)

    module = Module(store, wasm)

    instance = Instance[Exports](module)

    result = instance.exports.add(3, 4)
    assert result == 7
