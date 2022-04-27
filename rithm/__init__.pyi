from typing import (Any as _Any,
                    Optional as _Optional,
                    Tuple as _Tuple,
                    Union as _Union,
                    overload as _overload)

__version__: str = ...


class Endianness:
    BIG: 'Endianness' = ...
    LITTLE: 'Endianness' = ...

    def __repr__(self) -> str:
        ...


class Int:
    @property
    def denominator(self) -> 'Int':
        ...

    @property
    def numerator(self) -> 'Int':
        ...

    def bit_length(self) -> 'Int':
        ...

    def gcd(self, other: 'Int') -> 'Int':
        ...

    def to_bytes(self, endianness: Endianness) -> bytes:
        ...

    @classmethod
    def from_bytes(cls, value: bytes, endianness: Endianness) -> 'Int':
        ...

    @_overload
    def __new__(cls, _value: _Union['Int', int] = ...) -> 'Int':
        ...

    @_overload
    def __new__(cls, _value: str, _base: _Optional[int] = ...) -> 'Int':
        ...

    def __abs__(self) -> 'Int':
        ...

    def __add__(self, other: 'Int') -> 'Int':
        ...

    def __and__(self, other: 'Int') -> 'Int':
        ...

    def __bool__(self) -> bool:
        ...

    def __ceil__(self) -> 'Int':
        ...

    def __divmod__(self, other: 'Int') -> _Tuple['Int', 'Int']:
        ...

    @_overload
    def __eq__(self, other: 'Int') -> bool:
        ...

    @_overload
    def __eq__(self, other: _Any) -> _Any:
        ...

    def __float__(self) -> float:
        ...

    def __floor__(self) -> 'Int':
        ...

    def __floordiv__(self, other: 'Int') -> 'Int':
        ...

    def __ge__(self, other: 'Int') -> bool:
        ...

    def __getstate__(self) -> int:
        ...

    def __gt__(self, other: 'Int') -> bool:
        ...

    def __hash__(self) -> int:
        ...

    def __invert__(self) -> 'Int':
        ...

    def __int__(self) -> int:
        ...

    def __le__(self, other: 'Int') -> bool:
        ...

    def __lshift__(self, other: 'Int') -> 'Int':
        ...

    def __lt__(self, other: 'Int') -> bool:
        ...

    def __mod__(self, other: 'Int') -> 'Int':
        ...

    def __mul__(self, other: 'Int') -> 'Int':
        ...

    def __neg__(self) -> 'Int':
        ...

    def __or__(self, other: 'Int') -> 'Int':
        ...

    def __pos__(self) -> 'Int':
        ...

    def __pow__(self,
                exponent: 'Int',
                divisor: _Optional['Int'] = None
                ) -> _Union['Fraction', 'Int']:
        ...

    def __repr__(self) -> str:
        ...

    def __rshift__(self, other: 'Int') -> 'Int':
        ...

    def __setstate__(self, state: int) -> None:
        ...

    def __str__(self) -> str:
        ...

    def __sub__(self, other: 'Int') -> 'Int':
        ...

    def __truediv__(self, other: 'Int') -> 'Fraction':
        ...

    def __trunc__(self) -> 'Int':
        ...

    def __xor__(self, other: 'Int') -> 'Int':
        ...


class Fraction:
    @property
    def denominator(self) -> Int:
        ...

    @property
    def numerator(self) -> Int:
        ...

    __slots__ = '_denominator', '_numerator'

    @_overload
    def __new__(cls, _numerator: _Union[Int, float, int] = ...) -> 'Fraction':
        ...

    @_overload
    def __new__(cls,
                _numerator: _Union[Int, int],
                _denominator: _Union[Int, int]) -> 'Fraction':
        ...

    def __abs__(self) -> 'Fraction':
        ...

    def __add__(self, other: _Union['Fraction', Int]) -> 'Fraction':
        ...

    def __bool__(self) -> bool:
        ...

    @_overload
    def __eq__(self, other: 'Fraction') -> bool:
        ...

    @_overload
    def __eq__(self, other: _Any) -> _Any:
        ...

    def __float__(self) -> float:
        ...

    def __floordiv__(self, other: _Union['Fraction', Int]) -> Int:
        ...

    def __ge__(self, other: 'Fraction') -> bool:
        ...

    def __getstate__(self) -> _Tuple[Int, Int]:
        ...

    def __gt__(self, other: 'Fraction') -> bool:
        ...

    def __le__(self, other: 'Fraction') -> bool:
        ...

    def __lt__(self, other: 'Fraction') -> bool:
        ...

    def __mod__(self, other: _Union['Fraction', Int]) -> 'Fraction':
        ...

    def __mul__(self, other: _Any) -> _Any:
        ...

    def __neg__(self) -> 'Fraction':
        ...

    def __pos__(self) -> 'Fraction':
        ...

    def __pow__(self, exponent: 'Int', divisor: None = ...) -> 'Fraction':
        ...

    def __repr__(self) -> str:
        ...

    def __radd__(self, other: Int) -> 'Fraction':
        ...

    def __rfloordiv__(self, other: Int) -> Int:
        ...

    def __rmod__(self, other: Int) -> 'Fraction':
        ...

    def __rmul__(self, other: Int) -> 'Fraction':
        ...

    def __rsub__(self, other: Int) -> 'Fraction':
        ...

    def __setstate__(self, state: _Tuple[Int, Int]) -> None:
        ...

    def __str__(self) -> str:
        ...

    def __sub__(self, other: _Union['Fraction', Int]) -> 'Fraction':
        ...

    def __rtruediv__(self, other: Int) -> 'Fraction':
        ...

    def __truediv__(self, other: _Union['Fraction', Int]) -> 'Fraction':
        ...
