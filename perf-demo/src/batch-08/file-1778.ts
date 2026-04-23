import type { Type1777 } from '../batch-08/file-1777.js';
export interface Type1778 {
  id: 1778;
  name: 'File1778';
  next: Type1777;
}

export function make1778(): Type1778 {
  return { id: 1778, name: 'File1778', next: null as unknown as Type1777 };
}
