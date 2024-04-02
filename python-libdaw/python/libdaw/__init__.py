from .libdaw import *  # type: ignore

__doc__ = libdaw.__doc__ # type: ignore
if hasattr(libdaw, "__all__"): # type: ignore
    __all__ = libdaw.__all__ # type: ignore
