from typing import Any, Callable
from wasmer.engine import Engine

wat2wasm: Callable[[str], bytes]

class CompilerBase: ...

class Store:
    def __init__(self, engine: Engine) -> None: ...

class Module:
    def __init__(self, store: Store, src: bytes) -> None: ...

class Instance:
    def __init__(self, module: Module) -> None: ...

    exports: Any = ...
