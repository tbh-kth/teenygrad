"""
Did my own implementation sz.rs, but the dude that had tokenize_py cargo package deleted it. 

I am quite upset about it since I was quite proud of that code. 

Added a line at 29 to break the tokenization process if it detects a single line comment, therefore we do not count rust comments in our tokens.

- Alex
"""

#!/usr/bin/env python3
import os
import token
import tokenize
import itertools
from tabulate import tabulate

TOKEN_WHITELIST = [token.OP, token.NAME, token.NUMBER, token.STRING, token.COMMENT]

if __name__ == "__main__":
  headers = ["Name", "Lines", "Tokens/Line"]
  table = []
  for path, subdirs, files in os.walk("src/teenygrad"):
    for name in files:
      if not name.endswith(".rs"): continue
      filepath = os.path.join(path, name)
      with tokenize.open(filepath) as file_:
        tokens = []
        for t in tokenize.generate_tokens(file_.readline):
            if t.string == "//": break
            if t.type in TOKEN_WHITELIST: tokens.append(t)
        token_count, line_count = len(tokens), len(set([t.start[0] for t in tokens]))
        table.append([filepath, line_count, 0]) # token_count/line_count])

  print(tabulate([headers] + sorted(table, key=lambda x: -x[1]), headers="firstrow", floatfmt=".1f")+"\n")

  for dir_name, group in itertools.groupby(sorted([(x[0].rsplit("/", 1)[0], x[1]) for x in table]), key=lambda x:x[0]):
    print(f"{dir_name:30s} : {sum([x[1] for x in group]):6d}")

  total_line_count = sum([x[1] for x in table])
  print(f"\ntotal line count: {total_line_count}")
  assert total_line_count < 5000, "TEENYGRAD-RS IS SUPPOSE TO BE TEENY TINY"
