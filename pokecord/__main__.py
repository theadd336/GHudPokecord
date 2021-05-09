from pokecord import pokecord_backend

print("Hello World!")

test = pokecord_backend.sum_as_string(5, 7)
assert test == "12"
print(test)
