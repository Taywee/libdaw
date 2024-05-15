from collections import abc
from typing import Self
from libdaw.metronome import Beat, Metronome
from libdaw.nodes.instrument import Tone
from libdaw.pitch import A440, Pitch, PitchStandard

_Item = Chord | Note | Rest | Overlapped | Sequence

class _ChordIterator:
    def __iter__(self) -> Self: ...
    def __next__(self) -> Pitch: ...

class _OverlappedIterator:
    def __iter__(self) -> Self: ...
    def __next__(self) -> _Item: ...

class _SequenceIterator:
    def __iter__(self) -> Self: ...
    def __next__(self) -> _Item: ...

class Chord:
    def __init__(self, pitches: abc.Sequence[Pitch], length: Beat | None = None, duration: Beat | None = None) -> None: ...
    @staticmethod
    def parse(source: str) -> Chord: ...

    def resolve(
        self,
        *,
        offset: Beat = Beat(0),
        metronome: Metronome = Metronome(),
        pitch_standard: PitchStandard = A440(),
        previous_length: Beat = Beat(1),
    ) -> abc.Sequence[Tone]: ...

    def get_length(self) -> Beat | None: ...
    def get_duration(self) -> Beat | None: ...
    def set_length(self, value: Beat | None) -> None: ...
    def set_duration(self, value: Beat | None) -> None: ...
    def length(self, previous_length: Beat) -> Beat: ...
    def duration(self, previous_length: Beat) -> Beat: ...
    def __len__(self) -> int: ...
    def __getitem__(self, index: int) -> Pitch: ...
    def __setitem__(self, index: int, value: Pitch) -> None: ...
    def __delitem__(self, index: int) -> None: ...
    def __iter__(self) -> _ChordIterator: ...
    def insert(self, index: int, value: Pitch) -> None: ...
    def pop(self, index: int) -> Pitch: ...
    def append(self, value: Pitch) -> None: ...

class Note:
    def __init__(self, pitch: Pitch, length: Beat | None = None, duration: Beat | None = None) -> None: ...

    @staticmethod
    def parse(source: str) -> Note: ...

    @property
    def pitch(self) -> Pitch: ...

    @pitch.setter
    def pitch(self, value: Pitch) -> None: ...

    def resolve(
        self,
        *,
        offset: Beat = Beat(0),
        metronome: Metronome = Metronome(),
        pitch_standard: PitchStandard = A440(),
        previous_length: Beat = Beat(1),
    ) -> Tone: ...

    def get_length(self) -> Beat | None: ...
    def get_duration(self) -> Beat | None: ...
    def set_length(self, value: Beat | None) -> None: ...
    def set_duration(self, value: Beat | None) -> None: ...
    def length(self, previous_length: Beat) -> Beat: ...
    def duration(self, previous_length: Beat) -> Beat: ...

class Overlapped:
    def __init__(self, sections: abc.Sequence[_Item] | None = None) -> None: ...
    @staticmethod
    def parse(source: str) -> Overlapped: ...

    def resolve(
        self,
        *,
        offset: Beat = Beat(0),
        metronome: Metronome = Metronome(),
        pitch_standard: PitchStandard = A440(),
        previous_length: Beat = Beat(1),
    ) -> abc.Sequence[Tone]: ...

    def length(self, previous_length: Beat) -> Beat: ...
    def duration(self, previous_length: Beat) -> Beat: ...
    def __len__(self) -> int: ...
    def __getitem__(self, index: int) -> _Item: ...
    def __setitem__(self, index: int, value: _Item) -> None: ...
    def __delitem__(self, index: int) -> None: ...
    def __iter__(self) -> _OverlappedIterator: ...
    def insert(self, index: int, value: _Item) -> None: ...
    def pop(self, index: int) -> _Item: ...
    def append(self, value: _Item) -> None: ...

class Rest:
    def __init__(self, length: Beat | None = None) -> None: ...

    @staticmethod
    def parse(source: str) -> Rest: ...

    def get_length(self) -> Beat | None: ...
    def set_length(self, value: Beat | None) -> None: ...
    def length(self, previous_length: Beat) -> Beat: ...
    def duration(self) -> Beat: ...

class Sequence:
    def __init__(self, items: abc.Sequence[_Item] | None = None): ...

    @staticmethod
    def parse(source: str) -> Sequence: ...

    def resolve(
        self,
        *,
        offset: Beat = Beat(0),
        metronome: Metronome = Metronome(),
        pitch_standard: PitchStandard = A440(),
        previous_length: Beat = Beat(1),
    ) -> abc.Sequence[Tone]: ...
    def length(self, previous_length: Beat) -> Beat: ...
    def duration(self, previous_length: Beat) -> Beat: ...

    def __len__(self) -> int: ...
    def __getitem__(self, index: int) -> _Item: ...
    def __setitem__(self, index: int, value: _Item) -> None: ...
    def __delitem__(self, index: int) -> None: ...
    def __iter__(self) -> _SequenceIterator: ...
    def insert(self, index: int, value: _Item) -> None: ...
    def pop(self, index: int) -> _Item: ...
    def append(self, value: _Item) -> None: ...

def parse(source: str) -> _Item: ...
