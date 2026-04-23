import type { Type2432 } from '../batch-12/file-2432.js';
export interface Type2433 {
  id: 2433;
  name: 'File2433';
  next: Type2432;
}

export function make2433(): Type2433 {
  return { id: 2433, name: 'File2433', next: null as unknown as Type2432 };
}
