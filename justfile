watch +args='run --release traits/**/*.png':
  cargo watch --clear --exec '{{args}}'

open:
  open index/index.html

tree:
  tree traits
