from libdaw.time import Time


class Point:
    def __new__(
        cls: type,
        whence: float,
        volume: float,
        offset: Time | None = None,
    ): ...
