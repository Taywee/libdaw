from collections.abc import Callable, Sequence
from libdaw import Node, Sample
from python.libdaw.time import Duration, Timestamp
from .envelope import Point
from .instrument import Tone

class Add(Node):
    def __new__(cls: type): ...

class Callback(Node):
    def __new__(cls: type, node: Node, sample_rate: int = 48000): ...
    def add(
        self,
        callable: Callable[[Timestamp], bool | None],
        start: Timestamp = Timestamp.MIN,
        end: Timestamp = Timestamp.MAX,
        post: bool = False,
    ): ...

class ConstantValue(Node):
    def __new__(cls: type, value: float): ...

class Custom(Node):
    '''A custom Node.

    You can either pass a processing callable into this, assign it to its
    `callable` property, or subclass this with a callable.  If you subclass
    this, you **must** call super().__init__()

    A subclass may also set the callable, but this is probably pointless.
    '''
    def __new__(cls: type, callable: Callable[[Sequence[Sample]], Sequence[Sample]] | None = None): ...
    def __init__(self, callable: Callable[[Sequence[Sample]], Sequence[Sample]] | None = None): ...

    @property
    def callable(self) -> Callable[[Sequence[Sample]], Sequence[Sample]]:
        '''Returns the callable.

        Will return self if the callable is self, such as for a subclass.
        '''

    @callable.setter
    def callable(self, value: Callable[[Sequence[Sample]], Sequence[Sample]]): ...

class Delay(Node):
    def __new__(cls: type, delay: Duration, sample_rate: int = 48000): ...

class Detune(Node):
    def __new__(cls: type, detune: float = 0.0): ...

    @property
    def detune(self) -> float: ...

    @detune.setter
    def detune(self, value: float) -> None: ...

class Envelope(Node):
    def __new__(cls: type, length: Duration, envelope: Sequence[Point], sample_rate: int = 48000): ...

class Explode(Node):
    pass

class Gain(Node):
    def __new__(cls: type, gain: float): ...

    @property
    def gain(self) -> float: ...

    @gain.setter
    def gain(self, value: float) -> None: ...

class Graph(Node):
    def remove(self, node: Node) -> bool: ...
    def connect(self, source: Node, destination: Node, stream: int | None = None) -> None: ...
    def disconnect(self, source: Node, destination: Node, stream: int | None = None) -> bool: ...
    def input(self, destination: Node, stream: int | None = None) -> None: ...
    def remove_input(self, destination: Node, stream: int | None = None) -> bool: ...
    def output(self, source: Node, stream: int | None = None) -> None: ...
    def remove_output(self, source: Node, stream: int | None = None) -> bool: ...

class Implode(Node):
    pass

class Instrument(Node):
    def __new__(cls: type, factory: Callable[[Tone], Node], sample_rate: int = 48000): ...
    def add_tone(self, tone: Tone) -> None: ...

class Multiply(Node):
    def __new__(cls: type): ...

class Passthrough(Node):
    pass
