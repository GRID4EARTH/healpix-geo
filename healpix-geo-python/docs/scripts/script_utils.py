import hashlib


def hashsum(bytes_):
    return hashlib.new("sha256", bytes_).digest()


def differing_contents(old, new):
    old_sum = hashsum(old)
    new_sum = hashsum(new)

    return old_sum != new_sum
