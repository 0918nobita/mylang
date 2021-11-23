from typing import Generic, TypeVar, cast
from wasmer import Instance as InnerInstance, Module

T = TypeVar('T')
class Instance(Generic[T]):
    def __init__(self, module: Module) -> None:
        self.exports = cast(T, InnerInstance(module).exports)
