import type { Type1550 } from '../batch-07/file-1550.js';
export interface Type1551 {
  id: 1551;
  name: 'File1551';
  next: Type1550;
}

export function make1551(): Type1551 {
  return { id: 1551, name: 'File1551', next: null as unknown as Type1550 };
}
