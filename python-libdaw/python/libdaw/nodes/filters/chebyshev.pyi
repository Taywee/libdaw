from libdaw import Node

class LowPass(Node):
    def __new__(cls: type, n: int, epsilon: float, frequency: float, sample_rate: int = 48000): ...

class HighPass(Node):
    def __new__(cls: type, n: int, epsilon: float, frequency: float, sample_rate: int = 48000): ...

class BandPass(Node):
    def __new__(cls: type, n: int, epsilon: float, low_frequency: float, high_frequency: float, sample_rate: int = 48000): ...

class BandStop(Node):
    def __new__(cls: type, n: int, epsilon: float, low_frequency: float, high_frequency: float, sample_rate: int = 48000): ...

