from python.libdaw.time import Duration, Timestamp


class Tone:
    def __new__(cls: type, start: Timestamp, length: Duration, frequency: float): ...
