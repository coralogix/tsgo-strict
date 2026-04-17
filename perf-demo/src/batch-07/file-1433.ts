import type { Type1432 } from '../batch-07/file-1432.js';
export interface Type1433 {
  id: 1433;
  name: 'File1433';
  next: Type1432;
}

export function make1433(): Type1433 {
  return { id: 1433, name: 'File1433', next: null as unknown as Type1432 };
}
