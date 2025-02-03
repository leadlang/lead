*mod test/hello

*run test/index

$a: malloc string "12"

str::to_int $a ->$a

hello init ->$a