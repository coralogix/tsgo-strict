import type { Type3550 } from '../batch-17/file-3550.js';
export interface Type3551 {
  id: 3551;
  name: 'File3551';
  next: Type3550;
}

export function make3551(): Type3551 {
  return { id: 3551, name: 'File3551', next: null as unknown as Type3550 };
}
