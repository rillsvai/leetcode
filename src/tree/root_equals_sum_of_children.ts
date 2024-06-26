function checkTree(root: TreeNode | null): boolean {
  if (root.val === root?.right.val + root?.left.val) {
    return true;
  }
  return false;
}
