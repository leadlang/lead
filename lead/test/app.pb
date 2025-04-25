$a: malloc string "This is a string"

print $a

$c: fmt "Data: ${a}"

print $c

$hello: *mod hello

$a: malloc string "12"
$b: malloc bool false

print $b

# *else$b print $b

print $hello
$hello::init ->$a