from collections import namedtuple
from io import StringIO
from pathlib import Path

import poyo
import humps
import zlib


ENUM_TMPL = """
use thiserror::Error;
use enumn::N;

#[derive(Debug, Display, Error, N)]
pub enum ApiError {{
{items}
}}
"""

fields = ("name", "code", "status", "message")
Item = namedtuple("Item", fields, defaults=(None,)*len(fields))


if __name__ == "__main__":
  from argparse import ArgumentParser
  parser = ArgumentParser()
  parser.add_argument("errors_yaml", type=Path)
  parser.add_argument("rust_source", type=Path)
  args = parser.parse_args()

  entries = poyo.parse_string(args.errors_yaml.read_text())
  items = []
  for name, entry in entries.items():
    items.append(Item(name, **entry))

  with StringIO() as io:
    for item in items:
      code = item.code if item.status is None else item.code * item.status
      print(f"  #[error(\"{item.message}\")]", file=io)
      print(f"  {humps.pascalize(item.name)} = {code},\n", file=io)

    with args.rust_source.open("w") as fp:
      print(ENUM_TMPL.format(items=io.getvalue()), file=fp)