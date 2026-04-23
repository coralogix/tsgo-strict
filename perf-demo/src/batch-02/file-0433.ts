import type { Type432 } from '../batch-02/file-0432.js';
export interface Type433 {
  id: 433;
  name: 'File433';
  next: Type432;
}

export function make433(): Type433 {
  return { id: 433, name: 'File433', next: null as unknown as Type432 };
}
