def binary_search_private(items, target, left, right):
    if right < left:
        return None

    mid = left + (right - left) // 2

    if items[mid] == target:
        return mid
    elif items[mid] > target:
        return binary_search_private(items, target, left, mid - 1)
    else:
        return binary_search_private(items, target, mid + 1, right)

def binary_search(items, target):
    return binary_search_private(items, target, 0, len(items) - 1)

items = [1, 9, 45, 63, 31, 70, 20, 100, 2]
items = sorted(items)
result = binary_search(items, 31)

if result is None:
    print("Number not found")
else:
    print(f"{result}")