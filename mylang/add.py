from typing_extensions import Protocol

class Exports(Protocol):
    def add(self, lhs: int, rhs: int) -> int: ...
