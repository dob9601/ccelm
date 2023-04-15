with open("single-b.txt") as file:
    single = file.read().splitlines()

with open("multi-b.txt") as file:
    multi = file.read().splitlines()


print("\n".join([h for h in single if h not in multi]))
print("-----")
print("\n".join([h for h in multi if h not in single]))
