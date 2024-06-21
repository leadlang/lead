$data: array::malloc

# Creating a string to move into the array
$val: malloc string Hello World

# Types of References
# `$` Borrowed Value
# `->$` Moved Value
# `->&$` Borrowed Mutable Value
# `*` Pointer
array::push ->&$data ->$val

# Prints out data
# You cannot move out data even by using `->` syntax
print ->$data
print ->$data

# Still There
print $data

# Use Drop
drop ->$data

# Now its NONE
print $data