# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def postorderTraversal(self, root):
        """
        :type root: TreeNode
        :rtype: List[int]
        """
        if root is None:
            return []

        stack1 = []
        stack2 = []

        stack1.append(root)
        while stack1:
            node = stack1.pop()
            stack2.append(node.val)
            if node.left:
                stack1.append(node.left)
            if node.right:
                stack1.append(node.right)
        
        return stack2[::-1]
