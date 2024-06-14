from libdaw import Node
from libdaw.time import Duration

class MovingAverage(Node):
    def __new__(cls: type, window: Duration, sample_rate: int = 48000): ...

