__declare_global hello
  _fn init ->$ap
    $1: malloc string "Hello World"
    print $1 $ap
  _end

  _drop
    drop
  _end

__end