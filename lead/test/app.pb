$a: malloc string "This is a string"

$c: fmt "Data: ${a}"

print $c

$hello: *mod test/hello

$a: malloc string "12"
$b: malloc bool false

*else$b print $b

print $a
print $hello

$a: str::to_int ->$a

$a: unwrap ->$a

print $a

$hello::init ->$a

print $a

$os: os::name

print $os