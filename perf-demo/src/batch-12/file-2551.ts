import type { Type2550 } from '../batch-12/file-2550.js';
export interface Type2551 {
  id: 2551;
  name: 'File2551';
  next: Type2550;
}

export function make2551(): Type2551 {
  return { id: 2551, name: 'File2551', next: null as unknown as Type2550 };
}
