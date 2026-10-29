class Node:
    def __init__(self, data):
        self.data = data
        self.next = None

head = Node(10)
current = head
for i in range(2,6):
    node = Node(i*10)
    current.next = node
    current = current.next

while head is not None:
    print(head.data, end="-")
    head = head.next
print()
