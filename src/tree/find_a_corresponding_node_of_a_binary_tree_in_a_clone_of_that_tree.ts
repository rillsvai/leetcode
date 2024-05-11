class TreeNode {
  val: number;
  left: TreeNode | null;
  right: TreeNode | null;
  constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = val === undefined ? 0 : val;
    this.left = left === undefined ? null : left;
    this.right = right === undefined ? null : right;
  }
}

function getTargetCopy(
  original: TreeNode | null,
  cloned: TreeNode | null,
  target: TreeNode | null
): TreeNode | null {
  const stack_original = [original];
  const stack_cloned = [cloned];
  while (stack_original.length != 0) {
    const original_node = stack_original.pop();
    const cloned_node = stack_cloned.pop();

    if (original_node === target) {
      return cloned_node || null;
    }

    if (original_node?.left) {
      stack_original.push(original_node.left);
      stack_cloned.push(cloned_node!.left);
    }

    if (original_node?.right) {
      stack_original.push(original_node.right);
      stack_cloned.push(cloned_node!.right);
    }
  }

  return null;
}
