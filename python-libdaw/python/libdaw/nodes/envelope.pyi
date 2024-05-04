from libdaw.time import Time


class Point:
    def __init__(
        self,
        whence: float,
        volume: float,
        offset: Time | None = None,
    ) -> None: ...
