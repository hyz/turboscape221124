from dataclasses import dataclass
from typing import Any
import sys
import time
import codecs
import encodings
import json

__all__ = ["context"]

counter = 0


@dataclass
class Context:
    name: str
    something: Any


_context = Context(
    name="test-context",
    something=None,
)


def context() -> Context:
    global counter
    counter += 1
    _context.name += f'__{__name__}__{counter}'
    return _context


gk = '_hi___'
if gk in globals():
    gv = globals().get(gk)
    print(gk, gv == '世界', '世界', gv)
    with open('___temp.txt', 'w') as f:
        print(gk, gv, file=f)
        json.dump({'foo': 123}, fp=f)
print(gk, '___locals___', locals().keys())
print(gk, '___globals___', globals().keys())
print(gk, f'__name__:{__name__} #{counter}')
print(gk, sys.getdefaultencoding(), sys.getfilesystemencoding())
print()

if __name__ == "__main__":
    _context.name = f'{__name__}'
    # print(context().name, time.time())

