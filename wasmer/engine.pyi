from typing import Type
from wasmer import CompilerBase

class Engine: ...

class JIT(Engine):
    def __init__(self, compiler: Type[CompilerBase]) -> None: ...
