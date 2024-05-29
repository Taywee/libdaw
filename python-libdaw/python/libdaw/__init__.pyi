from collections.abc import Sequence

class Sample:
    def __new__(cls: type, channels: Sequence[float]):
        '''Create a new sample with the given starting values.
        '''
    def __getitem__(self, index: int) -> float: ...
    def __setitem__(self, index: int, value: float) -> None: ...
    def __len__(self) -> int: ...
    def __str__(self) -> str: ...
    def __add__(self, other: Sample) -> Sample: ...
    def __iadd__(self, other: Sample) -> Sample: ...
    def __mul__(self, other: Sample) -> Sample: ...
    def __imul__(self, other: Sample) -> Sample: ...

class Node:
    def process(self, inputs: Sequence[Sample]) -> Sequence[Sample]: ...

class FrequencyNode(Node):
    @property
    def frequency(self) -> float: ...

    @frequency.setter
    def frequency(self, value: float) -> None: ...

def play(node: Node, sample_rate: int = 48000, channels: int = 2) -> None: ...
