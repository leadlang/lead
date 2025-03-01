$hello: *mod test/hello

$a: malloc string "12"

print $a

$a: str::to_int ->$a

$a: unwrap ->$a

print $a

$hello::init ->$a

print $a

$os: os::name

print $os