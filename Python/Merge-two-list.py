from typing import Optional
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
class Solution:
    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        head = ListNode(0)
        node = head
        while list1.val & list2.val:

            if list1.val<list2.val:
                node.next=list1
                list1=list2.next
            else:
                node.next=list2
                list2=list2.next
            node=node.next
        node.next=list1 if list1 else list2
        return head.next

