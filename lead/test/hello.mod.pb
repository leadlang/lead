__declare_global hello
  _fn init
    $1: @s"Hello World"
    print $1
  _end

  _drop
    drop
  _end

__end