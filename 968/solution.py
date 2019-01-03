# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    WATCHER, MONITORED, FREE = range(3)
    watchers_count = 0

    def minCameraCover(self, root):
        """
        :type root: TreeNode
        :rtype: int
        """
        if self.dfs(root) == self.FREE:
            self.watchers_count += 1
        return self.watchers_count

    def dfs(self, root):
        if root is None:
            return self.MONITORED
        status_l, status_r = self.dfs(root.left), self.dfs(root.right)
        if status_l == self.FREE or status_r == self.FREE:
            self.watchers_count += 1
            return self.WATCHER
        elif status_l == self.WATCHER or status_r == self.WATCHER:
            return self.MONITORED
        else:
            return self.FREE
