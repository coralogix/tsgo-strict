import type { Type3432 } from '../batch-17/file-3432.js';
export interface Type3433 {
  id: 3433;
  name: 'File3433';
  next: Type3432;
}

export function make3433(): Type3433 {
  return { id: 3433, name: 'File3433', next: null as unknown as Type3432 };
}
